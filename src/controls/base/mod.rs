pub mod helper;
mod sysproc;

pub use ::controls::base::helper::*;
use ::controls::base::sysproc::{sub_wndproc, wndproc};

use std::ptr;
use std::mem;
use std::hash::Hash;

use winapi::{HWND, HINSTANCE, UINT, ACTCTXW, ULONG, ULONG_PTR, MAX_PATH,
  WS_VISIBLE, WS_CHILD, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW, WS_CAPTION, WS_SYSMENU,
  WS_MINIMIZEBOX, WS_MAXIMIZEBOX, WNDCLASSEXW, WS_EX_COMPOSITED,
  CS_HREDRAW, CS_VREDRAW,
  COLOR_WINDOW, IDC_ARROW, GWLP_USERDATA};

use user32::{CreateWindowExW, LoadCursorW, RegisterClassExW, UnregisterClassW, 
  SetWindowLongPtrW, GetWindowLongPtrW, DestroyWindow};

use kernel32::{GetModuleHandleW, GetLastError, ActivateActCtx, CreateActCtxW,
  GetSystemDirectoryW};

use comctl32::{SetWindowSubclass};

const CLASS_NAME: &'static str = "RustyWindow";

pub struct WindowBase<ID: Eq+Hash+Clone> {
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub visible: bool,
    pub resizable: bool,
    pub extra_style: u32,
    pub class: Option<String>,
    pub parent: Option<ID>
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
            hbrBackground: mem::transmute(COLOR_WINDOW as i64),
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
            let controls: &mut ::ControlCollection<ID> = &mut *ui.controls;
            match controls.get(&id) {
                Some(&(h,_)) => h,
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
    let mut flags = 0;
    if base.visible { flags |= WS_VISIBLE; }
    if !parent.is_null() { flags |= WS_CHILD; }
    if parent.is_null() { 
        if base.resizable { flags |= WS_OVERLAPPEDWINDOW; }
        else { flags |= WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_MAXIMIZEBOX}
    }

    flags |= base.extra_style;

    let hwnd = CreateWindowExW(
        WS_EX_COMPOSITED, class_name.as_ptr(), window_name.as_ptr(),
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

        if !use_custom_class {
            // Inject a custom window proc in a native window
            SetWindowSubclass(hwnd, Some(sub_wndproc::<ID>), 1, 0);
        }

        Ok(hwnd)
    }
}

/**
    hackish way to enable visual style without a manifest
*/
pub unsafe fn enable_visual_styles() {
    use constants::{ACTCTX_FLAG_RESOURCE_NAME_VALID, ACTCTX_FLAG_SET_PROCESS_DEFAULT, ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID};

    let mut sys_dir: Vec<u16> = Vec::with_capacity(MAX_PATH);
    sys_dir.set_len(MAX_PATH);
    GetSystemDirectoryW(sys_dir.as_mut_ptr(), MAX_PATH as u32);

    let mut source = to_utf16("shell32.dll".to_string());

    let mut activation_cookie: ULONG_PTR = 0;
    let mut act_ctx = ACTCTXW {
        cbSize: mem::size_of::<ACTCTXW> as ULONG,
        dwFlags: ACTCTX_FLAG_RESOURCE_NAME_VALID | ACTCTX_FLAG_SET_PROCESS_DEFAULT | ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID,
        lpSource: source.as_mut_ptr(),
        wProcessorArchitecture: 0,
        wLangId: 0,
        lpAssemblyDirectory: sys_dir.as_mut_ptr(),
        lpResourceName: mem::transmute(124usize), // ID_MANIFEST
        lpApplicationName: ptr::null_mut(),
        hModule: ptr::null_mut()
    };

    let handle = CreateActCtxW(&mut act_ctx);
    ActivateActCtx(handle, &mut activation_cookie);
}

////
//// Window data helper
////

/**
    Unregister the custom window class. If multiple UI manager were created
    this function will fail (silently)
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

/**
    Remove and free data from a window and destroy the window.
*/
pub unsafe fn free_handle<T>(handle: HWND) {
    let data_ptr = GetWindowLongPtrW(handle, GWLP_USERDATA);
    let data: *mut T = mem::transmute(data_ptr);
    DestroyWindow(handle);
    Box::from_raw(data);
}