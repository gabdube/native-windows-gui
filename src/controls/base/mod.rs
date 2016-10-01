pub mod helper;

pub use ::controls::base::helper::*;

use std::ptr;
use std::mem;
use std::hash::Hash;

use events::{Event, EventCallback};

use winapi::{HWND, HINSTANCE, UINT, WM_CREATE, WM_CLOSE, WPARAM, LPARAM, LRESULT,
  WS_VISIBLE, WS_CHILD, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW, WS_CAPTION, WS_SYSMENU,
  WS_MINIMIZEBOX, WS_MAXIMIZEBOX, WM_LBUTTONUP, WM_RBUTTONUP, WM_MBUTTONUP,
  GET_X_LPARAM, GET_Y_LPARAM, WM_COMMAND, HIWORD, BN_CLICKED, BN_SETFOCUS,
  BN_KILLFOCUS, WM_ACTIVATEAPP, UINT_PTR, DWORD_PTR, EN_SETFOCUS,
  EN_KILLFOCUS, EN_MAXTEXT, EN_CHANGE, WS_EX_COMPOSITED};

use user32::{PostQuitMessage, DefWindowProcW, CreateWindowExW};

use kernel32::{GetModuleHandleW};

use comctl32::{SetWindowSubclass, DefSubclassProc};

/**
    Map system events to application events
*/
fn map_command(handle: HWND, evt: UINT, w: WPARAM, l: LPARAM) -> (Event, HWND) {
    let command = HIWORD(w as u32);
    let owner: HWND = unsafe{ mem::transmute(l) };
    match command {
        BN_SETFOCUS | BN_KILLFOCUS | EN_SETFOCUS | EN_KILLFOCUS  => (Event::Focus, owner),
        EN_CHANGE => (Event::ValueChanged, owner),
        EN_MAXTEXT => (Event::MaxValue, owner),
        BN_CLICKED => (Event::Click, owner),
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
        WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP => (Event::MouseUp, handle),
        WM_ACTIVATEAPP => (Event::Focus, handle),
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
        &EventCallback::Click(ref c) | &EventCallback::ValueChanged(ref c) | &EventCallback::MaxValue(ref c) => {
            c(ui, caller); 
         },
        &EventCallback::Focus(ref c) => {
            let focus = match msg {
                WM_COMMAND => { HIWORD(w as u32) == BN_SETFOCUS },
                WM_ACTIVATEAPP => w == 1,
                _ => unreachable!()
            };
            c(ui, caller, focus);
        },
        _ => {}
    }
}

/**
    Window proc for subclasses
*/
pub unsafe extern "system" fn sub_wndproc<ID: Eq+Hash+Clone>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id_subclass: UINT_PTR, dref: DWORD_PTR) -> LRESULT {
    let (event, handle) = map_system_event(hwnd, msg, w, l);

    // If the window data was initialized, eval callbacks
    if let Some(data) = get_handle_data::<::WindowData<ID>>(handle) {
        // Build a temporary Ui that is then forgetted to pass it to the callbacks.
        let mut ui = ::Ui{controls: data.controls};

        // Eval the callbacks
        if let Some(functions) = data.callbacks.get(&event) {
            for f in functions.iter() {
                dispatch_event::<ID>(f, &mut ui, &data.id, msg, w, l); 
            }
        }
        
        mem::forget(ui);
    }

    return DefSubclassProc(hwnd, msg, w, l);
}

/**
    Custom window procedure for none built-in types
*/
pub unsafe extern "system" fn wndproc<ID: Eq+Hash+Clone>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    let (event, handle) = map_system_event(hwnd, msg, w, l);

    // If the window data was initialized, eval callbacks
    if let Some(data) = get_handle_data::<::WindowData<ID>>(handle) {
        // Build a temporary Ui that is then forgetted to pass it to the callbacks.
        let mut ui = ::Ui{controls: data.controls};
        
        // Eval the callbacks
        if let Some(functions) = data.callbacks.get(&event) {
            for f in functions.iter() {
                dispatch_event::<ID>(f, &mut ui, &data.id, msg, w, l); 
            }
        }
        
        mem::forget(ui);
    }

    match msg {
        WM_CREATE => 0,
        WM_CLOSE => {PostQuitMessage(0); 0},
        _ =>  DefWindowProcW(hwnd, msg, w, l)
    }
}


////////////////////////////////////////////////

pub const CLASS_NAME: &'static str = "RustyWindow";

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