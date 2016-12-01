/*!
    Various helper functions to create and interact with system window.
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

use std::ptr;
use std::mem;

use winapi::WNDPROC;

use low::other::to_utf16;
use error::SystemError;

pub struct SysclassParams<S: Into<String>> {
    pub class_name: S,
    pub sysproc: WNDPROC
}

/**
    Try to create a system class wuing the parameters provided in `SysclassParams`. Will not fail if
    the system class already exists.
    
    Returns `Err(SystemError::SysclassCreationFailed)` if the system class creation failed.
*/
pub unsafe fn build_sysclass<S: Into<String>>(p: SysclassParams<S>) -> Result<(), SystemError> {
    use kernel32::{GetModuleHandleW, GetLastError};
    use user32::{LoadCursorW, RegisterClassExW};
    use winapi::{WNDCLASSEXW, CS_HREDRAW, CS_VREDRAW, IDC_ARROW, COLOR_WINDOW, HBRUSH, UINT, ERROR_CLASS_ALREADY_EXISTS};

    let hmod = GetModuleHandleW(ptr::null_mut());
    if hmod.is_null() { return Err(SystemError::SystemClassCreation); }

    let class_name = to_utf16(p.class_name.into().as_ref());

    let class =
    WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: p.sysproc, 
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hmod,
        hIcon: ptr::null_mut(),
        hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
        hbrBackground: mem::transmute(COLOR_WINDOW as HBRUSH),
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: ptr::null_mut()
    };

    let class_token = RegisterClassExW(&class);
    if class_token == 0 && GetLastError() != ERROR_CLASS_ALREADY_EXISTS { 
        Err(SystemError::SystemClassCreation)
    } else {
        Ok(())
    }
}