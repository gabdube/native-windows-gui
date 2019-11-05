/*!
Native Windows GUI windowing base. Includes events dispatching and window creation.
*/

use winapi::shared::minwindef::{UINT, DWORD, HMODULE, WPARAM, LPARAM, LRESULT};
use winapi::shared::windef::{HWND, HMENU, HBRUSH};
use winapi::shared::basetsd::{DWORD_PTR, UINT_PTR};
use winapi::um::winuser::{WNDPROC};
use super::base_helper::{get_system_error, to_utf16};
use super::window_helper::{NOTICE_MESSAGE, NWG_INIT};
use crate::controls::ControlHandle;
use crate::{Event, MousePressEvent, SystemError};
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
        htype => panic!("Cannot bind control with an handle of type {:?}.", htype)
    }

}

/**
    Set a window subclass the uses the `process_raw_events` function of NWG.

*/
pub fn bind_raw_event_handler<F>(handle: &ControlHandle, id: UINT_PTR, f: F) 
    where F: Fn(HWND, UINT, WPARAM, LPARAM) -> Option<LRESULT> + 'static
{
    use winapi::um::commctrl::SetWindowSubclass;
    
    match handle {
        &ControlHandle::Hwnd(v) => unsafe {
            let proc: Box<F> = Box::new(f);
            let proc_data: DWORD_PTR = mem::transmute(proc);
            SetWindowSubclass(v, Some(process_raw_events::<F>), id, proc_data);
        },
        htype => panic!("Cannot bind control with an handle of type {:?}.", htype)
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
        build_sysclass(hmod, class_name, Some(blank_window_proc))?;
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

pub unsafe fn build_sysclass<'a>(
    hmod: HMODULE,
    class_name: &'a str,
    clsproc: WNDPROC,
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
        lpfnWndProc: clsproc, 
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
    use winapi::um::winuser::{WM_CREATE, WM_CLOSE, SW_HIDE};
    use winapi::um::winuser::{DefWindowProcW, PostMessageW, ShowWindow};

    let handled = match msg {
        WM_CREATE => {
            PostMessageW(hwnd, NWG_INIT, 0, 0);
            true
        },
        WM_CLOSE => {
            ShowWindow(hwnd, SW_HIDE);
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
    use winapi::um::winuser::{GetClassNameW, GetMenuItemID, NMHDR};
    use winapi::um::winuser::{WM_CLOSE, WM_COMMAND, WM_MENUCOMMAND, WM_TIMER, WM_NOTIFY, WM_HSCROLL, WM_VSCROLL, WM_LBUTTONDOWN, WM_LBUTTONUP,
      WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SIZE, WM_MOVE};
    use winapi::um::winnt::WCHAR;
    use winapi::shared::minwindef::HIWORD;

    let callback: Box<F> = mem::transmute(data);
    let base_handle = ControlHandle::Hwnd(hwnd);

    match msg {
        WM_NOTIFY => {
            let notif_ptr: *const NMHDR = mem::transmute(l);
            let notif = &*notif_ptr;
            let handle = ControlHandle::Hwnd(notif.hwndFrom);

            let mut class_name_raw: [WCHAR; 100] = mem::zeroed();
            let count = GetClassNameW(notif.hwndFrom, class_name_raw.as_mut_ptr(), 100) as usize;
            let class_name = OsString::from_wide(&class_name_raw[..count]).into_string().unwrap_or("".to_string());

            match &class_name as &str {
                "SysDateTimePick32" => callback(datetimepick_commands(notif.code), handle),
                "SysTabControl32" => callback(tabs_commands(notif.code), handle),
                "msctls_trackbar32" => callback(track_commands(notif.code), handle),
                _ => {}
            }
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
                "ComboBox" => callback(combo_commands(message), handle),
                "Static" => callback(static_commands(child_handle, message), handle),
                "ListBox" => callback(listbox_commands(message), handle),
                _ => {}
            }
        },
        WM_TIMER => callback(Event::OnTimerTick, ControlHandle::Timer(hwnd, w as u32)),
        WM_SIZE => callback(Event::OnResize, base_handle),
        WM_MOVE => callback(Event::OnMove, base_handle),
        WM_HSCROLL => callback(Event::OnHorizontalScroll, ControlHandle::Hwnd(l as HWND)),
        WM_VSCROLL => callback(Event::OnVerticalScroll, ControlHandle::Hwnd(l as HWND)),
        WM_LBUTTONUP => callback(Event::MousePress(MousePressEvent::MousePressLeftUp), base_handle), 
        WM_LBUTTONDOWN => callback(Event::MousePress(MousePressEvent::MousePressLeftDown), base_handle), 
        WM_RBUTTONUP => callback(Event::MousePress(MousePressEvent::MousePressRightUp), base_handle), 
        WM_RBUTTONDOWN => callback(Event::MousePress(MousePressEvent::MousePressRightDown), base_handle),
        NOTICE_MESSAGE => callback(Event::OnNotice, ControlHandle::Timer(hwnd, w as u32)),
        NWG_INIT => callback(Event::OnInit, base_handle),
        WM_CLOSE => {
            callback(Event::OnWindowClose, base_handle);
        },
        _ => {}
    }

    mem::forget(callback);

    DefSubclassProc(hwnd, msg, w, l)
}

/**
    A window subclass procedure that dispatch the windows control events to the associated application control
*/
#[allow(unused_variables)]
unsafe extern "system" fn process_raw_events<F>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT 
    where F: Fn(HWND, UINT, WPARAM, LPARAM) -> Option<LRESULT> + 'static
{
    let callback: Box<F> = mem::transmute(data);
    let result = callback(hwnd, msg, w, l);
    mem::forget(callback);

    match result {
        Some(r) => r,
        None => ::winapi::um::commctrl::DefSubclassProc(hwnd, msg, w, l)
    }
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

fn combo_commands(m: u16) -> Event {
    use winapi::um::winuser::{CBN_CLOSEUP, CBN_DROPDOWN, CBN_SELCHANGE};
    match m {
        CBN_CLOSEUP => Event::OnComboBoxClosed,
        CBN_DROPDOWN => Event::OnComboBoxDropdown,
        CBN_SELCHANGE => Event::OnComboxBoxSelection,
        _ => Event::Unknown
    }
}

fn datetimepick_commands(m: u32) -> Event {
    use winapi::um::commctrl::{DTN_CLOSEUP, DTN_DROPDOWN, DTN_DATETIMECHANGE};
    match m {
        DTN_CLOSEUP => Event::OnDatePickerClosed,
        DTN_DROPDOWN => Event::OnDatePickerDropdown,
        DTN_DATETIMECHANGE => Event::OnDatePickerChanged,
        _ => Event::Unknown
    }
}

fn tabs_commands(m: u32) -> Event {
    use winapi::um::commctrl::{TCN_SELCHANGE, TCN_SELCHANGING};
    match m {
        TCN_SELCHANGE => Event::TabsContainerChanged,
        TCN_SELCHANGING => Event::TabsContainerChanging,
        _ => Event::Unknown
    }
}

fn track_commands(m: u32) -> Event {
    use winapi::um::commctrl::{NM_RELEASEDCAPTURE};
    match m {
        NM_RELEASEDCAPTURE => Event::TrackBarUpdated,
        _ => Event::Unknown
    }
}

unsafe fn static_commands(handle: HWND, m: u16) -> Event {
    use winapi::um::winuser::{STN_CLICKED, STN_DBLCLK, STM_GETIMAGE, IMAGE_BITMAP};
    use winapi::um::winuser::SendMessageW;

    let has_image = SendMessageW(handle, STM_GETIMAGE, IMAGE_BITMAP as usize, 0) != 0;
    if has_image {
        match m {
            STN_CLICKED => Event::OnImageFrameClick,
            STN_DBLCLK => Event::OnImageFrameDoubleClick,
            _ => Event::Unknown
        }
    } else {
        match m {
            STN_CLICKED => Event::OnLabelClick,
            STN_DBLCLK => Event::OnLabelDoubleClick,
            _ => Event::Unknown
        }
    }   
}

unsafe fn listbox_commands(m: u16) -> Event {
    use winapi::um::winuser::{LBN_SELCHANGE, LBN_DBLCLK};

    match m {
        LBN_SELCHANGE => Event::OnListBoxSelect,
        LBN_DBLCLK => Event::OnListBoxDoubleClick,
        _ => Event::Unknown
    }
}
