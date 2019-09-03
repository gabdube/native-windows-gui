/*!
Native Windows GUI windowing base. Includes events dispatching and window creation.
*/

use winapi::shared::minwindef::{UINT, DWORD, HMODULE, WPARAM, LPARAM, LRESULT};
use winapi::shared::windef::{HWND, HMENU, HBRUSH};
use winapi::shared::basetsd::{DWORD_PTR, UINT_PTR};
use super::base_helper::{get_system_error, to_utf16};
use super::window_helper::{NOTICE_MESSAGE};
use crate::controls::ControlHandle;
use crate::{Event, SystemError};
use std::{ptr, mem};


static mut TIMER_ID: u32 = 1; 
static mut NOTICE_ID: u32 = 1; 


pub fn build_notice(parent: HWND) -> ControlHandle {
    let id = unsafe {
        let tmp = NOTICE_ID;
        NOTICE_ID += 1;
        tmp
    };
    ControlHandle::Timer(parent, id)
}

pub unsafe fn build_timer(parent: HWND, interval: u32, stopped: bool) -> ControlHandle {
    use winapi::um::winuser::SetTimer;
    
    let id = TIMER_ID;
    TIMER_ID += 1;

    if !stopped {
        SetTimer(parent, id as UINT_PTR, interval as UINT, None);
    }
    
    ControlHandle::Timer(parent, id)
}

/**
    Set a window subclass the uses the `process_events` function of NWG.
*/
pub fn bind_event_handler<F>(handle: &ControlHandle, f: F) 
    where F: Fn(Event, ControlHandle) -> () + 'static
{
    use winapi::um::commctrl::SetWindowSubclass;
    
    match handle {
        &ControlHandle::Hwnd(v) => unsafe {
            let proc: Box<F> = Box::new(f);
            let proc_data: DWORD_PTR = mem::transmute(proc);
            SetWindowSubclass(v, Some(process_events::<F>), 0, proc_data);
        },
        htype => panic!("Cannot bind control with an handle of type {:?}. THIS IS AN INTERNAL ERROR!", htype)
    }

}


/**
    High level function that handle the creation of custom window control or built in window control
*/
pub(crate) unsafe fn build_hwnd_control<'a>(
    class_name: Option<&'a str>,
    window_title: Option<&'a str>,
    size: Option<(i32, i32)>,
    pos: Option<(i32, i32)>,
    flags: Option<DWORD>,
    ex_flags: Option<DWORD>,
    forced_flags: DWORD,
    parent: Option<HWND>
) -> Result<ControlHandle, SystemError> 
{
    use winapi::um::winuser::{WS_EX_COMPOSITED, WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_CLIPCHILDREN, /*WS_EX_LAYERED*/};
    use winapi::um::winuser::{CreateWindowExW, AdjustWindowRectEx};
    use winapi::shared::windef::RECT;
    use winapi::um::libloaderapi::GetModuleHandleW;

    let hmod = GetModuleHandleW(ptr::null_mut());
    if hmod.is_null() { return Err(SystemError::GetModuleHandleFailed); }

    let class_name = class_name.unwrap_or("NativeWindowsGuiWindow");
    if class_name == "NativeWindowsGuiWindow" {
        build_sysclass(hmod, class_name)?;
    }

    let class_name = to_utf16(class_name);
    let window_title = to_utf16(window_title.unwrap_or("New Window"));
    let ex_flags = ex_flags.unwrap_or(WS_EX_COMPOSITED);
    let flags = flags.unwrap_or(WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_VISIBLE) | forced_flags;

    let (px, py) = pos.unwrap_or((0, 0));
    let (mut sx, mut sy) = size.unwrap_or((500, 500));
    let parent_handle = parent.unwrap_or(ptr::null_mut());
    let menu = ptr::null_mut();
    let lp_params = ptr::null_mut();

    if parent.is_none() {
        let mut rect = RECT {left: 0, top: 0, right: sx, bottom: sy};
        AdjustWindowRectEx(&mut rect, flags, 0, ex_flags);

        sx = rect.right - rect.left;
        sy = rect.bottom  - rect.top;
    }

    let handle = CreateWindowExW (
        ex_flags,
        class_name.as_ptr(), window_title.as_ptr(),
        flags,
        px, py,
        sx, sy,
        parent_handle,
        menu,
        hmod,
        lp_params
    );

    if handle.is_null() {
        println!("{:?}", get_system_error());
        Err(SystemError::WindowCreationFailed)
    } else {
        

        Ok(ControlHandle::Hwnd(handle))
    }
}

unsafe fn build_sysclass<'a>(
    hmod: HMODULE,
    class_name: &'a str
) -> Result<(), SystemError> 
{
    use winapi::um::winuser::{LoadCursorW, RegisterClassExW};
    use winapi::um::winuser::{CS_HREDRAW, CS_VREDRAW, COLOR_WINDOW, IDC_ARROW, WNDCLASSEXW};
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::shared::winerror::ERROR_CLASS_ALREADY_EXISTS;

    let class_name = to_utf16(class_name);
    let background: HBRUSH = mem::transmute(COLOR_WINDOW as usize);
    let style: UINT = CS_HREDRAW | CS_VREDRAW;

    let class =
    WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
        style: style,
        lpfnWndProc: Some(blank_window_proc), 
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hmod,
        hIcon: ptr::null_mut(),
        hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
        hbrBackground: background,
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: ptr::null_mut()
    };

    let class_token = RegisterClassExW(&class);
    if class_token == 0 && GetLastError() != ERROR_CLASS_ALREADY_EXISTS { 
        Err(SystemError::SystemClassCreationFailed)
    } else {
        Ok(())
    }
}

/**
    A blank system procedure used when creating new window class. Actual system event handling is done in the subclass produre `process_events`.
*/
unsafe extern "system" fn blank_window_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::um::winuser::WM_CREATE;
    use winapi::um::winuser::DefWindowProcW;

    let handled = match msg {
        WM_CREATE => true,
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}

/**
    A window subclass procedure that dispatch the windows control events to the associated application control
*/
#[allow(unused_variables)]
unsafe extern "system" fn process_events<F>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT 
    where F: Fn(Event, ControlHandle) -> () + 'static
{
    use std::os::windows::ffi::OsStringExt;
    use std::ffi::OsString;

    use winapi::um::commctrl::DefSubclassProc;
    use winapi::um::winuser::{GetClassNameW, GetMenuItemID};
    use winapi::um::winuser::{WM_CLOSE, WM_COMMAND, WM_MENUCOMMAND, WM_TIMER};
    use winapi::um::winnt::WCHAR;
    use winapi::shared::minwindef::HIWORD;

    let callback: Box<F> = mem::transmute(data);

    match msg {
        WM_CLOSE => {
            callback(Event::OnWindowClose, ControlHandle::Hwnd(hwnd));
        },
        WM_MENUCOMMAND => {
            let parent_handle: HMENU = mem::transmute(l);
            let item_id = GetMenuItemID(parent_handle, w as i32);
            let handle = ControlHandle::MenuItem(parent_handle, item_id);
            callback(Event::OnMenuItemClick, handle);
        },
        WM_COMMAND => {
            let child_handle: HWND = mem::transmute(l);
            let message = HIWORD(w as u32) as u16;
            let handle = ControlHandle::Hwnd(child_handle);
            
            // Converting the class name into rust string might not be the most efficient way to do this
            // It might be a good idea to just compare the class_name_raw
            let mut class_name_raw: Vec<WCHAR> = Vec::with_capacity(100);  class_name_raw.set_len(100);
            let count = GetClassNameW(child_handle, class_name_raw.as_mut_ptr(), 100) as usize;
            let class_name = OsString::from_wide(&class_name_raw[..count]).into_string().unwrap_or("".to_string());

            match &class_name as &str {
                "Button" => callback(button_commands(message), handle),
                "Edit" => callback(edit_commands(message), handle),
                "ComboBox" => callback(edit_combo(message), handle),
                _ => {}
            }
        },
        WM_TIMER => {
            let handle = ControlHandle::Timer(hwnd, w as u32);
            callback(Event::OnTimerTick, handle);
        },
        NOTICE_MESSAGE => {
            let handle = ControlHandle::Timer(hwnd, w as u32);
            callback(Event::OnNotice, handle);
        },
        _ => {}
    }

    mem::forget(callback);

    DefSubclassProc(hwnd, msg, w, l)
}

fn button_commands(m: u16) -> Event {
    use winapi::um::winuser::{BN_CLICKED, BN_DBLCLK};
    
    match m {
        BN_CLICKED => Event::OnButtonClick,
        BN_DBLCLK => Event::OnButtonDoubleClick,
        _ => Event::Unknown
    }
}

fn edit_commands(m: u16) -> Event {
    use winapi::um::winuser::{EN_CHANGE};

    match m {
        EN_CHANGE => Event::OnTextInput,
        _ => Event::Unknown
    }
}

fn edit_combo(m: u16) -> Event {
    use winapi::um::winuser::{CBN_CLOSEUP, CBN_DROPDOWN, CBN_DBLCLK, CBN_SELCHANGE};
    match m {
        CBN_CLOSEUP => Event::OnComboBoxClosed,
        CBN_DROPDOWN => Event::OnComboBoxDropdown,
        CBN_DBLCLK => Event::OnComboBoxDoubleClick,
        CBN_SELCHANGE => Event::OnComboxBoxSelection,
        _ => Event::Unknown
    }
}

