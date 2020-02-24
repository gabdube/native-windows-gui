/*!
    Low level tabs utility
*/
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT};
use winapi::shared::windef::{HWND};
use super::window::build_sysclass;
use crate::{NwgError};
use std::{ptr};

pub const TAB_CLASS_ID: &'static str = "NWG_TAB";


/// Create the NWG tab classes
pub fn create_tab_classes() -> Result<(), NwgError>  {
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::shared::windef::HBRUSH;
    use winapi::um::winuser::COLOR_BTNFACE;

    let hmod = unsafe { GetModuleHandleW(ptr::null_mut()) };
    if hmod.is_null() { return Err(NwgError::initialization("GetModuleHandleW failed")); }

    unsafe { 
        build_sysclass(hmod, TAB_CLASS_ID, Some(tab_proc), Some(COLOR_BTNFACE as HBRUSH), None)?;
    }

    Ok(())
}


unsafe extern "system" fn tab_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::um::winuser::{WM_CREATE};
    use winapi::um::winuser::{DefWindowProcW};

    let handled = match msg {
        WM_CREATE => Some(0),
        _ => None
    };

    if let Some(result) = handled {
        result
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}
