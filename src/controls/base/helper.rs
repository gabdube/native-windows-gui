/*!
    Low level window creation utilities
*/

use controls::base::{get_handle_data};

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::os::raw::{c_int};
use std::ptr;
use std::mem;
use std::hash::Hash;

use actions::{ActionReturn, ActMessageParams};
use constants::{Error, WindowDisplay, CheckState, BM_GETSTATE, BST_CHECKED, BST_INDETERMINATE, BST_UNCHECKED, BM_SETCHECK};

use winapi::{HWND, UINT, WPARAM, LPARAM, LRESULT, WS_CHILD, WS_OVERLAPPEDWINDOW,
  WS_CAPTION, WS_SYSMENU, WS_MINIMIZEBOX, WS_MAXIMIZEBOX, RECT, SW_RESTORE,
  SWP_NOMOVE, SWP_NOZORDER, POINT, LONG, SWP_NOSIZE, GWL_STYLE, LONG_PTR, WS_BORDER,
    WS_THICKFRAME, BOOL, SW_SHOW, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE};   

use user32::{SetWindowLongPtrW, GetWindowLongPtrW, EnumChildWindows, ShowWindow, 
  IsZoomed, IsIconic, GetClientRect, SetWindowPos, SetWindowTextW, GetWindowTextW, 
  GetWindowTextLengthW, MessageBoxW, ScreenToClient, GetWindowRect, GetParent,
  SetParent, SendMessageW, EnableWindow, IsWindowEnabled, IsWindowVisible};

/**
    String to utf16. Add a trailing null char.
*/
#[inline(always)]
pub fn to_utf16(n: String) -> Vec<u16> {
    OsStr::new(n.as_str())
      .encode_wide()
      .chain(Some(0u16).into_iter())
      .collect()
}

/** 
    Fix: Window size include the non client area. This behaviour is not wanted
    Resize the client area to match the "true" size. 
*/
pub unsafe fn fix_overlapped_window_size(handle: HWND, size: (u32, u32)) {
    let mut rect: RECT = mem::uninitialized();
    GetClientRect(handle, &mut rect);

    let delta_width = size.0 - (rect.right as u32);
    let delta_height = size.1 - (rect.bottom as u32);
    
    SetWindowPos(handle, ptr::null_mut(), 0, 0,
      (size.0+delta_width) as c_int, (size.1+delta_height) as c_int,
      SWP_NOMOVE|SWP_NOZORDER);
}


////
//// Actions functions shared by multiple controls
////

/**
    Thin wrapper around SendMessageW
*/
pub fn send_message(handle: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT { unsafe {
    SendMessageW(handle, msg, w, l)
}}

/**
    Set the text of a window.
*/
pub fn set_window_text<ID: Eq+Hash+Clone>(handle: HWND, _text: String) -> ActionReturn<ID> { unsafe {
    let text = to_utf16(_text);
    SetWindowTextW(handle, text.as_ptr());
    ActionReturn::None
}}

/**
    Get the text of a window.
*/
pub fn get_window_text<ID: Eq+Hash+Clone>(handle: HWND) -> ActionReturn<ID> { unsafe {
    let text_length = (GetWindowTextLengthW(handle) as usize)+1;
    let mut buffer: Vec<u16> = Vec::with_capacity(text_length);
    buffer.set_len(text_length);

    GetWindowTextW(handle, buffer.as_mut_ptr(), text_length as i32);

    let text = OsString::from_wide(&(buffer.as_slice()[0..text_length-1]));
    let text = text.into_string().unwrap_or("ERROR!".to_string());
    ActionReturn::Text(Box::new(text))
}}

/**
    Create a messagebox from params.
*/
pub fn show_message<ID: Eq+Hash+Clone>(handle: HWND, params: ActMessageParams) -> ActionReturn<ID> { unsafe {
    let text = to_utf16(params.content);
    let title = to_utf16(params.title);
    MessageBoxW(handle, text.as_ptr(), title.as_ptr(), params.type_ as UINT);
    ActionReturn::None
}}

/**
    Return the position of a window
*/
pub fn get_window_pos<ID: Eq+Hash+Clone>(handle: HWND, from_parent: bool) -> ActionReturn<ID> { unsafe {
    let mut rect: RECT = mem::uninitialized();
    GetWindowRect(handle, &mut rect);

    if !from_parent {
        return ActionReturn::Position(rect.left as i32, rect.top as i32);
    }

    let mut point: POINT = POINT{x: rect.left as LONG, y: rect.top as LONG};
    ScreenToClient(GetParent(handle), &mut point);
    
    ActionReturn::Position(point.x as i32, point.y as i32)
}}

/**
    Set the position of a window
*/
pub fn set_window_pos<ID: Eq+Hash+Clone>(handle: HWND, x: i32, y:i32) -> ActionReturn<ID> { unsafe {
    SetWindowPos(handle, ptr::null_mut(), x, y, 0, 0, SWP_NOZORDER | SWP_NOSIZE);
    ActionReturn::None
}}

/**
    Get the size of a Window
*/
pub fn get_window_size<ID: Eq+Hash+Clone>(handle: HWND) -> ActionReturn<ID> { unsafe {
    let mut rect: RECT = mem::uninitialized();
    GetClientRect(handle, &mut rect);

    ActionReturn::Size(rect.right as u32, rect.bottom as u32)
}}

/**
    Set the size of a Window
*/
pub fn set_window_size<ID: Eq+Hash+Clone>(handle: HWND, w: u32, h:u32) -> ActionReturn<ID> { unsafe {
    SetWindowPos(handle, ptr::null_mut(), 0, 0, w as c_int, h as c_int, SWP_NOMOVE|SWP_NOZORDER);
    ActionReturn::None
}}

/**
    Return the ui identifier of a window or None if there is none.
*/
pub fn get_window_parent<ID: Eq+Hash+Clone>(handle: HWND) -> ActionReturn<ID> { unsafe {
    let parent = GetParent(handle);

    if let Some(d) = get_handle_data::<::WindowData<ID>>(parent) {
        ActionReturn::Parent(Box::new(Some(d.id.clone())))
    } else {
        ActionReturn::Parent(Box::new(None))
    }
}}


/**
    Set or removed window style when a parent is added or removed from a control.
*/
fn set_parent_update_style(handle: HWND, parent_removed: bool) { unsafe {
    let mut style = GetWindowLongPtrW(handle, GWL_STYLE);

    if parent_removed {
        // When removing parents, set the window style to overlapped
        let child = WS_CHILD as LONG_PTR;
        style |= WS_OVERLAPPEDWINDOW as LONG_PTR;
        if style & child != 0 { style ^= child; }
    } else {
        // Remove any window styles if found
        style |= WS_CHILD as LONG_PTR;
        for i in [WS_CAPTION, WS_SYSMENU, WS_MINIMIZEBOX, WS_THICKFRAME, WS_BORDER, WS_MAXIMIZEBOX].iter() {
            let i = *i as LONG_PTR;
            if style & i != 0 { 
                style ^= i; 
            }
        }
    }

    SetWindowLongPtrW(handle, GWL_STYLE, style);
}}

/**
    Set or remove the parent of a window. 
    If the control must have a parent, setting `force_parent` to true will make the function fail if the parent is None.
    If the parent is removed, apply the WS_OVERLAPPEDWINDOW style to the control and remove the WS_CHILD style.
*/
pub fn set_window_parent<ID: Eq+Hash+Clone>(ui: &::Ui<ID>, handle: HWND, parent: Option<ID>, force_parent: bool) -> ActionReturn<ID> { unsafe {
    match parent {
        Some(id) => {
            let controls: &mut ::ControlCollection<ID> =  &mut *ui.controls;
            if let Some(&(parent_handle, _)) = controls.get(&id) {
                set_parent_update_style(handle, false);
                SetParent(handle, parent_handle);
            } else {
                return ActionReturn::Error( Error::CONTROL_NOT_FOUND );
            }
        },
        None => {
            if force_parent { 
                return ActionReturn::Error( Error::MUST_HAVE_PARENT );
            }
            set_parent_update_style(handle, true);
            SetParent(handle, ptr::null_mut()); 
        }
    }

    ActionReturn::None
}}

unsafe extern "system" fn get_children_proc<ID: Eq+Hash+Clone>(handle: HWND, param: LPARAM) -> BOOL {
     let &(parent_handle, children_raw): &(HWND, *mut Vec<ID>) = mem::transmute(param);
     let children: &mut Vec<ID> = &mut *children_raw;

     if GetParent(handle) == parent_handle {
         if let Some(d) = get_handle_data::<::WindowData<ID>>(handle) {
            children.push(d.id.clone());
        }
     }

     1
}

unsafe extern "system" fn get_descendant_proc<ID: Eq+Hash+Clone>(handle: HWND, param: LPARAM) -> BOOL {
     let children_raw: *mut Vec<ID> = mem::transmute(param);
     let children: &mut Vec<ID> = &mut *children_raw;

     if let Some(d) = get_handle_data::<::WindowData<ID>>(handle) {
         children.push(d.id.clone());
     }

     1
}

/**
    Return the names of the window children in a Vec.
*/
pub fn get_window_children<ID: Eq+Hash+Clone>(handle: HWND) -> ActionReturn<ID> { unsafe {
    let children: *mut Vec<ID> = Box::into_raw(Box::new(Vec::new()));
    let data = (handle, children);
    EnumChildWindows(handle, Some(get_children_proc::<ID>), mem::transmute(&data));
    ActionReturn::Children(Box::from_raw(children))
}}

/**
    Return the names of the windows children in a Vec. Recursive.
*/
pub fn get_window_descendant<ID: Eq+Hash+Clone>(handle: HWND) -> ActionReturn<ID> { unsafe {
    let children: *mut Vec<ID> = Box::into_raw(Box::new(Vec::new()));
    EnumChildWindows(handle, Some(get_descendant_proc::<ID>), mem::transmute(children));
    ActionReturn::Children(Box::from_raw(children))
}}

/**
    Return True if the window is enabled, else return false.
*/
pub fn get_window_enabled<ID: Eq+Hash+Clone>(handle: HWND) -> ActionReturn<ID> { unsafe {
    ActionReturn::Enabled(IsWindowEnabled(handle) == 1)
}}

/**
    Enable or disable a window
*/
pub fn set_window_enabled<ID: Eq+Hash+Clone>(handle: HWND, enabled: bool) -> ActionReturn<ID> { unsafe {
    EnableWindow(handle, enabled as BOOL);
    ActionReturn::None
}}

/**
    Return True if the window is visible, else return false.
*/
pub fn get_window_visibility<ID: Eq+Hash+Clone>(handle: HWND) -> ActionReturn<ID> { unsafe {
    ActionReturn::Visibility(IsWindowVisible(handle) == 1)
}}

/**
    Show or hide a window
*/
pub fn set_window_visibility<ID: Eq+Hash+Clone>(handle: HWND, visible: bool) -> ActionReturn<ID> { unsafe {
    let show = if visible { SW_SHOW } else { SW_HIDE };
    ShowWindow(handle, visible as BOOL);

    ActionReturn::None
}}

/**
    Get the window display status (maximized, minimized, normal)
*/
pub fn get_window_display<ID: Eq+Hash+Clone>(handle: HWND) -> ActionReturn<ID> { unsafe {
    ActionReturn::WindowDisplay(
        if IsZoomed(handle) == 1 { WindowDisplay::Maximised }
        else if IsIconic(handle) == 1 { WindowDisplay::Minimized }
        else { WindowDisplay::Normal }
    )
}}

/**
    Set the window display status (maximized, minimized, normal)
*/
pub fn set_window_display<ID: Eq+Hash+Clone>(handle: HWND, d: WindowDisplay) -> ActionReturn<ID> { unsafe {
    match d {
        WindowDisplay::Maximised => ShowWindow(handle, SW_MAXIMIZE),
        WindowDisplay::Minimized => ShowWindow(handle, SW_MINIMIZE),
        WindowDisplay::Normal => ShowWindow(handle, SW_RESTORE)
    };
    ActionReturn::None
}}

/**
    Get the check state of a control that can be checked
*/
pub fn get_check_state<ID: Eq+Clone+Hash >(handle: HWND) -> ActionReturn<ID> {
    let state = send_message(handle, BM_GETSTATE, 0, 0) as u32;
    let state = if state & BST_CHECKED != 0 {
        CheckState::Checked
    } else if state & BST_INDETERMINATE != 0 {
        CheckState::Indeterminate
    } else {
        CheckState::Unchecked
    };

    ActionReturn::CheckState(state)
}

/**
    Set the check state of a control that can be checked
*/
pub fn set_check_state<ID: Eq+Clone+Hash >(handle: HWND, state: CheckState) -> ActionReturn<ID> {
    let state = match state {
        CheckState::Checked => BST_CHECKED,
        CheckState::Indeterminate => BST_INDETERMINATE,
        CheckState::Unchecked => BST_UNCHECKED
    };
    send_message(handle, BM_SETCHECK, state as WPARAM, 0);
    ActionReturn::None
}