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

use std::mem;
use std::ptr;
use std::hash::Hash;

use winapi::{HWND, UINT, WPARAM, LPARAM, UINT_PTR, DWORD_PTR, LRESULT};

use ui::UiInner;
use events::EventArgs;

/// A magic number to identify the NWG subclass that dispatches events
const EVENTS_DISPATCH_ID: UINT_PTR = 2465;

/// A structure saved in a subclass to retrieve the ui in the callback
struct UiInnerWithId<ID: Hash+Clone+'static> {
  pub inner: *mut UiInner<ID>,
  pub id: u64,
}


/**
  Proc that dispatches the NWG events
*/
#[allow(unused_variables)]
unsafe extern "system" fn process_events<ID: Hash+Clone+'static>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT {
  use comctl32::{DefSubclassProc};
  use winapi::{WM_KEYDOWN, WM_KEYUP, WM_UNICHAR, WM_CHAR, UNICODE_NOCHAR, WM_MENUCOMMAND, WM_CLOSE, WM_LBUTTONUP, WM_LBUTTONDOWN, 
    WM_RBUTTONUP, WM_RBUTTONDOWN, WM_MBUTTONUP, WM_MBUTTONDOWN, c_int};
  use events::{Event, EventArgs};

  match msg {
    WM_LBUTTONUP | WM_RBUTTONUP  | WM_MBUTTONUP => {
      let ui: &mut UiInnerWithId<ID> = mem::transmute(data);
      let (inner, id) = (ui.inner, ui.id);

      (&mut *inner).trigger(id, Event::MouseUp, parse_mouse_click(msg, l));

      if msg == WM_LBUTTONUP {
        (&mut *inner).trigger(id, Event::Clicked, EventArgs::None);
      }
    },
    WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN => {
      let ui: &mut UiInnerWithId<ID> = mem::transmute(data);
      let (inner, id) = (ui.inner, ui.id);

      (&mut *inner).trigger(id, Event::MouseDown, parse_mouse_click(msg, l));
    },
    WM_KEYDOWN | WM_KEYUP => {
      let ui: &mut UiInnerWithId<ID> = mem::transmute(data);
      let (inner, id) = (ui.inner, ui.id);

      let evt = if msg == WM_KEYDOWN { Event::KeyDown } else { Event::KeyUp };

      (&mut *inner).trigger(id, evt, EventArgs::Key(w as u32));
    },
    WM_MENUCOMMAND => {
      let ui: &mut UiInnerWithId<ID> = mem::transmute(data);
      let inner = ui.inner;
      let id = ::low::menu_helper::get_menu_id( mem::transmute(l), w as c_int );

      (&mut *inner).trigger(id, Event::Clicked, EventArgs::None);
    },
    WM_UNICHAR | WM_CHAR => {
      let ui: &mut UiInnerWithId<ID> = mem::transmute(data);
      let (inner, id) = (ui.inner, ui.id);

      if w == UNICODE_NOCHAR {
        return 1;
      } 

      if let Some(c) = ::std::char::from_u32(w as u32) {
        (&mut *inner).trigger(id, Event::Char, EventArgs::Char( c ));
      }
    },
    WM_CLOSE => {
      let ui: &mut UiInnerWithId<ID> = mem::transmute(data);
      let (inner, id) = (ui.inner, ui.id);

      (&mut *inner).trigger(id, Event::Closed, EventArgs::None);
    },
    _ => { }
  }

  DefSubclassProc(hwnd, msg, w, l)
}

/**
    Add a subclass that dispatches the system event to the application callbacks to a window control.
*/
pub fn hook_window_events<ID: Hash+Clone+'static>(uiinner: &mut UiInner<ID>, id: u64, handle: HWND) { unsafe {
  use comctl32::SetWindowSubclass;

  // While definitely questionable in term of safeness, the reference to the UiInner is actually (always)
  // a raw pointer belonging to a Ui. Also, when the Ui goes out of scope, every window control
  // gets destroyed BEFORE the UiInner, this guarantees that uinner lives long enough.
  let ui_inner_raw: *mut UiInner<ID> = uiinner as *mut UiInner<ID>;
  let data: *mut UiInnerWithId<ID> = Box::into_raw(Box::new(UiInnerWithId{ inner: ui_inner_raw, id: id }));

  SetWindowSubclass(handle, Some(process_events::<ID>), EVENTS_DISPATCH_ID, mem::transmute(data));
}}

/**
  Remove a subclass and free the associated data
*/
pub fn unhook_window_events<ID: Hash+Clone+'static>(handle: HWND) { unsafe {
  use comctl32::{RemoveWindowSubclass, GetWindowSubclass};
  use winapi::{TRUE, DWORD_PTR};

  let mut data: DWORD_PTR = 0;
  if GetWindowSubclass(handle, Some(process_events::<ID>), EVENTS_DISPATCH_ID, &mut data) == TRUE {
    Box::from_raw(mem::transmute::<_, *mut UiInnerWithId<ID>>(data));
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
    let data: &mut UiInnerWithId<ID> = mem::transmute(data);
    if data.inner == inner_ref {
      Some(data.id)
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
  use winapi::{WM_LBUTTONUP, WM_LBUTTONDOWN, WM_RBUTTONUP, WM_RBUTTONDOWN, WM_MBUTTONUP, WM_MBUTTONDOWN, 
    GET_X_LPARAM, GET_Y_LPARAM};

  let btn = match msg {
    WM_LBUTTONUP | WM_LBUTTONDOWN => MouseButton::Left,
    WM_RBUTTONUP | WM_RBUTTONDOWN => MouseButton::Right,
    WM_MBUTTONUP | WM_MBUTTONDOWN => MouseButton::Middle,
    _ => MouseButton::Left
  };

  let x = GET_X_LPARAM(l) as i32; 
  let y = GET_Y_LPARAM(l) as i32;

  EventArgs::MouseClick{btn: btn, pos: (x, y)}
}