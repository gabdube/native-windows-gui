/*!
    Low level events functions
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::any::TypeId;
use std::hash::Hash;
use std::mem;
use std::ptr;

use winapi::{DWORD, DWORD_PTR, HMENU, HWND, LPARAM, LRESULT, UINT, UINT_PTR, WPARAM};

use controls::{AnyHandle, ControlType, Timer};
use events::{Event, EventArgs};
use ui::UiInner;

/// A magic number to identify the NWG subclass that dispatches events
const EVENTS_DISPATCH_ID: UINT_PTR = 2465;

// WARNING! This WHOLE section (from parse_listbox_command to parse_command) will be replaced with the events overhaul in NWG BETA2

fn parse_listbox_command(id: u64, ncode: u32) -> Option<(u64, Event, EventArgs)> {
    use low::defs::{LBN_DBLCLK, LBN_KILLFOCUS, LBN_SELCHANGE, LBN_SETFOCUS};

    match ncode {
        LBN_SELCHANGE => Some((id, Event::SelectionChanged, EventArgs::None)),
        LBN_DBLCLK => Some((id, Event::DoubleClick, EventArgs::None)),
        LBN_SETFOCUS | LBN_KILLFOCUS => {
            Some((id, Event::Focus, EventArgs::Focus(ncode == LBN_SETFOCUS)))
        }
        _ => None,
    }
}

fn parse_button_command(id: u64, ncode: u32) -> Option<(u64, Event, EventArgs)> {
    use low::defs::{BN_CLICKED, BN_DBLCLK, BN_KILLFOCUS, BN_SETFOCUS};
    match ncode {
        BN_CLICKED => Some((id, Event::Click, EventArgs::None)),
        BN_DBLCLK => Some((id, Event::DoubleClick, EventArgs::None)),
        BN_SETFOCUS | BN_KILLFOCUS => {
            Some((id, Event::Focus, EventArgs::Focus(ncode == BN_SETFOCUS)))
        }
        _ => None,
    }
}

fn parse_edit_command(id: u64, ncode: u32) -> Option<(u64, Event, EventArgs)> {
    use low::defs::{EN_KILLFOCUS, EN_MAXTEXT, EN_SETFOCUS, EN_UPDATE};
    match ncode {
        EN_UPDATE => Some((id, Event::ValueChanged, EventArgs::None)),
        EN_MAXTEXT => Some((id, Event::LimitReached, EventArgs::None)),
        EN_SETFOCUS | EN_KILLFOCUS => {
            Some((id, Event::Focus, EventArgs::Focus(ncode == EN_SETFOCUS)))
        }
        _ => None,
    }
}

fn parse_static_command(id: u64, ncode: u32) -> Option<(u64, Event, EventArgs)> {
    use low::defs::{STN_CLICKED, STN_DBLCLK};
    match ncode {
        STN_CLICKED => Some((id, Event::Click, EventArgs::None)),
        STN_DBLCLK => Some((id, Event::DoubleClick, EventArgs::None)),
        _ => None,
    }
}

fn parse_datepicker_command(id: u64, ncode: u32) -> Option<(u64, Event, EventArgs)> {
    use winapi::DTN_CLOSEUP;
    match ncode {
        DTN_CLOSEUP => {
            // DTN_DATETIMECHANGE is sent twice so instead we catch DTN_CLOSEUP ¯\_(ツ)_/¯
            Some((id, Event::DateChanged, EventArgs::None))
        }
        _ => None,
    }
}

/**
  Parse the common controls notification passed through the `WM_COMMAND` message.
*/
#[inline(always)]
fn parse_notify(id: u64, control_type: ControlType, w: WPARAM) -> Option<(u64, Event, EventArgs)> {
    match control_type {
        ControlType::DatePicker => parse_datepicker_command(id, w as u32),
        _ => None,
    }
}

/**
  Parse the common controls notification passed through the `WM_COMMAND` message.
*/
#[inline(always)]
fn parse_command(id: u64, control_type: ControlType, w: WPARAM) -> Option<(u64, Event, EventArgs)> {
    use winapi::HIWORD;

    let ncode = HIWORD(w as DWORD) as u32;
    match control_type {
        ControlType::ListBox => parse_listbox_command(id, ncode),
        ControlType::Button => parse_button_command(id, ncode),
        ControlType::TextInput | ControlType::TextBox => parse_edit_command(id, ncode),
        ControlType::Label => parse_static_command(id, ncode),
        ControlType::DatePicker => parse_datepicker_command(id, ncode),
        _ => None,
    }
}

/**
  Proc that dispatches the NWG events
*/
#[allow(unused_variables)]
unsafe extern "system" fn process_events<ID: Hash + Clone + 'static>(
    hwnd: HWND,
    msg: UINT,
    w: WPARAM,
    l: LPARAM,
    id: UINT_PTR,
    data: DWORD_PTR,
) -> LRESULT {
    use comctl32::DefSubclassProc;
    use low::defs::{NWG_CUSTOM_MAX, NWG_CUSTOM_MIN};
    use low::menu_helper::get_menu_id;
    use user32::GetClientRect;
    use winapi::{
        c_int, HIWORD, LOWORD, NMHDR, RECT, UNICODE_NOCHAR, WM_CHAR, WM_CLOSE, WM_COMMAND,
        WM_EXITSIZEMOVE, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN,
        WM_MBUTTONUP, WM_MENUCOMMAND, WM_MOVE, WM_NOTIFY, WM_PAINT, WM_RBUTTONDOWN, WM_RBUTTONUP,
        WM_SIZE, WM_SIZING, WM_TIMER, WM_UNICHAR,
    };

    let inner: &mut UiInner<ID> = mem::transmute(data);
    let inner_id: u64;

    let callback_data = match msg {
        WM_PAINT => {
            inner_id = inner
                .inner_id_from_handle(&AnyHandle::HWND(hwnd))
                .expect("Could not match system handle to ui control (msg: WM_PAINT)");
            Some((inner_id, Event::Paint, EventArgs::None))
        }
        WM_COMMAND => {
            if l == 0 {
                None
            } else {
                // Somehow, WM_COMMAND messages get sent while freeing and so inner_id_from_handle can fail...
                let nhandle: HWND = mem::transmute(l);
                if let Some(id) = inner.inner_id_from_handle(&AnyHandle::HWND(nhandle)) {
                    let control_type = (&mut *inner
                        .controls
                        .get(&id)
                        .expect("Could not find a control with with the specified type ID")
                        .as_ptr())
                        .control_type();
                    parse_command(id, control_type, w)
                } else {
                    None
                }
            }
        }
        WM_NOTIFY => {
            // WM_NOTIFY is the new WM_COMMAND for the new windows controls
            let nmdr: &NMHDR = mem::transmute(l);
            if let Some(id) = inner.inner_id_from_handle(&AnyHandle::HWND(nmdr.hwndFrom)) {
                let control_type = (&mut *inner
                    .controls
                    .get(&id)
                    .expect("Could not find a control with with the specified type ID")
                    .as_ptr())
                    .control_type();
                parse_notify(id, control_type, nmdr.code as WPARAM)
            } else {
                None
            }
        }
        WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP => {
            inner_id = inner.inner_id_from_handle( &AnyHandle::HWND(hwnd) ).expect("Could not match system handle to ui control (msg: WM_LBUTTONUP | WM_RBUTTONUP  | WM_MBUTTONUP)");
            Some((inner_id, Event::MouseUp, parse_mouse_click(msg, l)))
        }
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN => {
            inner_id = inner.inner_id_from_handle( &AnyHandle::HWND(hwnd) ).expect("Could not match system handle to ui control (msg: WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN)");
            Some((inner_id, Event::MouseDown, parse_mouse_click(msg, l)))
        }
        WM_KEYDOWN | WM_KEYUP => {
            inner_id = inner
                .inner_id_from_handle(&AnyHandle::HWND(hwnd))
                .expect("Could not match system handle to ui control (msg: WM_KEYDOWN | WM_KEYUP)");
            let evt = if msg == WM_KEYDOWN {
                Event::KeyDown
            } else {
                Event::KeyUp
            };
            Some((inner_id, evt, EventArgs::Key(w as u32)))
        }
        WM_MENUCOMMAND => {
            let parent_menu: HMENU = mem::transmute(l);
            let handle = AnyHandle::HMENU_ITEM(parent_menu, get_menu_id(parent_menu, w as c_int));

            // Custom controls might have their own way to handle the message
            if let Some(inner_id) = inner.inner_id_from_handle(&handle) {
                Some((inner_id, Event::Triggered, EventArgs::None))
            } else {
                None
            }
        }
        WM_UNICHAR | WM_CHAR => {
            inner_id = inner
                .inner_id_from_handle(&AnyHandle::HWND(hwnd))
                .expect("Could not match system handle to ui control (msg: WM_UNICHAR | WM_CHAR)");
            if w == UNICODE_NOCHAR {
                return 1;
            }
            if let Some(c) = ::std::char::from_u32(w as u32) {
                Some((inner_id, Event::Char, EventArgs::Char(c)))
            } else {
                None
            }
        }
        WM_TIMER => {
            let handle = AnyHandle::Custom(TypeId::of::<Timer>(), w as usize);

            // Here I assume WM_TIMER will only be sent by built-in timers. Using a user event might be a better idea.
            // Custom controls might have their own way to handle the message
            if let Some(inner_id) = inner.inner_id_from_handle(&handle) {
                let timer: &mut Box<Timer> =
                    mem::transmute(inner.controls.get(&inner_id).unwrap().as_ptr());
                Some((inner_id, Event::Tick, EventArgs::Tick(timer.elapsed())))
            } else {
                None
            }
        }
        WM_MOVE => {
            inner_id = inner
                .inner_id_from_handle(&AnyHandle::HWND(hwnd))
                .expect("Could not match system handle to ui control (msg: WM_MOVE)");
            let (x, y) = (LOWORD(l as u32), HIWORD(l as u32));
            Some((
                inner_id,
                Event::Moved,
                EventArgs::Position(x as i32, y as i32),
            ))
        }
        WM_SIZING | WM_SIZE => {
            inner_id = inner
                .inner_id_from_handle(&AnyHandle::HWND(hwnd))
                .expect("Could not match system handle to ui control (msg: WM_SIZING)");
            let mut r: RECT = mem::uninitialized();
            GetClientRect(hwnd, &mut r);
            let w: u32 = (r.right - r.left) as u32;
            let h: u32 = (r.bottom - r.top) as u32;
            Some((inner_id, Event::Resized, EventArgs::Size(w, h)))
        }
        WM_EXITSIZEMOVE => {
            inner_id = inner
                .inner_id_from_handle(&AnyHandle::HWND(hwnd))
                .expect("Could not match system handle to ui control (msg: WM_SIZING)");
            let mut r: RECT = mem::uninitialized();
            GetClientRect(hwnd, &mut r);
            let w: u32 = (r.right - r.left) as u32;
            let h: u32 = (r.bottom - r.top) as u32;
            Some((inner_id, Event::Resized, EventArgs::Size(w, h)))
        }
        WM_CLOSE => {
            inner_id = inner
                .inner_id_from_handle(&AnyHandle::HWND(hwnd))
                .expect("Could not match system handle to ui control (msg: WM_CLOSE)");
            Some((inner_id, Event::Closed, EventArgs::None))
        }
        _ => None,
    };

    if let Some((inner_id, evt, params)) = callback_data {
        inner.trigger(inner_id, evt, params);
    }

    // Trigger a raw event
    if msg < NWG_CUSTOM_MIN || msg > NWG_CUSTOM_MAX {
        if let Some(inner_id) = inner.inner_id_from_handle(&AnyHandle::HWND(hwnd)) {
            inner.trigger(
                inner_id,
                Event::Raw,
                EventArgs::Raw(msg, w as usize, l as usize),
            );
        }
    }

    DefSubclassProc(hwnd, msg, w, l)
}

/**
    Add a subclass that dispatches the system event to the application callbacks to a window control.
*/
pub fn hook_window_events<ID: Hash + Clone + 'static>(uiinner: &mut UiInner<ID>, handle: HWND) {
    unsafe {
        use comctl32::SetWindowSubclass;

        // While definitely questionable in term of safety, the reference to the UiInner is actually (always)
        // a raw pointer belonging to a Ui. Also, when the Ui goes out of scope, every window control
        // gets destroyed BEFORE the UiInner, this guarantees that uinner lives long enough.
        let ui_inner_raw: *mut UiInner<ID> = uiinner as *mut UiInner<ID>;
        SetWindowSubclass(
            handle,
            Some(process_events::<ID>),
            EVENTS_DISPATCH_ID,
            mem::transmute(ui_inner_raw),
        );
    }
}

/**
  Remove a subclass and free the associated data
*/
pub fn unhook_window_events<ID: Hash + Clone + 'static>(handle: HWND) {
    unsafe {
        use comctl32::{GetWindowSubclass, RemoveWindowSubclass};
        use winapi::TRUE;

        let mut data: DWORD_PTR = 0;
        if GetWindowSubclass(
            handle,
            Some(process_events::<ID>),
            EVENTS_DISPATCH_ID,
            &mut data,
        ) == TRUE
        {
            RemoveWindowSubclass(handle, Some(process_events::<ID>), EVENTS_DISPATCH_ID);
        }
    }
}

/**
  Check if a window is hooked by nwg. If it is, return its ID, if not return None
*/
pub unsafe fn window_id<ID: Clone + Hash>(
    handle: HWND,
    inner_ref: *mut UiInner<ID>,
) -> Option<u64> {
    use comctl32::GetWindowSubclass;
    use winapi::TRUE;

    let mut data: DWORD_PTR = 0;
    if GetWindowSubclass(
        handle,
        Some(process_events::<ID>),
        EVENTS_DISPATCH_ID,
        &mut data,
    ) == TRUE
    {
        let data: *mut UiInner<ID> = mem::transmute(data);
        if data == inner_ref {
            (&*data).inner_id_from_handle(&AnyHandle::HWND(handle))
        } else {
            None
        }
    } else {
        None
    }
}

/**
    Dispatch the messages waiting the the system message queue to the associated Uis. This includes NWG custom messages.

    Return once a quit event was received.
*/
#[inline(always)]
pub unsafe fn dispatch_events() {
    use user32::{DispatchMessageW, GetMessageW, TranslateMessage};
    use winapi::MSG;

    let mut msg: MSG = mem::uninitialized();
    while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
        // TODO dispatch events sent from other thread / other processes ( after first stable release )
    }
}

/**
    Send a WM_QUIT to the system queue. Breaks the dispatch_events loop.
*/
#[inline(always)]
pub unsafe fn exit() {
    use user32::PostMessageW;
    use winapi::WM_QUIT;

    PostMessageW(ptr::null_mut(), WM_QUIT, 0, 0);
}

fn parse_mouse_click(msg: UINT, l: LPARAM) -> EventArgs {
    use defs::MouseButton;
    use winapi::{
        GET_X_LPARAM, GET_Y_LPARAM, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
        WM_RBUTTONDOWN, WM_RBUTTONUP,
    };

    let btn = match msg {
        WM_LBUTTONUP | WM_LBUTTONDOWN => MouseButton::Left,
        WM_RBUTTONUP | WM_RBUTTONDOWN => MouseButton::Right,
        WM_MBUTTONUP | WM_MBUTTONDOWN => MouseButton::Middle,
        _ => MouseButton::Left,
    };

    let x = GET_X_LPARAM(l) as i32;
    let y = GET_Y_LPARAM(l) as i32;

    EventArgs::MouseClick {
        btn: btn,
        pos: (x, y),
    }
}
