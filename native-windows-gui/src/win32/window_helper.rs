use super::base_helper::{to_utf16, from_utf16};
use super::high_dpi;
use winapi::shared::windef::{HFONT, HWND};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT};
use winapi::um::winuser::WM_USER;
use winapi::ctypes::c_int;
use std::{ptr, mem};


pub const NOTICE_MESSAGE: UINT = WM_USER+100;
pub const NWG_INIT: UINT = WM_USER + 101;
pub const NWG_TRAY: UINT = WM_USER + 102;


/// Haha you maybe though that destroying windows would be easy right? WRONG.
/// The window children must first be destroyed otherwise `DestroyWindow` will free them and the associated rust value will be ~CORRUPTED~
pub fn destroy_window(hwnd: HWND) { 
    use winapi::um::winuser::{SetParent, DestroyWindow};

    // Remove the children from the window
    iterate_window_children(hwnd, |child| {
        unsafe {
            set_window_visibility(child, false);
            SetParent(child, ptr::null_mut());
        }
    });

    unsafe { DestroyWindow(hwnd); }
}

/// Execute the callback for each first level children of the window 
pub fn iterate_window_children<F>(hwnd_parent: HWND, cb: F) 
    where F: FnMut(HWND) -> ()
{
    use winapi::um::winuser::EnumChildWindows;
    use winapi::shared::minwindef::BOOL;

    struct EnumChildData<F> {
        parent: HWND,
        callback: F,
    }

    unsafe extern "system" fn enum_child<F>(hwnd: HWND, p: LPARAM) -> BOOL 
        where F: FnMut(HWND) -> ()
    {
        // Only iterate over the top level children
        let enum_data_ptr = p as *mut EnumChildData<F>;
        let enum_data = &mut *enum_data_ptr;
        if get_window_parent(hwnd) == enum_data.parent {
            (enum_data.callback)(hwnd);
        };

        1
    }

    unsafe {
        let mut data = EnumChildData {
            parent: hwnd_parent,    
            callback: cb
        };
        EnumChildWindows(hwnd_parent, Some(enum_child::<F>), &mut data as *mut EnumChildData<F> as _);
    }
}

#[cfg(any(feature="timer", feature="notice"))]
pub fn window_valid(hwnd: HWND) -> bool {
    use winapi::um::winuser::IsWindow;

    unsafe {
        IsWindow(hwnd) != 0
    }
}

pub fn get_window_parent(hwnd: HWND) -> HWND {
    use winapi::um::winuser::GetParent;
    unsafe { GetParent(hwnd) }
}

pub fn get_window_font(handle: HWND) -> HFONT {
    use winapi::um::winuser::{ WM_GETFONT };
    unsafe { 
        let h = send_message(handle, WM_GETFONT, 0, 0);
        mem::transmute(h)
    }
}

/// Set the font of a window
pub unsafe fn set_window_font(handle: HWND, font_handle: Option<HFONT>, redraw: bool) {
    use winapi::um::winuser::WM_SETFONT;
    use winapi::um::winuser::SendMessageW;

    let font_handle = font_handle.unwrap_or(ptr::null_mut());

    SendMessageW(handle, WM_SETFONT, mem::transmute(font_handle), redraw as LPARAM);
}


#[cfg(feature = "timer")]
pub fn kill_timer(hwnd: HWND, id: u32) {
    use winapi::um::winuser::KillTimer;
    use winapi::shared::basetsd::UINT_PTR;

    unsafe {
        KillTimer(hwnd, id as UINT_PTR);
    }
}

#[cfg(feature = "timer")]
pub fn start_timer(hwnd: HWND, id: u32, interval: u32) {
    use winapi::um::winuser::SetTimer;
    use winapi::shared::basetsd::UINT_PTR;

    unsafe {
        SetTimer(hwnd, id as UINT_PTR, interval, None);
    }
}

pub fn get_style(handle: HWND) -> UINT {
    use ::winapi::um::winuser::GWL_STYLE;
    get_window_long(handle, GWL_STYLE) as UINT
}

#[cfg(any(feature = "list-view", feature = "progress-bar"))]
pub fn set_style(handle: HWND, style: u32) {
    use ::winapi::um::winuser::GWL_STYLE;
    set_window_long(handle, GWL_STYLE, style as usize);
}

pub fn send_message(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    unsafe { ::winapi::um::winuser::SendMessageW(hwnd, msg, w, l) }
}

pub fn post_message(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) {
    unsafe { ::winapi::um::winuser::PostMessageW(hwnd, msg, w, l) };
}

pub unsafe fn set_focus(handle: HWND) {
    ::winapi::um::winuser::SetFocus(handle);
}

pub unsafe fn get_focus(handle: HWND) -> bool {
    ::winapi::um::winuser::GetFocus() == handle
}

pub unsafe fn get_window_text(handle: HWND) -> String {
    use winapi::um::winuser::{GetWindowTextW, GetWindowTextLengthW};

    let mut buffer_size = GetWindowTextLengthW(handle) as usize;
    if buffer_size == 0 { return String::new(); }

    buffer_size += 1;
    let mut buffer: Vec<u16> = Vec::with_capacity(buffer_size);
    buffer.set_len(buffer_size);

    if GetWindowTextW(handle, buffer.as_mut_ptr(), buffer_size as c_int) == 0 {
        String::new()
    } else {
        from_utf16(&buffer[..])
    }
}

pub unsafe fn set_window_text<'a>(handle: HWND, text: &'a str) {
    use winapi::um::winuser::{SetWindowTextW};

    let text = to_utf16(text);
    SetWindowTextW(handle, text.as_ptr());
}

pub unsafe fn set_window_position(handle: HWND, x: i32, y: i32) {
    use winapi::um::winuser::SetWindowPos;
    use winapi::um::winuser::{SWP_NOZORDER, SWP_NOSIZE, SWP_NOACTIVATE};

    let (x, y) = high_dpi::logical_to_physical(x, y);
    SetWindowPos(handle, ptr::null_mut(), x as c_int, y as c_int, 0, 0, SWP_NOZORDER|SWP_NOSIZE|SWP_NOACTIVATE);
}

pub unsafe fn get_window_position(handle: HWND) -> (i32, i32) {
    use winapi::um::winuser::{GetWindowRect, ScreenToClient, GetParent};
    use winapi::shared::windef::{RECT, POINT};
    
    let mut r: RECT = mem::zeroed();
    GetWindowRect(handle, &mut r);

    let parent = GetParent(handle);
    let (x, y) = if !parent.is_null() {
        let mut pt = POINT{x: r.left, y: r.top};
        ScreenToClient(parent, &mut pt);
        (pt.x as i32, pt.y as i32)
    } else {
        (r.left as i32, r.top as i32)
    };

    high_dpi::physical_to_logical(x, y)
}

pub unsafe fn set_window_size(handle: HWND, w: u32, h: u32, fix: bool) {
    use winapi::um::winuser::{SetWindowPos, AdjustWindowRectEx, GetWindowLongW};
    use winapi::um::winuser::{SWP_NOZORDER, SWP_NOMOVE, SWP_NOACTIVATE, SWP_NOCOPYBITS, GWL_STYLE, GWL_EXSTYLE};
    use winapi::shared::windef::RECT;

    let (mut w, mut h) = high_dpi::logical_to_physical(w as i32, h as i32);

    if fix {
        let flags = GetWindowLongW(handle, GWL_STYLE) as u32;
        let ex_flags = GetWindowLongW(handle, GWL_EXSTYLE) as u32;
        let mut rect = RECT {left: 0, top: 0, right: w, bottom: h};
        AdjustWindowRectEx(&mut rect, flags, 0, ex_flags);

        w = rect.right - rect.left;
        h = rect.bottom  - rect.top;
    }

    SetWindowPos(handle, ptr::null_mut(), 0, 0, w, h, SWP_NOZORDER|SWP_NOMOVE|SWP_NOACTIVATE|SWP_NOCOPYBITS);
}

pub unsafe fn get_window_size(handle: HWND) -> (u32, u32) {
    get_window_size_impl(handle, false)
}

#[allow(unused)]
pub unsafe fn get_window_physical_size(handle: HWND) -> (u32, u32) {
    get_window_size_impl(handle, true)
}

unsafe fn get_window_size_impl(handle: HWND, return_physical: bool) -> (u32, u32) {
    use winapi::um::winuser::GetClientRect;
    use winapi::shared::windef::RECT;
    
    let mut r: RECT = mem::zeroed();
    GetClientRect(handle, &mut r);

    let (w, h) = if return_physical {
        (r.right, r.bottom)
    } else {
        high_dpi::physical_to_logical(r.right, r.bottom)
    };

    (w as u32, h as u32)
}

pub unsafe fn set_window_visibility(handle: HWND, visible: bool) {
    use winapi::um::winuser::ShowWindow;
    use winapi::um::winuser::{SW_HIDE, SW_SHOW};

    let visible = if visible { SW_SHOW } else { SW_HIDE };
    ShowWindow(handle, visible);
}

pub unsafe fn get_window_visibility(handle: HWND) -> bool {
    use winapi::um::winuser::IsWindowVisible;
    IsWindowVisible(handle) != 0
}

pub unsafe fn get_window_enabled(handle: HWND) -> bool {
    use winapi::um::winuser::{GWL_STYLE, WS_DISABLED};

    let style = get_window_long(handle, GWL_STYLE) as UINT;
    (style & WS_DISABLED) != WS_DISABLED
}

pub unsafe fn set_window_enabled(handle: HWND, enabled: bool) {
    use winapi::um::winuser::{GWL_STYLE, WS_DISABLED};
    use winapi::um::winuser::{UpdateWindow, InvalidateRect};

    let old_style = get_window_long(handle, GWL_STYLE) as usize;
    if enabled {
        set_window_long(handle, GWL_STYLE, old_style&(!WS_DISABLED as usize) );
    } else {
        set_window_long(handle, GWL_STYLE, old_style|(WS_DISABLED as usize));
    }

    // Tell the control to redraw itself to show the new style.
    InvalidateRect(handle, ptr::null(), 1);
    UpdateWindow(handle);
}

#[cfg(feature = "tabs")]
pub unsafe fn get_window_class_name(handle: HWND) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use winapi::shared::ntdef::WCHAR;
    use winapi::um::winuser::GetClassNameW;

    let mut class_name_raw: Vec<WCHAR> = Vec::with_capacity(100); 
    class_name_raw.set_len(100);

    let count = GetClassNameW(handle, class_name_raw.as_mut_ptr(), 100) as usize;
    
    OsString::from_wide(&class_name_raw[..count]).into_string().unwrap_or("".to_string())
}

#[cfg(target_arch = "x86")] use winapi::shared::ntdef::LONG;
#[cfg(target_arch = "x86_64")] use winapi::shared::basetsd::LONG_PTR;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn get_window_long(handle: HWND, index: c_int) -> LONG_PTR {
    unsafe{ ::winapi::um::winuser::GetWindowLongPtrW(handle, index) }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn get_window_long(handle: HWND, index: c_int) -> LONG {
    unsafe { ::winapi::um::winuser::GetWindowLongW(handle, index) }
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn set_window_long(handle: HWND, index: c_int, v: usize) {
    unsafe{ ::winapi::um::winuser::SetWindowLongPtrW(handle, index, v as LONG_PTR); }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn set_window_long(handle: HWND, index: c_int, v: usize) {
    unsafe { ::winapi::um::winuser::SetWindowLongW(handle, index, v as LONG); }
}
