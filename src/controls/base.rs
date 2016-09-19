/*!
    Low level window creation utilities
*/

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::raw::c_int;
use std::ptr;
use std::mem;
use std::hash::Hash;

use events::{Event, EventCallback};

use winapi::{HWND, HINSTANCE, WNDCLASSEXW, UINT, CS_HREDRAW, CS_VREDRAW,
  COLOR_WINDOWFRAME, WM_CREATE, WM_CLOSE, WPARAM, LPARAM, LRESULT, IDC_ARROW,
  WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_VISIBLE, WS_CHILD, WS_OVERLAPPED,
  WS_OVERLAPPEDWINDOW, WS_CAPTION, WS_SYSMENU, WS_MINIMIZEBOX, WS_MAXIMIZEBOX,
  GWLP_USERDATA, WM_LBUTTONUP, WM_RBUTTONUP, WM_MBUTTONUP, GET_X_LPARAM, GET_Y_LPARAM,
  RECT, LONG, SWP_NOMOVE, SWP_NOZORDER, WM_COMMAND, BN_CLICKED, HIWORD};

use user32::{LoadCursorW, RegisterClassExW, PostQuitMessage, DefWindowProcW,
  CreateWindowExW, UnregisterClassW, SetWindowLongPtrW, GetWindowLongPtrW,
  GetClientRect, SetWindowPos};

use kernel32::{GetModuleHandleW, GetLastError};

const CLASS_NAME: &'static str = "RustyWindow";

pub struct WindowBase<ID: Eq+Hash+Clone> {
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub visible: bool,
    pub resizable: bool,
    pub class: Option<String>,
    pub parent: Option<ID>
}

/**
    Map system events to application events
*/
fn map_command(handle: HWND, evt: UINT, w: WPARAM, l: LPARAM) -> (Event, HWND) {
    let command = HIWORD(w as u32);
    match command {
        BN_CLICKED => (Event::ButtonClick, unsafe{ mem::transmute(l) } ),
        _ => (Event::Unknown, handle)
    }
}

/**
    Map system events to application events
*/
#[inline(always)]
fn map_system_event(handle: HWND, evt: UINT, w: WPARAM, l: LPARAM) -> (Event, HWND) {
    match evt {
        WM_COMMAND => map_command(handle, evt, w, l), // WM_COMMAND is a special snowflake, it can represent hundreds of different commands
        WM_LBUTTONUP => (Event::MouseUp, handle),
        WM_RBUTTONUP => (Event::MouseUp, handle),
        WM_MBUTTONUP => (Event::MouseUp, handle),
        _ => (Event::Unknown, handle)
    }
}

/**
    Translate a system button event param's
*/
fn handle_btn(msg: UINT, w: WPARAM, l: LPARAM) -> (i32, i32, u32, u32) {
    use ::constants::*;

    let (x,y): (i32, i32) = (GET_X_LPARAM(l), GET_Y_LPARAM(l));
    let modifiers = (w as u32) & (MOD_MOUSE_CTRL | MOD_MOUSE_SHIFT);
    let mut btn = (w as u32) & (BTN_MOUSE_MIDDLE | BTN_MOUSE_RIGHT | BTN_MOUSE_LEFT );
    btn |= match msg {
        WM_LBUTTONUP => BTN_MOUSE_LEFT,
        WM_RBUTTONUP => BTN_MOUSE_RIGHT,
        WM_MBUTTONUP => BTN_MOUSE_MIDDLE,
        _ => 0
    };

    (x, y, btn, modifiers)
}

/**
    Execute an event
*/
#[inline(always)]
fn dispatch_event<ID: Eq+Hash+Clone>(ec: &EventCallback<ID>, ui: &mut ::Ui<ID>, caller: &ID, msg: UINT, w: WPARAM, l: LPARAM) {
    
    match ec {
        &EventCallback::MouseUp(ref c) => {
            let (x, y, btn, modifiers) = handle_btn(msg, w, l);
            c(ui, caller, x, y, btn, modifiers);
        },
        &EventCallback::ButtonClick(ref c) => {
            c(ui, caller);
        }
        //_ => {}
    }
}

/**
    Custom window procedure for none built-in types
*/
unsafe extern "system" fn wndproc<ID: Eq+Hash+Clone>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    let (event, handle) = map_system_event(hwnd, msg, w, l);

    // If the window data was initialized, eval callbacks
    if let Some(data) = get_handle_data::<::WindowData<ID>>(handle) {
        // Build a temporary Ui that is then forgetted to pass it to the callbacks.
        let mut ui = ::Ui{controls: data.controls};
        
        // Eval the callbacks
        let functions = &data.callbacks[event as usize];
        for f in functions.iter() {
            dispatch_event::<ID>(f, &mut ui, &data.id, msg, w, l); 
        }

        mem::forget(ui);
    }
    
    match msg {
        WM_CREATE => 0,
        WM_CLOSE => {PostQuitMessage(0); 0},
        _ => DefWindowProcW(hwnd, msg, w, l)
    }
}

/**
    String to utf16. Add a trailing null char.
*/
#[inline(always)]
fn to_utf16(n: String) -> Vec<u16> {
    OsStr::new(n.as_str())
      .encode_wide()
      .chain(Some(0u16).into_iter())
      .collect()
}

/**
    Register a new window class. Return true if the class already exists 
    or the creation was successful and false if it failed.
*/
unsafe fn register_custom_class<ID: Eq+Clone+Hash>(hmod: HINSTANCE, name: &Vec<u16>) -> bool {
    let class =
        WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<ID>), 
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hmod as HINSTANCE,
            hIcon: ptr::null_mut(),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            hbrBackground: mem::transmute(COLOR_WINDOWFRAME as i64),
            lpszMenuName: ptr::null(),
            lpszClassName: name.as_ptr(),
            hIconSm: ptr::null_mut()
        };


    let class_token = RegisterClassExW(&class);
    if class_token == 0 && GetLastError() != 1410 {
        // If the class registration failed and the reason is not that
        // the class already exists (1410), return false.
        false
    } else {
        true
    }
}

/** 
    Fix: Window size include the non client area. This behaviour is not wanted
    Resize the client area to match the "true" size. 
*/
unsafe fn fix_overlapped_window_size(handle: HWND, size: (u32, u32)) {
    let mut rect: RECT = mem::uninitialized();
    GetClientRect(handle, &mut rect);

    let delta_width = size.0 - (rect.right as u32);
    let delta_height = size.1 - (rect.bottom as u32);
    
    SetWindowPos(handle, ptr::null_mut(), 0, 0,
      (size.0+delta_width) as c_int, (size.1+delta_height) as c_int,
      SWP_NOMOVE|SWP_NOZORDER);
}

/**
    Create a new window. The window details is determined by the base 
    parameters passed to the function.

    If successful, return an handle to the new window.
*/
pub unsafe fn create_base<ID: Eq+Clone+Hash>(ui: &mut ::Ui<ID>, base: WindowBase<ID>) -> Result<HWND, ()> {
    let hmod = GetModuleHandleW(ptr::null());
    let use_custom_class = base.class.is_none();

    // Resolve the parent if provided, else return an empty handle
    let parent: HWND = match base.parent {
        Some(id) => {
            let controls: &mut ::ControlCollection<ID> = &mut *ui.controls;
            match controls.get(&id) {
                Some(h) => *h,
                None => { return Err(()); }
            }
        },
        None => ptr::null_mut()
    };

    let class_name = to_utf16(base.class.unwrap_or(CLASS_NAME.to_string()));
    let window_name = to_utf16(base.text);

    // If the created control is not built-in (most likely a Window),
    // use a custom class
    if use_custom_class {
        if !register_custom_class::<ID>(hmod as HINSTANCE, &class_name) {
            return Err(())
        }
    }

    // Eval the window flags
    let mut flags = WS_CLIPCHILDREN | WS_CLIPSIBLINGS;
    if base.visible { flags |= WS_VISIBLE; }
    if !parent.is_null() { flags |= WS_CHILD; }
    if parent.is_null() { 
        if base.resizable { flags |= WS_OVERLAPPEDWINDOW; }
        else { flags |= WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_MAXIMIZEBOX}
    }

    let hwnd = CreateWindowExW(
        0, class_name.as_ptr(), window_name.as_ptr(),
        flags,
        base.position.0, base.position.1,
        base.size.0 as i32, base.size.1 as i32,
        parent,
        ptr::null_mut(),
        hmod,
        ptr::null_mut()
    );

    if hwnd.is_null() {
        Err(())
    } else {
        if flags & WS_OVERLAPPEDWINDOW != 0 {
            fix_overlapped_window_size(hwnd, base.size);
        }
        Ok(hwnd)
    }
}

/**
    Unregister the custom window class. If multiple UI manager were created
    this function will fail
*/
pub unsafe fn cleanup() {
    let hmod = GetModuleHandleW(ptr::null());
    let class_name = to_utf16(CLASS_NAME.to_string());

    UnregisterClassW(class_name.as_ptr(), hmod);
}

/**
    Store data in a window
*/
pub unsafe fn set_handle_data<T>(handle: HWND, data: T) {
    let data_raw = Box::into_raw(Box::new(data));
    SetWindowLongPtrW(handle, GWLP_USERDATA, mem::transmute(data_raw));
}

/**
    Retrieve data in a window
*/
pub unsafe fn get_handle_data<'a, T>(handle: HWND) -> Option<&'a mut T> {
    let data_ptr = GetWindowLongPtrW(handle, GWLP_USERDATA);
    if data_ptr != 0 {
        let data: *mut T = mem::transmute(data_ptr);
        Some(&mut *data)
    } else {
        None
    }
}

/**
    Remove and free data from a window
*/
pub unsafe fn free_handle_data<T>(handle: HWND) {
    let data_ptr = GetWindowLongPtrW(handle, GWLP_USERDATA);
    let data: *mut T = mem::transmute(data_ptr);
    Box::from_raw(data);

    SetWindowLongPtrW(handle, GWLP_USERDATA, mem::transmute(ptr::null_mut::<()>()));
}