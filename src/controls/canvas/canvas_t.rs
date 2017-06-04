/*!
    A canvas control where the user can draw stuff.
    This defines the canvas template
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

/// System class identifier
pub const CANVAS_CLASS_NAME: &'static str = "NWG_BUILTIN_D2DCANVAS";

use std::hash::Hash;
use std::any::TypeId;
use std::ptr;

use winapi::{HWND, ID2D1Factory, ID2D1HwndRenderTarget};

use ui::Ui;
use controls::{Control, ControlT};
use error::{Error, SystemError};
use events::{Event, Destroyed, Moved, Resized, KeyDown, KeyUp, Char, MouseDown, MouseUp, Paint};
use super::{Canvas, CanvasProtected};

/**
    A template that creates a canvas

    Events supported:  
    `Destroyed, KeyDown, KeyUp, Char, MouseDown, MouseUp, Moved, Resized, Paint, Any`

    Members:  
    • `parent`: The canvas parent.
    • `position` : Starting posiion of the canvas after it is created  
    • `size` : Starting size of the canvas after it is created  
    • `visible` : If the user can see the canvas or not  
    • `disabled` : If the canvas is enabled or not. A disabled canvas do not process events  
*/
pub struct CanvasT<ID: Hash+Clone> {
    pub parent: ID,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
}

impl<ID: Hash+Clone+'static> ControlT<ID> for CanvasT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<Canvas<ID>>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, KeyDown, KeyUp, Char, MouseDown, MouseUp, Moved, Resized, Paint, Event::Any]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
       unsafe{
            if let Err(e) = build_sysclass() { return Err(e); }
            match build_window(ui, &self) {
                Ok((h, (f, r))) => { Ok( Box::new( 
                    Canvas::<ID>::create(h, f, r)
                ) as Box<Control> ) },
                Err(e) => Err(e)
            }
        } // unsafe
    }
}

/*
    Private unsafe control methods
*/

use winapi::{UINT, WPARAM, LPARAM, LRESULT};
type RenderOut = (*mut ID2D1Factory, *mut ID2D1HwndRenderTarget);

#[allow(unused_variables)]
unsafe extern "system" fn canvas_sysproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::{WM_CREATE, WM_CLOSE};
    use user32::{DefWindowProcW, ShowWindow};

    let handled = match msg {
        WM_CREATE => true,
        WM_CLOSE => {
            ShowWindow(hwnd, 0);
            true
        }
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}

#[inline(always)]
unsafe fn build_sysclass() -> Result<(), Error> {
    use low::window_helper::{SysclassParams, build_sysclass};
    use winapi::{CS_HREDRAW, CS_VREDRAW};

    let params = SysclassParams { 
        class_name: CANVAS_CLASS_NAME,
        sysproc: Some(canvas_sysproc),
        background: Some(ptr::null_mut()),
        style: Some(CS_HREDRAW | CS_VREDRAW)
    };
    
    if let Err(e) = build_sysclass(params) {
        Err(Error::System(e))
    } else {
        Ok(())
    }
}

#[inline(always)]
pub unsafe fn build_render_target(hwnd: HWND, factory: &mut ID2D1Factory) -> Result<*mut ID2D1HwndRenderTarget, SystemError> {
    use winapi::{S_OK, RECT, D2D_SIZE_U, D2D1_PRESENT_OPTIONS_NONE, D2D1_PIXEL_FORMAT, D2D1_RENDER_TARGET_PROPERTIES,
      D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1_FEATURE_LEVEL_DEFAULT, D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_RENDER_TARGET_USAGE_NONE,
      DXGI_FORMAT_B8G8R8A8_UNORM, D2D1_ALPHA_MODE_PREMULTIPLIED};
    use user32::GetClientRect;
    use std::mem;

    let mut rc: RECT = mem::uninitialized();
    GetClientRect(hwnd, &mut rc);

    let size = D2D_SIZE_U { 
        width: (rc.right-rc.left) as u32,
        height: (rc.bottom-rc.top) as u32 
    };

    let pixel_format = D2D1_PIXEL_FORMAT {
        format: DXGI_FORMAT_B8G8R8A8_UNORM,
        alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED
    };

    let render_props = D2D1_RENDER_TARGET_PROPERTIES {
        _type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
        pixelFormat: pixel_format,
        dpiX: 0.0, dpiY: 0.0,
        usage: D2D1_RENDER_TARGET_USAGE_NONE,
        minLevel: D2D1_FEATURE_LEVEL_DEFAULT
    };

    let hwnd_render_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
        hwnd: hwnd,
        pixelSize: size,
        presentOptions: D2D1_PRESENT_OPTIONS_NONE
    };

    let mut render_target: *mut ID2D1HwndRenderTarget = ptr::null_mut();
    if factory.CreateHwndRenderTarget(&render_props, &hwnd_render_props, &mut render_target) != S_OK {
        factory.Release();
        let msg = "Could not create render target".to_string();
        Err(SystemError::ComError(msg))
    } else {
        Ok(render_target)
    }
}

#[inline(always)]
unsafe fn build_renderer(hwnd: HWND) -> Result<RenderOut, SystemError> {
    use winapi::{UuidOfID2D1Factory, D2D1_FACTORY_TYPE_SINGLE_THREADED, S_OK};
    use low::defs::D2D1CreateFactory;
    
    // Build the D2D Factory
    let mut factory: *mut ID2D1Factory = ptr::null_mut();
    let result = D2D1CreateFactory(
        D2D1_FACTORY_TYPE_SINGLE_THREADED,
        &UuidOfID2D1Factory,
        ptr::null(),
        &mut factory
    );
    
    if result != S_OK {
        let msg = "Could not create D2D1 factory".to_string();
        return Err(SystemError::ComError(msg));
    }

    // Build the render target
    match build_render_target(hwnd, &mut *factory) {
        Ok(render_target) => Ok( (factory, render_target) ),
        Err(e) => Err(e)
    }
}

#[inline(always)]
unsafe fn build_window<ID: Hash+Clone>(ui: &Ui<ID>, t: &CanvasT<ID>) -> Result<(HWND, RenderOut), Error> {
    use low::window_helper::{WindowParams, build_window, handle_of_window};
    use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD};
    use user32::DestroyWindow;   

    let flags: DWORD = WS_CHILD | 
    if t.visible    { WS_VISIBLE }   else { 0 } |
    if t.disabled   { WS_DISABLED }  else { 0 };

    // Get the parent handle
    let parent = match handle_of_window(ui, &t.parent, "The parent of a canvas must be a window-like control.") {
        Ok(h) => h,
        Err(e) => { return Err(e); }
    };

    let params = WindowParams {
        title:  "",
        class_name: CANVAS_CLASS_NAME,
        position: t.position.clone(),
        size: t.size.clone(),
        flags: flags,
        ex_flags: None,
        parent: parent
    };

    let handle = match build_window(params) {
        Ok(h) => h,
        Err(e) => { return Err(Error::System(e)); }
    };

    let renderer = match build_renderer(handle) {
        Ok(r) => r,
        Err(e) => {
            DestroyWindow(handle);
            { return Err(Error::System(e)); }
        }
    };

    Ok((handle, renderer))
}