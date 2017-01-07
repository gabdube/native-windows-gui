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

use winapi::{HWND, HFONT, HBRUSH, WNDPROC, DWORD, LPARAM, BOOL, c_int};

use ui::{UiInner, Ui};
use controls::{AnyHandle};
use low::other_helper::to_utf16;
use error::{Error, SystemError};

/**
    Params used to build a system class

    class_name: System class name
    sysproc: The system class procedure
*/
pub struct SysclassParams<S: Into<String>> {
    pub class_name: S,
    pub sysproc: WNDPROC,
    pub background: Option<HBRUSH>,
    pub style: Option<u32>
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
    pub flags: DWORD,
    pub parent: HWND
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
    use winapi::{WNDCLASSEXW, CS_HREDRAW, CS_VREDRAW, IDC_ARROW, COLOR_WINDOW, UINT, ERROR_CLASS_ALREADY_EXISTS};

    let hmod = GetModuleHandleW(ptr::null_mut());
    if hmod.is_null() { return Err(SystemError::SystemClassCreation); }

    let class_name = to_utf16(p.class_name.into().as_ref());

    let background: HBRUSH = match p.background {
        Some(bg) => bg,
        None => mem::transmute(COLOR_WINDOW as usize)
    };

    let style: UINT = match p.style {
        Some(s) => s as UINT,
        None=> CS_HREDRAW | CS_VREDRAW
    };

    let class =
    WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
        style: style,
        lpfnWndProc: p.sysproc, 
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hmod,
        hIcon: ptr::null_mut(),
        hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
        hbrBackground: background,
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
    use winapi::WS_EX_COMPOSITED;

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
        p.parent,
        ptr::null_mut(),
        hmod,
        ptr::null_mut()
    );

    if handle.is_null() {
        Err(SystemError::WindowCreationFail)
    } else {
        fix_overlapped_window_size(handle, p.size);
        Ok(handle)
    }
}

/** 
    Fix: Window size include the non client area. This behaviour is not wanted
    Resize the client area to match the "true" size. 
*/
unsafe fn fix_overlapped_window_size(handle: HWND, size: (u32, u32)) {
    use winapi::{RECT, SWP_NOMOVE, SWP_NOZORDER, c_int};
    use user32::{GetClientRect, SetWindowPos};

    let mut rect: RECT = mem::uninitialized();
    GetClientRect(handle, &mut rect);

    let (w, h) = (size.0 as c_int, size.1 as c_int);
    let delta_width = w - rect.right;
    let delta_height = h - rect.bottom;
    
    SetWindowPos(handle, ptr::null_mut(), 0, 0,
      w+delta_width, h+delta_height,
      SWP_NOMOVE|SWP_NOZORDER);
}


unsafe extern "system" fn list_children_window<ID: Clone+Hash+'static>(handle: HWND, params: LPARAM) -> BOOL {
    let &mut (inner, ref mut ids): &mut (*mut UiInner<ID>, Vec<u64>) = mem::transmute(params);

    // Check if the window belongs to the ui
    if let Some(id) = ::low::events::window_id(handle, inner) {
        ids.push(id)
    }

    1
}

/**
    Return the children control found in the window. Includes the window menubar if one is present.
*/
#[allow(unused_variables)]
pub unsafe fn list_window_children<ID: Clone+Hash>(handle: HWND, ui: *mut UiInner<ID>) -> Vec<u64> {
    use user32::{GetMenu, EnumChildWindows};
    use low::menu_helper::list_menu_children;

    let mut children = Vec::new();

    let menu = GetMenu(handle);
    if !menu.is_null() {
        children.append(&mut list_menu_children((&*ui), menu) );
    }

    let mut params: (*mut UiInner<ID>, Vec<u64>) = (ui, children);
    EnumChildWindows(handle, Some(list_children_window::<ID>), mem::transmute(&mut params));

    params.1
}

/// Set the font of a window
pub unsafe fn set_window_font(handle: HWND, font_handle: Option<HFONT>, redraw: bool) {
    use user32::SendMessageW;
    use winapi::{WM_SETFONT, LPARAM};

    let font_handle = font_handle.unwrap_or(ptr::null_mut());

    SendMessageW(handle, WM_SETFONT, mem::transmute(font_handle), redraw as LPARAM);
}

/// Get the window text
#[inline(always)]
pub unsafe fn get_window_text(handle: HWND) -> String {
    use user32::{GetWindowTextW, GetWindowTextLengthW};
    use low::other_helper::from_utf16;

    let mut buffer_size = GetWindowTextLengthW(handle) as usize;
    if buffer_size == 0 { return String::new(); }

    buffer_size += 1;
    let mut buffer: Vec<u16> = Vec::with_capacity(buffer_size);
    buffer.set_len(buffer_size);

    if GetWindowTextW(handle, buffer.as_mut_ptr(), buffer_size as c_int) == 0 {
        String::new()
    } else {
        from_utf16(&buffer[..])
    }
}

/// Set the window text
#[inline(always)]
pub unsafe fn set_window_text<'a>(handle: HWND, text: &'a str) {
    use user32::SetWindowTextW;
    use low::other_helper::to_utf16;

    let text = to_utf16(text);
    SetWindowTextW(handle, text.as_ptr());
}


/// Set window position
#[inline(always)]
pub unsafe fn set_window_position(handle: HWND, x: i32, y: i32) {
    use user32::SetWindowPos;
    use winapi::{c_int, SWP_NOZORDER, SWP_NOSIZE, SWP_NOACTIVATE};

    SetWindowPos(handle, ptr::null_mut(), x as c_int, y as c_int, 0, 0, SWP_NOZORDER|SWP_NOSIZE|SWP_NOACTIVATE);
}

/// Get window position
#[inline(always)]
pub unsafe fn get_window_position(handle: HWND) -> (i32, i32) {
    use user32::{GetWindowRect, ScreenToClient, GetParent};
    use winapi::{RECT, POINT};
    
    let mut r: RECT = mem::uninitialized();
    GetWindowRect(handle, &mut r);

    let parent = GetParent(handle);
    if !parent.is_null() {
        let mut pt = POINT{x: r.left, y: r.top};
        ScreenToClient(parent, &mut pt);
        (pt.x as i32, pt.y as i32)
    } else {
        (r.left as i32, r.top as i32)
    }
}

/// Set window size
#[inline(always)]
pub unsafe fn set_window_size(handle: HWND, w: u32, h: u32, fix: bool) {
    use user32::SetWindowPos;
    use winapi::{c_int, SWP_NOZORDER, SWP_NOMOVE, SWP_NOACTIVATE};

    SetWindowPos(handle, ptr::null_mut(), 0, 0, w as c_int, h as c_int, SWP_NOZORDER|SWP_NOMOVE|SWP_NOACTIVATE);

    if fix { fix_overlapped_window_size(handle, (w, h)); }
}

/// Get window size
#[inline(always)]
pub unsafe fn get_window_size(handle: HWND) -> (u32, u32) {
    use user32::GetClientRect;
    use winapi::RECT;
    
    let mut r: RECT = mem::uninitialized();
    GetClientRect(handle, &mut r);

    (r.right as u32, r.bottom as u32)
}

/// Get the window enabled state
#[inline(always)]
pub unsafe fn get_window_enabled(handle: HWND) -> bool {
    use winapi::{GWL_STYLE, WS_DISABLED, UINT};

    let style = get_window_long(handle, GWL_STYLE) as UINT;
    (style & WS_DISABLED) != WS_DISABLED
}

/// Set the window enabled state
#[inline(always)]
pub unsafe fn set_window_enabled(handle: HWND, enabled: bool) {
    use winapi::{GWL_STYLE, WS_DISABLED};
    use user32::{UpdateWindow, InvalidateRect};

    let old_style = get_window_long(handle, GWL_STYLE) as usize;
    if enabled {
        set_window_long(handle, GWL_STYLE, old_style&(!WS_DISABLED as usize) );
    } else {
        set_window_long(handle, GWL_STYLE, old_style|(WS_DISABLED as usize));
    }

    // Tell the control to redraw itself to show the new style.
    InvalidateRect(handle, ptr::null(), 1);
    UpdateWindow(handle);
}

/// Set window visibility
#[inline(always)]
pub unsafe fn set_window_visibility(handle: HWND, visible: bool) {
    use user32::ShowWindow;
    use winapi::{SW_HIDE, SW_SHOW};

    let visible = if visible { SW_SHOW } else { SW_HIDE };
    ShowWindow(handle, visible);
}

/**
    Get window visibility
*/
#[inline(always)]
pub unsafe fn get_window_visibility(handle: HWND) -> bool {
    use user32::IsWindowVisible;
    IsWindowVisible(handle) != 0
}


#[inline(always)]
pub fn handle_of_window<ID: Clone+Hash>(ui: &Ui<ID>, id: &ID, err: &'static str) -> Result<HWND, Error> {
    match ui.handle_of(id) {
        Ok(AnyHandle::HWND(h)) => Ok(h),
        Ok(_) => Err(Error::BadParent(err.to_string())),
        Err(e) => Err(e)
    }
}

#[inline(always)]
pub fn handle_of_font<ID: Clone+Hash>(ui: &Ui<ID>, id: &ID, err: &'static str) -> Result<HFONT, Error> {
    match ui.handle_of(id) {
        Ok(AnyHandle::HFONT(h)) => Ok(h),
        Ok(_) => Err(Error::BadResource(err.to_string())),
        Err(e) => Err(e)
    }
} 

#[cfg(target_arch = "x86")] use winapi::LONG;
#[cfg(target_arch = "x86_64")] use winapi::LONG_PTR;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn get_window_long(handle: HWND, index: c_int) -> LONG_PTR {
    use user32::GetWindowLongPtrW;
    unsafe{ GetWindowLongPtrW(handle, index) }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn get_window_long(handle: HWND, index: c_int) -> LONG {
    use user32::GetWindowLongW;
    unsafe { GetWindowLongW(handle, index) }
}

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn set_window_long(handle: HWND, index: c_int, v: usize) {
    use user32::SetWindowLongPtrW;
    unsafe{ SetWindowLongPtrW(handle, index, v as LONG_PTR); }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn set_window_long(handle: HWND, index: c_int, v: usize) {
    use user32::SetWindowLongW;
    unsafe { SetWindowLongW(handle, index, v as LONG); }
}