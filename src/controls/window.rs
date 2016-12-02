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

use winapi::HWND;

use controls::{Control, ControlT};
use error::Error;
use args::AnyHandle;

/// System class identifier
const WINDOW_CLASS_NAME: &'static str = "NWG_BUILTIN_WINDOW";

/**
    A template that will create a window.
*/
pub struct WindowT<S: Clone+Into<String>> {
    pub title: S,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub resizable: bool,
    pub visible: bool,
    pub disabled: bool,
    pub exit_on_close: bool
}

/**
    A window control.
*/
#[allow(dead_code)]
pub struct Window {
    handle: HWND
}

impl<S: Clone+Into<String>> ControlT for WindowT<S> {
    fn type_id(&self) -> TypeId { TypeId::of::<Window>() }

    fn build(&self) -> Result<Box<Control>, Error> {
        unsafe{
            if let Err(e) = build_sysclass() { return Err(e); }
            match build_window(&self) {
                Ok(h) => { Ok( Box::new(Window{handle: h}) as Box<Control> ) },
                Err(e) => Err(e)
            }
        } // unsafe
    }
}

impl Control for Window {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

}


/*
    Private unsafe control methods
*/

use winapi::{UINT, WPARAM, LPARAM, LRESULT};

#[allow(unused_variables)]
unsafe extern "system" fn window_sysproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::{WM_CREATE, WM_CLOSE};
    use user32::{DefWindowProcW, PostQuitMessage, DestroyWindow};
    use low::window_helper::{get_window_long};

    let handled = match msg {
        WM_CREATE => true,
        WM_CLOSE => {
            let exit_on_close = get_window_long(hwnd) == 1;
            if exit_on_close {
                DestroyWindow(hwnd);
                PostQuitMessage(0);
                true
            } else {
                // DestroyWindow is called by the default behaviour
                false
            }
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
    let params = SysclassParams{ class_name: WINDOW_CLASS_NAME, sysproc: Some(window_sysproc) };
    
    if let Err(e) = build_sysclass(params) {
        Err(Error::System(e))
    } else {
        Ok(())
    }
}

#[inline(always)]
unsafe fn build_window<S: Clone+Into<String>>(t: &WindowT<S>) -> Result<HWND, Error> {
    use low::window_helper::{WindowParams, build_window, set_window_long};
    use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_OVERLAPPEDWINDOW, WS_CAPTION, WS_OVERLAPPED, WS_MINIMIZEBOX, WS_MAXIMIZEBOX, WS_SYSMENU};

    let fixed_window: DWORD = WS_SYSMENU | WS_CAPTION | WS_OVERLAPPED | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
    let flags: DWORD = 
    if t.visible    { WS_VISIBLE }   else { 0 } |
    if t.disabled   { WS_DISABLED }  else { 0 } |
    if !t.resizable { fixed_window } else { WS_OVERLAPPEDWINDOW } ;

    let params = WindowParams {
        title: t.title.clone().into(),
        class_name: WINDOW_CLASS_NAME,
        position: t.position.clone(),
        size: t.size.clone(),
        flags: flags
    };

    match build_window(params) {
        Ok(h) => {
            set_window_long(h, t.exit_on_close as usize);
            Ok(h)
        },
        Err(e) => Err(Error::System(e))
    }
}