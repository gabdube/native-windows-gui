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
//use std::any::Any;

use winapi::{HWND, UINT, WPARAM, LPARAM, UINT_PTR, DWORD_PTR, LRESULT};

use ui::UiInner;

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
  use low::defs::{NWG_UNPACK_INDIRECT};
  use args::UnpackArgs;
  
  let handled = match msg {
    NWG_UNPACK_INDIRECT => {
      let ui: &mut UiInnerWithId<ID> = mem::transmute(data);
      let (inner, id) = (ui.inner, ui.id);
      mem::forget(ui); // Forget ui because it will point to freed memory after the unpack result.
      (&mut *inner).unpack( UnpackArgs{id: id} ); 
      true
    },
    _ => { false }
  }; 

  if handled {
    0
  } else {
    DefSubclassProc(hwnd, msg, w, l)
  }
}

/**
    Add a subclass that dispatches the system event to the application callbacks to a window control.
*/
pub fn hook_window_events<ID: Hash+Clone+'static>(uiinner: &mut UiInner<ID>, id: u64, handle: HWND) { unsafe {
  use comctl32::SetWindowSubclass;

  // While definitely questionable in term of safeness, the reference to the UiInner is actually
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