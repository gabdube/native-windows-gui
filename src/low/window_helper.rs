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
use std::hash::Hash;

use winapi::{HWND, WNDPROC, DWORD, GWL_USERDATA};

use ui::UiInner;
use low::other_helper::to_utf16;
use error::SystemError;

/**
    Params used to build a system class

    class_name: System class name
    sysproc: The system class procedure
*/
pub struct SysclassParams<S: Into<String>> {
    pub class_name: S,
    pub sysproc: WNDPROC
}

/**
    Params used to build a system window

    class_name: System class name
    sysproc: The system class procedure
*/
pub struct WindowParams<S1: Into<String>, S2: Into<String>> {
    pub title: S1,
    pub class_name: S2,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub flags: DWORD
}

/**
    Try to create a system class using the parameters provided in `SysclassParams`. Will not fail if
    the system class already exists.
    
    Returns `Err(SystemError::SysclassCreationFailed)` if the system class creation failed.

    Note that if the system class window proc used is malformed, the program will most likely segfault.
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

/**
    Try to create a system class using the parameters provided in `WindowParams`.
    
    Returns `Ok(HWND)` where HWND is the newly created window handle
    Returns `Err(SystemError::WindowCreationFail)` if the system window creation failed.

    Note that if the system class window proc used is malformed, the program will most likely segfault.
*/
pub unsafe fn build_window<S1: Into<String>, S2: Into<String>>(p: WindowParams<S1, S2>) -> Result<HWND, SystemError>{
    use kernel32::GetModuleHandleW;
    use user32::CreateWindowExW;
    use winapi::{WS_EX_COMPOSITED};

    let hmod = GetModuleHandleW(ptr::null_mut());
    if hmod.is_null() { return Err(SystemError::WindowCreationFail); }

    let class_name = to_utf16(p.class_name.into().as_ref());
    let window_name = to_utf16(p.title.into().as_ref());

    let handle = CreateWindowExW (
        WS_EX_COMPOSITED,
        class_name.as_ptr(), window_name.as_ptr(),
        p.flags,
        p.position.0, p.position.1,
        p.size.0 as i32, p.size.1 as i32,
        ptr::null_mut(),
        ptr::null_mut(),
        hmod,
        ptr::null_mut()
    );

    if handle.is_null() {
        Err(SystemError::WindowCreationFail)
    } else {
        Ok(handle)
    }
}

/**
    Return the children control found in the window. Includes the window menubar if one is present.
*/
#[allow(unused_variables)]
pub unsafe fn list_window_children<ID: Clone+Hash>(handle: HWND, ui: *mut UiInner<ID>) -> Vec<u64> {
    use user32::GetMenu;
    use low::menu_helper::list_menu_children;

    let mut children = Vec::new();

    let menu = GetMenu(handle);
    if !menu.is_null() {
        println!("{:?}", "About to free the window children");
        children.append(&mut list_menu_children(menu) );
    }

    // TODO to window children

    children
}

/**
    Remove any NWG data associated to the window handle. This is called by custom controls if
    they must be unregistered but can't access the Ui. Ex: When a user close a Window control.

    This has the same effect of calling `ui.unpack_control(x)`

    If the window do not belong to NWG or was already freed, nothing is done.
*/
#[inline(always)]
pub unsafe fn unpack_window_indirect(handle: HWND) {
    use user32::SendMessageW;
    use low::defs::NWG_UNPACK_INDIRECT;
    SendMessageW(handle, NWG_UNPACK_INDIRECT, 0, 0);
}


#[cfg(target_arch = "x86")] use winapi::LONG;
#[cfg(target_arch = "x86_64")] use winapi::LONG_PTR;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn get_window_long(handle: HWND) -> LONG_PTR {
    use user32::GetWindowLongPtrW;
    unsafe{ GetWindowLongPtrW(handle, GWL_USERDATA) }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn get_window_long(handle: HWND) -> LONG {
    use user32::GetWindowLongW;
    unsafe { GetWindowLongW(handle, GWL_USERDATA) }
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn set_window_long(handle: HWND, v: usize) {
    use user32::SetWindowLongPtrW;
    unsafe{ SetWindowLongPtrW(handle, GWL_USERDATA, v as LONG_PTR); }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn set_window_long(handle: HWND, v: usize) {
    use user32::SetWindowLongW;
    unsafe { SetWindowLongW(handle, GWL_USERDATA, v as LONG); }
}