/*!
    Window control definition
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

use winapi::HWND;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::{Event, Destroyed, Moved, Resized, Char, Closed, KeyDown, KeyUp, MouseUp, MouseDown};

/// System class identifier
const WINDOW_CLASS_NAME: &'static str = "NWG_BUILTIN_WINDOW";

/**
    A template that will create a window.

    Events:  
    `Destroyed, KeyDown, KeyUp, Char, Closed, MouseDown, MouseUp, Moved, Resized, Any`  

    Members:  
      • `title` : The title of the window (in the title bar)  
      • `position` : Starting posiion of the window after it is created  
      • `size` : Starting size of the window after it is created  
      • `resizable` : If the user can resize the window or not  
      • `visible` : If the user can see the window or not  
      • `disabled` : If the window is enabled or not. A disabled window do not process events  
      • `exit_on_close` : If NWG should break the event processing loop when this window is closed  
*/
#[derive(Clone)]
pub struct WindowT<S: Clone+Into<String>> {
    pub title: S,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub resizable: bool,
    pub visible: bool,
    pub disabled: bool,
    pub exit_on_close: bool
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for WindowT<S> {
    fn type_id(&self) -> TypeId { TypeId::of::<Window>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, KeyDown, KeyUp, Char, Closed, MouseDown, MouseUp, Moved, Resized, Event::Any]
    }

    #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        unsafe{
            if let Err(e) = build_sysclass() { return Err(e); }
            match build_window(&self) {
                Ok(h) => { Ok( Box::new(Window{handle: h}) as Box<Control> ) },
                Err(e) => Err(e)
            }
        } // unsafe
    }
}

/**
    A window control.
*/
#[allow(dead_code)]
pub struct Window {
    handle: HWND,
}

impl Window {
    /**
        Close the window as if the user clicked on the X button. This do **NOT** remove the window from the ui,
        it only set it hidden. In order to also destroy the window, add an unpack statement on the **Closed** event.
    */
    pub fn close(&self) {
        use user32::PostMessageW;
        use winapi::WM_CLOSE;

        unsafe{ PostMessageW(self.handle, WM_CLOSE, 0, 0) };
    }

    /// Activate the window and set it above the other windows
    pub fn activate(&self) { unsafe{ 
        use user32::SetForegroundWindow;
        SetForegroundWindow(self.handle); 
    } }

    pub fn get_title(&self) -> String { unsafe{ ::low::window_helper::get_window_text(self.handle) } }
    pub fn set_title<'a>(&self, text: &'a str) { unsafe{ ::low::window_helper::set_window_text(self.handle, text); } }
    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
}

impl Control for Window {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType {
        ControlType::Window 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}


/*
    Private unsafe control methods
*/

use winapi::{UINT, WPARAM, LPARAM, LRESULT};

#[allow(unused_variables)]
unsafe extern "system" fn window_sysproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::{WM_CREATE, WM_CLOSE, GWL_USERDATA};
    use user32::{DefWindowProcW, PostQuitMessage, ShowWindow};
    use low::window_helper::get_window_long;

    let handled = match msg {
        WM_CREATE => true,
        WM_CLOSE => {
            ShowWindow(hwnd, 0);

            let exit_on_close = get_window_long(hwnd, GWL_USERDATA) & 0x01 == 1;
            if exit_on_close {
                PostQuitMessage(0);
            }
            true
        }
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}

#[inline(always)]
unsafe fn build_sysclass() -> Result<(), Error> {
    use low::window_helper::{SysclassParams, build_sysclass};
    let params = SysclassParams { 
        class_name: WINDOW_CLASS_NAME,
        sysproc: Some(window_sysproc),
        background: None, style: None
    };
    
    if let Err(e) = build_sysclass(params) {
        Err(Error::System(e))
    } else {
        Ok(())
    }
}

#[inline(always)]
unsafe fn build_window<S: Clone+Into<String>>(t: &WindowT<S>) -> Result<HWND, Error> {
    use low::window_helper::{WindowParams, build_window, set_window_long};
    use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_OVERLAPPEDWINDOW, WS_CAPTION, WS_OVERLAPPED, WS_MINIMIZEBOX,
      WS_MAXIMIZEBOX, WS_SYSMENU, GWL_USERDATA, WS_CLIPCHILDREN};

    let fixed_window: DWORD = WS_CLIPCHILDREN| WS_SYSMENU | WS_CAPTION | WS_OVERLAPPED | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
    let flags: DWORD = 
    if t.visible    { WS_VISIBLE }   else { 0 } |
    if t.disabled   { WS_DISABLED }  else { 0 } |
    if !t.resizable { fixed_window } else { WS_OVERLAPPEDWINDOW } ;

    let params = WindowParams {
        title: t.title.clone().into(),
        class_name: WINDOW_CLASS_NAME,
        position: t.position.clone(),
        size: t.size.clone(),
        flags: flags,
        ex_flags: None,
        parent: ::std::ptr::null_mut()
    };

    match build_window(params) {
        Ok(h) => {
            set_window_long(h, GWL_USERDATA, t.exit_on_close as usize);
            Ok(h)
        },
        Err(e) => Err(Error::System(e))
    }
}