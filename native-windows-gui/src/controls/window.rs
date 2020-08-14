/*!
    A basic top level window.
*/

use winapi::um::winuser::{WS_OVERLAPPEDWINDOW, WS_CLIPCHILDREN, WS_VISIBLE, WS_DISABLED, WS_MAXIMIZE, WS_MINIMIZE, WS_CAPTION,
WS_MINIMIZEBOX, WS_MAXIMIZEBOX, WS_SYSMENU, WS_THICKFRAME, WS_POPUP, WS_EX_TOPMOST, WS_EX_ACCEPTFILES, WS_EX_COMPOSITED};

use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::{NwgError, Icon};
use super::{ControlBase, ControlHandle};

const NOT_BOUND: &'static str = "Window is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Window handle is not HWND!";


bitflags! {

    /**
        The window flags. 

        Example: `WindowFlags::MAIN_WINDOW | WindowFlags::VISIBLE`

        Window flags:
        * MAIN_WINDOW: Combine all the top level system window decoration: A title, a system menu, a resizable frame, and the close, minimize, maximize buttons
        * WINDOW:  A window with a title, a system menu, a close button, and a non resizable border. 
        * MINIMIZE_BOX: Includes a minimize button
        * MAXIMIZE_BOX: Includes a maximize button
        * SYS_MENU: Includes a system menu when the user right click the window header
        * MAXIMIZED: Create the window as maximized
        * MINIMIZED: Create the window as minimized
        * RESIZABLE: Add a resizable border
        * VISIBLE: Show the window right away
    */
    pub struct WindowFlags: u32 {
        const MAIN_WINDOW = WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_THICKFRAME | WS_MAXIMIZEBOX;
        const WINDOW = WS_CAPTION | WS_SYSMENU;
        const MINIMIZE_BOX = WS_MINIMIZEBOX;
        const MAXIMIZE_BOX = WS_MAXIMIZEBOX;
        const SYS_MENU = WS_SYSMENU;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const MAXIMIZED = WS_MAXIMIZE;
        const MINIMIZED = WS_MINIMIZE;
        const RESIZABLE = WS_THICKFRAME | WS_MAXIMIZEBOX;
        const POPUP = WS_POPUP;
    }
}


/**
    A basic top level window. At least one top level window is required to make a NWG application.

    Windows can be heavily customized using the window flags. If your application don't need a visible window
    (ex: a system tray app), use `MessageWindow` instead.

    **Builder parameters:**
      * `flags`: The window flags. See `WindowFlags`
      * `title`: The text in the window title bar
      * `size`: The default size of the window
      * `position`: The default position of the window in the desktop
      * `icon`: The window icon
      * `accept_file`: If the window should accept files by drag & drop
      * `topmost`: If the window should always be on top of other system window
      * `parent`: Logical parent of the window, unlike children controls, this is NOT required.

    **Control events:**
      * `OnInit`: The window was created
      * `MousePress(_)`: Generic mouse press events on the button
      * `OnMouseMove`: Generic mouse mouse event
      * `OnMouseWheel`: Generic mouse wheel event
      * `OnPaint`: Generic on paint event
      * `OnKeyPress`: Generic key press
      * `OnKeyRelease`: Generic ket release
      * `OnResize`: When the window is resized
      * `OnResizeBegin`: Just before the window begins being resized by the user
      * `OnResizeEnd`: Just after the user stops resizing the window
      * `OnWindowMaximize`: When the window is maximized
      * `OnWindowMinimize`: When the window is minimized
      * `OnMove`: When the window is moved by the user
      * `OnFileDrop`: When a file is dropped in the window (only raised if accept_file is set)
      * `OnMinMaxInfo`: When the size or position of the window is about to change and the size of the windows must be restricted

*/
#[derive(Default, PartialEq, Eq)]
pub struct Window {
    pub handle: ControlHandle
}

impl Window {

    pub fn builder<'a>() -> WindowBuilder<'a> {
        WindowBuilder {
            title: "New Window",
            size: (500, 500),
            position: (300, 300),
            accept_files: false,
            topmost: false,
            flags: None,
            icon: None,
            parent: None
        }
    }

    /// Force the window to refraw iteself and all its children
    pub fn invalidate(&self) {
        use winapi::um::winuser::InvalidateRect;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { InvalidateRect(handle, ::std::ptr::null(), 1); }
    }

    /// Close the window as if the user clicked the X button.
    pub fn close(&self) {
        use winapi::um::winuser::WM_CLOSE;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::post_message(handle, WM_CLOSE, 0, 0);
    }

    /// Return the icon of the window
    pub fn icon(&self) -> Option<Icon> {
        use winapi::um::winuser::WM_GETICON;
        use winapi::um::winnt::HANDLE;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let handle = wh::send_message(handle, WM_GETICON, 0, 0);
        if handle == 0 {
            None
        } else {
            Some(Icon { handle: handle as HANDLE, owned: false })
        }
    }

    /// Set the icon in the window
    /// - icon: The new icon. If None, the icon is removed
    pub fn set_icon(&self, icon: Option<&Icon>) {
        use winapi::um::winuser::WM_SETICON;
        use std::{mem, ptr};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let image_handle = icon.map(|i| i.handle).unwrap_or(ptr::null_mut());
        unsafe {
            wh::send_message(handle, WM_SETICON, 0, mem::transmute(image_handle));
        }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
    pub fn set_focus(&self) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, true) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return window title
    pub fn text(&self) -> String { 
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the window title
    pub fn set_text<'a>(&self, v: &'a str) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "NativeWindowsGuiWindow"
    }

    // Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_OVERLAPPEDWINDOW | WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        WS_CLIPCHILDREN
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

pub struct WindowBuilder<'a> {
    title: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    accept_files: bool,
    topmost: bool,
    flags: Option<WindowFlags>,
    icon: Option<&'a Icon>,
    parent: Option<ControlHandle>
}

impl<'a> WindowBuilder<'a> {

    pub fn flags(mut self, flags: WindowFlags) -> WindowBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn title(mut self, text: &'a str) -> WindowBuilder<'a> {
        self.title = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> WindowBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> WindowBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn icon(mut self, ico: Option<&'a Icon>) -> WindowBuilder<'a> {
        self.icon = ico;
        self
    }

    pub fn accept_files(mut self, accept_files: bool) ->  WindowBuilder<'a> {
        self.accept_files = accept_files;
        self
    }

    pub fn topmost(mut self, topmost: bool) ->  WindowBuilder<'a> {
        self.topmost = topmost;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: Option<C>) -> WindowBuilder<'a> {
        self.parent = p.map(|p2| p2.into());
        self
    }

    pub fn build(self, out: &mut Window) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let mut ex_flags = WS_EX_COMPOSITED;
        if self.topmost { ex_flags |= WS_EX_TOPMOST; }
        if self.accept_files { ex_flags |= WS_EX_ACCEPTFILES; }

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .ex_flags(ex_flags)
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .text(self.title)
            .parent(self.parent)
            .build()?;

        if self.icon.is_some() {
            out.set_icon(self.icon);
        }

        Ok(())
    }

}
