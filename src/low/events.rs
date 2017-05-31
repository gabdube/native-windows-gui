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
#![allow(non_upper_case_globals, unused_variables)]

use std::mem;
use std::ptr;
use std::hash::{Hash, Hasher};

use winapi::{HWND, UINT, WPARAM, LPARAM, UINT_PTR, DWORD_PTR, LRESULT, WORD};

use winapi::{WM_MOVE, WM_SIZING, WM_SIZE, WM_EXITSIZEMOVE, WM_PAINT, WM_UNICHAR, WM_CHAR,
  WM_CLOSE, WM_LBUTTONUP, WM_RBUTTONUP, WM_MBUTTONUP, WM_LBUTTONDOWN, WM_RBUTTONDOWN,
  WM_MBUTTONDOWN, WM_KEYDOWN, WM_KEYUP, BN_CLICKED, BN_DBLCLK, BN_SETFOCUS, BN_KILLFOCUS,
  DTN_CLOSEUP, WM_COMMAND, WM_NOTIFY, WM_USER};

use ui::{UiInner, EventHandlerCollection};
use events::{Event, EventArgs, EventParam};
use controls::{AnyHandle};
use low::defs::{NWG_DESTROY, CBN_SELCHANGE, CBN_KILLFOCUS, CBN_SETFOCUS, STN_CLICKED, STN_DBLCLK,
  LBN_SELCHANGE, LBN_DBLCLK, LBN_SETFOCUS, LBN_KILLFOCUS, EN_SETFOCUS, EN_KILLFOCUS, EN_UPDATE,
  EN_MAXTEXT};

/// A magic number to identify the NWG subclass that dispatches events
const EVENTS_DISPATCH_ID: UINT_PTR = 2465;


// Definition of common system events
pub const Destroyed: Event = Event::System(NWG_DESTROY, &system_event_unpack_no_args);
pub const Paint: Event = Event::System(WM_PAINT, &system_event_unpack_no_args);
pub const Closed: Event = Event::System(WM_CLOSE, &system_event_unpack_no_args);
pub const Moved: Event = Event::System(WM_MOVE, &unpack_move);
pub const KeyDown: Event = Event::System(WM_KEYDOWN, &unpack_key);
pub const KeyUp: Event = Event::System(WM_KEYUP, &unpack_key);
pub const Resized: Event = Event::SystemGroup(&[WM_SIZING, WM_SIZE, WM_EXITSIZEMOVE], &unpack_size);
pub const Char: Event = Event::SystemGroup(&[WM_UNICHAR, WM_CHAR], &unpack_char);
pub const MouseUp: Event = Event::SystemGroup(&[WM_LBUTTONUP, WM_RBUTTONUP, WM_MBUTTONUP], &unpack_mouseclick);
pub const MouseDown: Event = Event::SystemGroup(&[WM_LBUTTONDOWN, WM_RBUTTONDOWN, WM_MBUTTONDOWN], &unpack_mouseclick);

// Button events
pub const BtnClick: Event = Event::Command(BN_CLICKED, &command_event_unpack_no_args);
pub const BtnDoubleClick: Event = Event::Command(BN_DBLCLK, &command_event_unpack_no_args);
pub const BtnFocus: Event = Event::CommandGroup(&[BN_SETFOCUS, BN_KILLFOCUS], &unpack_btn_focus);

// Combobox events
pub const CbnFocus: Event = Event::CommandGroup(&[CBN_SETFOCUS, CBN_KILLFOCUS], &unpack_cbn_focus);
pub const CbnSelectionChanged: Event = Event::Command(CBN_SELCHANGE, &command_event_unpack_no_args);

// Static events
pub const StnClick: Event = Event::Command(STN_CLICKED, &command_event_unpack_no_args);
pub const StnDoubleClick: Event = Event::Command(STN_DBLCLK, &command_event_unpack_no_args);

// Datepicker events
pub const DateChanged: Event = Event::Notify(DTN_CLOSEUP, &notify_event_unpack_no_args);

// Listbox events
pub const LbnSelectionChanged: Event = Event::Command(LBN_SELCHANGE, &command_event_unpack_no_args);
pub const LbnDoubleClick: Event = Event::Command(LBN_DBLCLK, &command_event_unpack_no_args);
pub const LbnFocus: Event = Event::CommandGroup(&[LBN_SETFOCUS, LBN_KILLFOCUS], &unpack_lbn_focus);

// Textedit events
pub const EnValueChanged: Event = Event::Command(EN_UPDATE, &command_event_unpack_no_args);
pub const EnLimit: Event = Event::Command(EN_MAXTEXT, &command_event_unpack_no_args);
pub const EnFocus: Event = Event::CommandGroup(&[EN_SETFOCUS, EN_KILLFOCUS], &unpack_en_focus);

// Event unpackers for events that have no arguments
pub fn system_event_unpack_no_args(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> Option<EventArgs> { Some(EventArgs::None) }
pub fn command_event_unpack_no_args(hwnd: HWND, ncode: WORD) -> Option<EventArgs> { Some(EventArgs::None) }
pub fn notify_event_unpack_no_args(hwnd: HWND) -> Option<EventArgs> { Some(EventArgs::None) }

// Event unpackers for the events defined above
fn unpack_move(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> Option<EventArgs> {
    use winapi::{LOWORD, HIWORD};
    
    let (x, y) = (LOWORD(l as u32), HIWORD(l as u32));
    Some(EventArgs::Position(x as i32, y as i32))
}

fn unpack_size(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> Option<EventArgs> {
    use winapi::RECT;
    use user32::GetClientRect;

    let mut r: RECT = unsafe{ mem::uninitialized() };

    unsafe{ GetClientRect(hwnd, &mut r); }
    let w: u32 = (r.right-r.left) as u32;
    let h: u32 = (r.bottom-r.top) as u32;

    Some(EventArgs::Size(w, h))
}

fn unpack_char(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> Option<EventArgs> {
    use winapi::UNICODE_NOCHAR;

    if w == UNICODE_NOCHAR { 
      return None; 
    } 

    if let Some(c) = ::std::char::from_u32(w as u32) {
      Some( EventArgs::Char( c ) )
    } else {
      None
    }
}

fn unpack_mouseclick(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> Option<EventArgs> {
  use defs::MouseButton;
  use winapi::{GET_X_LPARAM, GET_Y_LPARAM};

  let btn = match msg {
    WM_LBUTTONUP | WM_LBUTTONDOWN => MouseButton::Left,
    WM_RBUTTONUP | WM_RBUTTONDOWN => MouseButton::Right,
    WM_MBUTTONUP | WM_MBUTTONDOWN => MouseButton::Middle,
    _ => MouseButton::Left
  };

  let x = GET_X_LPARAM(l) as i32; 
  let y = GET_Y_LPARAM(l) as i32;

  Some(EventArgs::MouseClick{btn: btn, pos: (x, y)})
}

fn unpack_key(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> Option<EventArgs> {
   Some(EventArgs::Key(w as u32))
}

fn unpack_btn_focus(hwnd: HWND, ncode: WORD) -> Option<EventArgs> {
    Some(EventArgs::Focus(ncode==BN_SETFOCUS))
}

fn unpack_cbn_focus(hwnd: HWND, ncode: WORD) -> Option<EventArgs> {
    Some(EventArgs::Focus(ncode==CBN_SETFOCUS))
}

fn unpack_lbn_focus(hwnd: HWND, ncode: WORD) -> Option<EventArgs> {
    Some(EventArgs::Focus(ncode==LBN_SETFOCUS))
}

fn unpack_en_focus(hwnd: HWND, ncode: WORD) -> Option<EventArgs> {
    Some(EventArgs::Focus(ncode==EN_SETFOCUS))
}

/**
  Proc that dispatches the NWG events
*/
#[allow(unused_variables)]
unsafe extern "system" fn process_events<ID: Hash+Clone+'static>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT {
    use comctl32::DefSubclassProc;
    use low::defs::{NWG_CUSTOM_MIN, NWG_CUSTOM_MAX};

    let inner: &mut UiInner<ID> = mem::transmute(data);
    let inner_id: u64;

    if let Some(events) = inner.event_handlers(msg as u32) {
        let handle = build_handle(hwnd, msg, w, l);
        let params = build_events_params(&handle, hwnd, msg, w, l);
        if let Some(inner_id) = inner.inner_id_from_handle( &handle ) {
            println!("TEST");
            for event in events.iter() {
                if let Some(args) = build_args(event, &params) {
                    inner.trigger(inner_id, *event, args);
                }
            }
        }
    }
    
    // Trigger the `Any` event 
    if msg < NWG_CUSTOM_MIN || msg > NWG_CUSTOM_MAX {
      if let Some(inner_id) = inner.inner_id_from_handle( &AnyHandle::HWND(hwnd) ) {
        inner.trigger(inner_id, Event::Any, EventArgs::Raw(msg, w, l));
      }
    }

    DefSubclassProc(hwnd, msg, w, l)
}

#[inline(always)]
fn build_handle(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> AnyHandle {
    use winapi::NMHDR;

    if msg == WM_COMMAND {
        AnyHandle::HWND( unsafe{ mem::transmute(l) } )
    } else if msg == WM_NOTIFY {
        let nmdr: &NMHDR = unsafe{ mem::transmute(l) };
        AnyHandle::HWND(nmdr.hwndFrom)
    } else if msg < WM_USER {
        AnyHandle::HWND(hwnd)
    } else {
        AnyHandle::HWND(hwnd)
    }
}

#[inline(always)]
fn build_events_params(handle: &AnyHandle, hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> EventParam {
    use winapi::{HIWORD, DWORD};

    if msg == WM_COMMAND {
        let ncode: WORD = HIWORD(w as DWORD);
        match handle {
            &AnyHandle::HWND(hwnd) => EventParam::CommandParam(hwnd, ncode),
            _ => unreachable!()
        }
    } else if msg == WM_NOTIFY {
        match handle {
            &AnyHandle::HWND(hwnd) => EventParam::NotifyParam(hwnd),
            _ => unreachable!()
        }
    } else if msg < WM_USER {
        match handle { 
            &AnyHandle::HWND(hwnd) => EventParam::SystemParam(hwnd, msg, w, l),
            _ => EventParam::None
        }
    } else {
        EventParam::None
    }
}

#[inline(always)]
fn build_args(event: &Event, params: &EventParam) -> Option<EventArgs> {
    match params {
        &EventParam::SystemParam(hwnd, msg, w, l) => {
            match event {
                &Event::System(_, fnptr) | &Event::SystemGroup(_, fnptr) => (fnptr)(hwnd, msg, w, l),
                _ => { unreachable!(); }
            }
        },
        &EventParam::CommandParam(hwnd, ncode) => {
            match event {
                &Event::Command(_, fnptr) | &Event::CommandGroup(_, fnptr) => (fnptr)(hwnd, ncode),
                _ => { unreachable!(); }
            }
        }
        &EventParam::NotifyParam(hwnd) => {
            match event {
                &Event::Notify(_, fnptr) => (fnptr)(hwnd),
                _ => { unreachable!(); }
            }
        }
        &EventParam::None => None
    }
}

/**
    Add a subclass that dispatches the system event to the application callbacks to a window control.
*/
pub fn hook_window_events<ID: Hash+Clone+'static>(uiinner: &mut UiInner<ID>, handle: HWND) { unsafe {
  use comctl32::SetWindowSubclass;

  // While definitely questionable in term of safety, the reference to the UiInner is actually (always)
  // a raw pointer belonging to a Ui. Also, when the Ui goes out of scope, every window control
  // gets destroyed BEFORE the UiInner, this guarantees that uinner lives long enough.
  let ui_inner_raw: *mut UiInner<ID> = uiinner as *mut UiInner<ID>;
  SetWindowSubclass(handle, Some(process_events::<ID>), EVENTS_DISPATCH_ID, mem::transmute(ui_inner_raw));
}}

/**
  Remove a subclass and free the associated data
*/
pub fn unhook_window_events<ID: Hash+Clone+'static>(handle: HWND) { unsafe {
  use comctl32::{RemoveWindowSubclass, GetWindowSubclass};
  use winapi::{TRUE, DWORD_PTR};

  let mut data: DWORD_PTR = 0;
  if GetWindowSubclass(handle, Some(process_events::<ID>), EVENTS_DISPATCH_ID, &mut data) == TRUE {
    RemoveWindowSubclass(handle, Some(process_events::<ID>), EVENTS_DISPATCH_ID);
  }
}}

/**
  Check if a window is hooked by nwg. If it is, return its ID, if not return None
*/
pub unsafe fn window_id<ID: Clone+Hash>(handle: HWND, inner_ref: *mut UiInner<ID>) -> Option<u64> {
  use comctl32::GetWindowSubclass;
  use winapi::{TRUE, DWORD_PTR};

  let mut data: DWORD_PTR = 0;
  if GetWindowSubclass(handle, Some(process_events::<ID>), EVENTS_DISPATCH_ID, &mut data) == TRUE {
    let data: *mut UiInner<ID> = mem::transmute(data);
    if data == inner_ref {
      (&*data).inner_id_from_handle( &AnyHandle::HWND(handle) )
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
  use winapi::MSG;
  use user32::{GetMessageW, TranslateMessage, DispatchMessageW};

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

/**
    Hash the function pointer of an events. Assumes the pointer as a size of [usize; 2].
    There's a test that check this.
*/
#[inline(always)]
pub fn hash_fn_ptr<T: Sized, H: Hasher>(fnptr: &T, state: &mut H) {
    unsafe{
        let ptr_v: [usize; 2] = mem::transmute_copy(fnptr);
        ptr_v.hash(state);
    }
}