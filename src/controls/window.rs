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

/// System class identifier
const WINDOW_CLASS_NAME: &'static str = "NWG_BUILTIN_WINDOW";

/**
    A template that will create a window.
*/
pub struct WindowT<S: Into<String>> {
    pub title: S,
    pub pos: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub enabled: bool,
}

/**
    A window control.
*/
pub struct Window {
    handle: HWND
}

impl<S: Into<String>> ControlT for WindowT<S> {
    fn type_id(&self) -> TypeId { TypeId::of::<Window>() }

    fn build(&self) -> Result<Box<Control>, Error> {

        if let Err(e) = unsafe{ build_sysclass() } {
            return Err(e);
        }

        Err(Error::BorrowError)
    }
}

impl Control for Window {}


/*
    Private unsafe control methods
*/

use winapi::{UINT, WPARAM, LPARAM, LRESULT};

#[allow(unused_variables)]
unsafe extern "system" fn window_sysproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::{WM_CREATE};
    use user32::{DefWindowProcW};

    let handled = match msg {
        WM_CREATE => true,
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