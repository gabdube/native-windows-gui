/*!
    Low level extern canvas utility
*/
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT};
use winapi::shared::windef::{HWND};
use super::window::build_sysclass;
use crate::NwgError;
use std::ptr;

pub const EXT_CANVAS_CLASS_ID: &'static str = "NWG_EXTERN_CANVAS";


/// Create the NWG tab classes
pub fn create_extern_canvas_classes() -> Result<(), NwgError>  {
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::shared::windef::HBRUSH;
    use winapi::um::winuser::{CS_HREDRAW, CS_VREDRAW, CS_OWNDC};

    let hmod = unsafe { GetModuleHandleW(ptr::null_mut()) };
    if hmod.is_null() { return Err(NwgError::initialization("GetModuleHandleW failed")); }

    unsafe { 
        build_sysclass(hmod, EXT_CANVAS_CLASS_ID, Some(extern_canvas_proc), Some(0 as HBRUSH), Some(CS_OWNDC|CS_VREDRAW|CS_HREDRAW))?;
    }

    Ok(())
}


unsafe extern "system" fn extern_canvas_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::um::winuser::{WM_CREATE, WM_ERASEBKGND};
    use winapi::um::winuser::DefWindowProcW;

    let handled = match msg {
        WM_CREATE => Some(0),
        WM_ERASEBKGND => Some(1),
        _ => None
    };

    if let Some(result) = handled {
        result
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}
