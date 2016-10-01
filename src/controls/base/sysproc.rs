use std::mem;
use std::hash::Hash;

use controls::base::{get_handle_data};
use events::{Event, EventCallback};
use ::constants::{MOD_MOUSE_CTRL, MOD_MOUSE_SHIFT, BTN_MOUSE_MIDDLE, BTN_MOUSE_RIGHT,
 BTN_MOUSE_LEFT};

use winapi::{HWND, UINT, WM_CREATE, WM_CLOSE, WPARAM, LPARAM, LRESULT,
  WM_LBUTTONUP, WM_RBUTTONUP, WM_MBUTTONUP, GET_X_LPARAM, GET_Y_LPARAM,
  WM_COMMAND, HIWORD, BN_CLICKED, BN_SETFOCUS, BN_KILLFOCUS, WM_ACTIVATEAPP,
  UINT_PTR, DWORD_PTR, EN_SETFOCUS, EN_KILLFOCUS, EN_MAXTEXT, EN_CHANGE,
  WM_DESTROY};

use user32::{PostQuitMessage, DefWindowProcW};
use comctl32::{DefSubclassProc};

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
        WM_DESTROY => (Event::Removed, handle),
        _ => (Event::Unknown, handle)
    }
}

/**
    Translate a system button event param's
*/
fn handle_btn(msg: UINT, w: WPARAM, l: LPARAM) -> (i32, i32, u32, u32) {
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
        &EventCallback::Click(ref c) | &EventCallback::ValueChanged(ref c) | &EventCallback::MaxValue(ref c) | 
        &EventCallback::Removed(ref c) => {
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
        //_ => {}
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
