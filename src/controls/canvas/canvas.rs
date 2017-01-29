/*!
    A canvas control where the user can draw stuff.
    This defines the canvas control
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

use std::hash::Hash;
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use std::collections::HashMap;

use winapi::{HWND, ID2D1Factory, ID2D1HwndRenderTarget, ID2D1SolidColorBrush, ID2D1StrokeStyle, 
  S_OK, D2D1_MATRIX_3X2_F};

use controls::{Control, ControlType, AnyHandle};
use error::{Error, SystemError};
use super::{CanvasRenderer, RendererProtected, build_render_target, CANVAS_CLASS_NAME};
use defs;


/**
    D2d resources held by a canvas
*/
#[derive(Clone)]
pub enum CanvasResources {
    SolidBrush(*mut ID2D1SolidColorBrush),
    StrokeStyle(*mut ID2D1StrokeStyle)
}

/**
    A blank control that can be painted to
*/
pub struct Canvas<ID: Clone+Hash> {
    handle: HWND,
    factory: *mut ID2D1Factory,
    render_target: *mut ID2D1HwndRenderTarget,
    must_recreate_target: bool,
    resources: HashMap<u64, CanvasResources>,
    p: PhantomData<ID>
}

impl<ID: Clone+Hash> Canvas<ID> {
    /**
        Make the canvas "paint ready" and return an object to paint to it.
        In very very **very** rare case, the renderer creation can fail.
    */
    pub fn renderer<'a>(&'a mut self) -> Result<CanvasRenderer<'a, ID>, Error> {
        CanvasRenderer::prepare(self)
    }

    /**
        Create a solid brush into the canvas and add it under the selected `name`.

        Errors:
        • `Error::System` if the canvas could not create the brush.
        • `Error::KeyExists` if the a resource with the specified name already exists
    */
    pub fn create_solid_brush(&mut self, name: &ID, brush: &defs::SolidBrush) -> Result<(), Error> {
        use winapi::{D2D1_COLOR_F, D2D1_BRUSH_PROPERTIES};

        let id = Canvas::hash_id(name);
        if self.resources.contains_key(&id) {
            return Err(Error::KeyExists);
        }

        let c = &brush.color;
        let color = D2D1_COLOR_F{r: c.0, g: c.1, b: c.2, a: c.3};
        let identity = D2D1_MATRIX_3X2_F {matrix: [[1.0, 0.0],[0.0, 1.0],[0.0, 0.0]]};
        let property = D2D1_BRUSH_PROPERTIES { opacity: 1.0, transform: identity};
        let mut brush: *mut ID2D1SolidColorBrush = ptr::null_mut();
        let result = unsafe{ self.CreateSolidColorBrush(&color, &property, &mut brush) };

        if result == S_OK {
            self.resources.insert(id, CanvasResources::SolidBrush(brush));
            Ok(())
        } else {
            Err(Error::System(SystemError::ComError("Failed to import brush".to_string())))
        }
    }

    /**
        Create a pen into the canvas and add it under the selected `name`.

        Errors:
        • `Error::System` if the canvas could not create the brush.
        • `Error::KeyExists` if the a resource with the specified name already exists
    */
    pub fn create_pen(&mut self, name: &ID, pen: &defs::Pen) -> Result<(), Error> {
        use winapi::{D2D1_STROKE_STYLE_PROPERTIES, D2D1_CAP_STYLE, D2D1_LINE_JOIN, D2D1_DASH_STYLE};

        let id = Canvas::hash_id(name);
        if self.resources.contains_key(&id) {
            return Err(Error::KeyExists);
        }

        let pen = pen.clone();
        let start_cap = D2D1_CAP_STYLE(pen.start_cap as u32);
        let end_cap = D2D1_CAP_STYLE(pen.end_cap as u32);
        let dash_cap = D2D1_CAP_STYLE(pen.dash_cap as u32);
        let line_join = D2D1_LINE_JOIN(pen.line_join as u32);
        let dash_style = D2D1_DASH_STYLE(pen.dash_style as u32);
        let stroke_style_prop = D2D1_STROKE_STYLE_PROPERTIES {
            startCap: start_cap,
            endCap: end_cap,
            dashCap: dash_cap,
            lineJoin: line_join,
            miterLimit: pen.miter_limit,
            dashStyle: dash_style,
            dashOffset: pen.dash_offset
        };

        let mut stroke_style: *mut ID2D1StrokeStyle = ptr::null_mut();
        let result = unsafe{ (&mut *self.factory).CreateStrokeStyle(&stroke_style_prop, ptr::null(), 0, &mut stroke_style) };

        if result == S_OK {
            self.resources.insert(id, CanvasResources::StrokeStyle(stroke_style));
            Ok(())
        } else {
            Err(Error::System(SystemError::ComError("Failed to import brush".to_string())))
        }

    }

    /**
        Redraw the canvas
    */
    pub fn redraw(&self) {
        use user32::RedrawWindow;
        use winapi::{RDW_ERASE, RDW_INVALIDATE};
        unsafe { 
            RedrawWindow(self.handle, ptr::null(), ptr::null_mut(), RDW_ERASE|RDW_INVALIDATE);
        }
    }
    
    /**
        Set the render target resolution.  
        If the control size do not match the render target size, the result will be upscaled or downscaled
    */
    pub fn set_render_size(&mut self, w: u32, h: u32) {
        use winapi::D2D_SIZE_U;
        let render_size = D2D_SIZE_U{width: w, height: h};
        unsafe{ self.Resize(&render_size); }
    }

    /**
        Return the render target's dots per inch (DPI).
    */
    pub fn get_dpi(&mut self) -> (f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        unsafe { self.GetDpi(&mut x, &mut y); }
        (x, y)
    }

    /**
        Sets the dots per inch (DPI) of the render target.   

        Arguments:
        • `dpix`:  A value greater than or equal to zero that specifies the horizontal DPI of the render target.  
        • `dpiy`:  A value greater than or equal to zero that specifies the vertical DPI of the render target.
    */
    pub fn set_dpi(&mut self, dpix: f32, fpiy: f32) {
        unsafe { self.SetDpi(dpix, fpiy); }
    }

    /// Hash an ID before inserting it in the canvas resources
    #[inline(always)]
    fn hash_id(id: &ID) -> u64 {
        use std::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;
        let mut s1 = DefaultHasher::new();
        id.hash(&mut s1);
        s1.finish()
    }

    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
}

impl<ID: Clone+Hash> Control for Canvas<ID> {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Canvas 
    }

    fn free(&mut self) {
        unsafe{
            use user32::{DestroyWindow, UnregisterClassW};
            use kernel32::GetModuleHandleW;
            use low::other_helper::to_utf16;

            let factory = &mut *self.factory;
            let render_target = &mut *self.render_target;

            for (_, v) in self.resources.drain() {
                match v {
                    CanvasResources::SolidBrush(r) => { (&mut *r).Release(); },
                    CanvasResources::StrokeStyle(s) => { (&mut *s).Release(); },
                }
            }

            render_target.Release();
            factory.Release();
            DestroyWindow(self.handle);

            let cls = to_utf16(CANVAS_CLASS_NAME);
            let hmod = GetModuleHandleW(ptr::null_mut());
            UnregisterClassW(cls.as_ptr(), hmod);
        };
    }

}

impl<'a, ID: Clone+Hash> Deref for Canvas<ID> {
    type Target = ID2D1HwndRenderTarget;

    #[inline(always)]
    fn deref(&self) -> &ID2D1HwndRenderTarget {
        unsafe{ & *self.render_target }
    }

}

impl<'a, ID: Clone+Hash> DerefMut for Canvas<ID> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut ID2D1HwndRenderTarget {
        unsafe{ &mut *self.render_target }
    }

}

/**
    Protected renderer method (only available in the canvas control module)
*/
pub trait CanvasProtected<ID: Clone+Hash>  {
    fn get_must_recreate_target(&mut self) -> bool;
    fn set_must_recreate_target(&mut self, recreate: bool);
    fn create(h: HWND, f: *mut ID2D1Factory, r: *mut ID2D1HwndRenderTarget) -> Canvas<ID>;
    fn rebuild(&mut self) -> Result<(), SystemError>;
    fn get_resource(&mut self, id: &ID) -> Result<CanvasResources, Error>;
}

impl<ID: Clone+Hash> CanvasProtected<ID> for Canvas<ID> {

    fn get_must_recreate_target(&mut self) -> bool {
        self.must_recreate_target
    }

    fn set_must_recreate_target(&mut self, recreate: bool) {
        self.must_recreate_target = recreate;
    }

    fn create(h: HWND, f: *mut ID2D1Factory, r: *mut ID2D1HwndRenderTarget) -> Canvas<ID> {
         Canvas::<ID>{
            handle: h,
            factory: f,
            render_target: r,
            must_recreate_target: false,
            resources: HashMap::with_capacity(10),
            p: PhantomData
        }
    }

    /// Rebuild the canvas renderer
    fn rebuild(&mut self) -> Result<(), SystemError> {
        let result = unsafe{ build_render_target(self.handle,  &mut *self.factory) };
        match result {
            Ok(render_target) => {
                self.render_target = render_target;
                self.must_recreate_target = false;
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    fn get_resource(&mut self, id: &ID) -> Result<CanvasResources, Error> {
        let id = Canvas::hash_id(id);
        if let Some(r) = self.resources.get(&id) {
             Ok( r.clone() )
        } else {
            Err(Error::KeyNotFound)
        }
    }

}