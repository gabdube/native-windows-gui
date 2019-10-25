/*!
    Low level tabs utility
*/
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT};
use winapi::shared::windef::{HWND};
use super::window::build_sysclass;
use crate::{SystemError};
use std::{ptr, mem};

pub const TAB_CLASS_ID: &'static str = "NWG_TAB";


/// Create the NWG tab classes
pub fn create_tab_classes() -> Result<(), SystemError>  {
    use winapi::um::libloaderapi::GetModuleHandleW;

    let hmod = unsafe { GetModuleHandleW(ptr::null_mut()) };
    if hmod.is_null() { return Err(SystemError::GetModuleHandleFailed); }

    unsafe { 
        build_sysclass(hmod, TAB_CLASS_ID, Some(tab_proc))?;
    }

    Ok(())
}


unsafe extern "system" fn tab_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::um::winuser::{PAINTSTRUCT, WM_CREATE, WM_PAINT, COLOR_BTNFACE};
    use winapi::um::winuser::{DefWindowProcW, BeginPaint, FillRect, EndPaint};

    let handled = match msg {
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = mem::zeroed();

            let hdc = BeginPaint(hwnd, &mut ps); 
            FillRect(hdc, &ps.rcPaint, mem::transmute(COLOR_BTNFACE as usize));
            EndPaint(hwnd, &ps); 

            return 1;
        },
        WM_CREATE => true,
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}
