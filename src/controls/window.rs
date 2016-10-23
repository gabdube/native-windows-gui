/*!
    A blank custom control.
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

use std::hash::Hash;
use std::mem;
use std::ptr;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text, show_message,
  get_window_pos, set_window_pos, get_window_size, set_window_size,
  get_window_enabled, set_window_enabled, get_window_visibility,
  set_window_visibility, get_window_display, set_window_display, get_window_children,
  get_window_descendant, get_control_type, close_window, to_utf16, set_handle_data_off,
  get_handle_data_off, destroy_control, free_handle_data_off};
use actions::{Action, ActionReturn};
use events::Event;
use constants::ControlType;

use winapi::{HWND, HINSTANCE, WNDCLASSEXW, UINT, CS_HREDRAW, CS_VREDRAW, IDC_ARROW,
  COLOR_WINDOW, WPARAM, LPARAM, LRESULT, WM_CLOSE, WM_CREATE};

use user32::{RegisterClassExW, LoadCursorW, PostQuitMessage, DefWindowProcW};
use kernel32::{GetLastError, GetModuleHandleW};

const CLASS_NAME: &'static str = "RustWindow";

/**
    Configuration properties to create a window

    * caption: Window title (in the upper bar)
    * size: Window size (width, height) in pixels
    * position: Starting position (x, y) of the window 
    * visible: If the window should be visible from the start
    * resizable: If the window should be resizable by the user
*/
pub struct Window {
    pub caption: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub visible: bool,
    pub resizable: bool,
    pub exit_on_close: bool
}

struct PrivateWindowData {
    pub exit_on_close: bool
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for Window {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> { 
        if unsafe { !register_custom_class::<ID>() } {
            return Err(());
        }

        let base = WindowBase::<ID> {
            text: self.caption.clone(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: self.visible,
            resizable: self.resizable,
            extra_style: 0,
            class: CLASS_NAME.to_string(),
            parent: None
        };

        if let Ok(handle) = unsafe{ create_base::<ID>(ui, base) } {
            unsafe{ set_handle_data_off(handle, PrivateWindowData{exit_on_close: self.exit_on_close}, 0); }
            Ok(handle)
        } else {
            Err(())
        } 
    }

    fn supported_events(&self) -> Vec<Event> {
        vec![Event::MouseUp, Event::MouseDown, Event::Focus, Event::Removed, Event::Resize, Event::Move,
            Event::KeyDown, Event::KeyUp]
    }

    fn evaluator(&self) -> ::ActionEvaluator<ID> {
        Box::new( |ui, id, handle, action| {
            match action {
                Action::Message(p) => show_message(handle, *p),
                Action::GetText => get_window_text(handle),
                Action::SetText(t) => set_window_text(handle, *t),
                Action::GetPosition => get_window_pos(handle, false),
                Action::SetPosition(x, y) => set_window_pos(handle, x, y),
                Action::GetSize => get_window_size(handle),
                Action::SetSize(w, h) => set_window_size(handle, w, h),
                Action::GetChildren => get_window_children(handle),
                Action::GetDescendants => get_window_descendant(handle),
                Action::GetEnabled => get_window_enabled(handle),
                Action::SetEnabled(e) => set_window_enabled(handle, e),
                Action::GetVisibility => get_window_visibility(handle),
                Action::SetVisibility(v) => set_window_visibility(handle, v),
                Action::GetControlType => get_control_type(handle),

                Action::GetWindowDisplay => get_window_display(handle),
                Action::SetWindowDisplay(d) => set_window_display(handle, d),
                
                Action::Close => close_window(handle),

                _ => ActionReturn::NotSupported
            }            
        })
    }

    fn control_type(&self) -> ControlType {
        ControlType::Window
    }

}


/**
    Register a new window class. Return true if the class already exists 
    or the creation was successful and false if it failed.
*/
unsafe fn register_custom_class<ID: Eq+Clone+Hash>() -> bool {
    let name = to_utf16(CLASS_NAME.to_string());
    let hmod = GetModuleHandleW(ptr::null());
    let class =
        WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<ID>), 
            cbClsExtra: 0,
            cbWndExtra: mem::size_of::<usize>() as i32,
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
    Handle the WM_CLOSE event on a window. If exit_on_close is set,
    nwg will free all its resources by itself, but if exit_on_close is not
    set, the window must be manually destroyed.
*/
unsafe fn handle_close<ID: Eq+Clone+Hash>(handle: HWND) -> LRESULT {
    if let Some(d) = get_handle_data_off::<PrivateWindowData>(handle, 0) {
        free_handle_data_off::<PrivateWindowData>(handle, 0);
        if d.exit_on_close { 
            PostQuitMessage(0); 
        } else {
            destroy_control::<ID>(handle).unwrap();
        }
    }

    0 
} 

/**
    Custom window procedure for Window types.
*/
unsafe extern "system" fn wndproc<ID: Eq+Clone+Hash>(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => 0,
        WM_CLOSE => handle_close::<ID>(hwnd),
        _ =>  DefWindowProcW(hwnd, msg, w, l)
    }
}