/*!
    Low level canvas utility
*/
use winapi::um::d2d1::{ID2D1Factory, ID2D1HwndRenderTarget};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT};
use winapi::shared::windef::{HWND};
use super::window::build_sysclass;
use super::window_helper::{NWG_INIT};
use crate::{SystemError};
use std::ptr;

pub const CANVAS_CLASS_ID: &'static str = "NWG_CANVAS";


/// Inner working of the D2D1 renderer
#[derive(Debug)]
pub struct CanvasRenderer {
    pub(crate) renderer: *mut ID2D1Factory,
    pub(crate) render_target: *mut ID2D1HwndRenderTarget,
}

impl Default for CanvasRenderer {

    fn default() -> CanvasRenderer {
        CanvasRenderer {
            renderer: ptr::null_mut(),
            render_target: ptr::null_mut(),
        }
    }

}


pub unsafe fn build_render_target(hwnd: HWND, factory: &mut ID2D1Factory) -> Result<*mut ID2D1HwndRenderTarget, SystemError> {
    use winapi::um::d2d1::{D2D1_RENDER_TARGET_PROPERTIES, D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_RENDER_TARGET_USAGE_NONE,
        D2D1_FEATURE_LEVEL_DEFAULT, D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1_PRESENT_OPTIONS_NONE};

    use winapi::shared::{windef::RECT, winerror::S_OK};
    use winapi::shared::dxgiformat::{DXGI_FORMAT_B8G8R8A8_UNORM};
    use winapi::um::dcommon::{D2D_SIZE_U, D2D1_PIXEL_FORMAT, D2D1_ALPHA_MODE_PREMULTIPLIED};
    use winapi::um::winuser::GetClientRect;
    use std::mem;

    let mut rc: RECT = mem::zeroed();
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
        Err(SystemError::CanvasRenderTargetCreationFailed)
    } else {
        Ok(render_target)
    }
}


pub(crate) unsafe fn build_renderer(handle: HWND) -> Result<CanvasRenderer, SystemError> {
    use winapi::um::d2d1::{D2D1CreateFactory};
    use winapi::um::d2d1::{D2D1_FACTORY_TYPE_SINGLE_THREADED};
    use winapi::shared::winerror::S_OK;
    use winapi::ctypes::c_void;
    use winapi::Interface;

    let mut factory: *mut ID2D1Factory = ptr::null_mut();
    let result = D2D1CreateFactory(
        D2D1_FACTORY_TYPE_SINGLE_THREADED,
        &ID2D1Factory::uuidof(),
        ptr::null(),
        (&mut factory as *mut *mut ID2D1Factory) as *mut *mut c_void
    );

    if result != S_OK {
        return Err(SystemError::CanvasRendererCreationFailed);
    }

    // Build the render target
    let render_target = build_render_target(handle, &mut *factory)?;

    let renderer = CanvasRenderer {
        renderer: factory,
        render_target
    };

    Ok(renderer)
}


/// Create the NWG tab classes
pub fn create_canvas_classes() -> Result<(), SystemError>  {
    use winapi::um::libloaderapi::GetModuleHandleW;

    let hmod = unsafe { GetModuleHandleW(ptr::null_mut()) };
    if hmod.is_null() { return Err(SystemError::GetModuleHandleFailed); }

    unsafe { 
        build_sysclass(hmod, CANVAS_CLASS_ID, Some(canvas_proc))?;
    }

    Ok(())
}

unsafe extern "system" fn canvas_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::um::winuser::{WM_CREATE};
    use winapi::um::winuser::{DefWindowProcW, PostMessageW};

    let handled = match msg {
        WM_CREATE => {
            PostMessageW(hwnd, NWG_INIT, 0, 2);
            true
        },
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}
