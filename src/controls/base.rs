/*!
    Low level window creation utilities
*/

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::mem;
use std::hash::Hash;

use winapi::{HWND, HINSTANCE, WNDCLASSEXW, UINT, CS_HREDRAW, CS_VREDRAW,
  COLOR_WINDOWFRAME, WM_CREATE, WM_CLOSE, WPARAM, LPARAM, LRESULT, IDC_ARROW,
  WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_VISIBLE, WS_CHILD, WS_OVERLAPPED,
  WS_OVERLAPPEDWINDOW, WS_CAPTION, WS_SYSMENU, WS_MINIMIZEBOX, WS_MAXIMIZEBOX};
use kernel32::{GetModuleHandleW, GetLastError};
use user32::{LoadCursorW, RegisterClassExW, PostQuitMessage, DefWindowProcW,
  CreateWindowExW};

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
    Custom window procedure for none built-in types
*/
unsafe extern "system" fn wndproc<ID: Eq+Hash+Clone>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
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
            let controls: &mut ::ControlCollection<ID> = unsafe{ &mut *ui.controls };
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
        Ok(hwnd)
    }
}