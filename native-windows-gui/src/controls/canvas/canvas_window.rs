/*!
    A canvas window is a window that can be entirely painted by the user.

    By default, this windows is not decorated by the system.
    It's possible to reactivate the system decorations by passing the flags such as `WINDOW` or  `MAINWINDOW`.
    A CanvasWindow has the same flags as a regular Window.
*/

use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_MAXIMIZE, WS_MINIMIZE, WS_CAPTION, WS_MINIMIZEBOX, WS_MAXIMIZEBOX, WS_SYSMENU, WS_THICKFRAME, WS_POPUP};

use crate::win32::window_helper as wh;
use crate::win32::canvas;
use crate::{SystemError, Image};
use super::super::{ControlBase, ControlHandle};
use std::ops::Deref;

const NOT_BOUND: &'static str = "CanvasWindow is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: CanvasWindow handle is not HWND!";


bitflags! {
    pub struct CanvasWindowFlags: u32 {
        const POPUP = WS_POPUP;
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
    }
}


/**
A canvas window is a window that can be entirely painted by the user.
See the module level documentation for more details
*/
#[derive(Default, Debug)]
pub struct CanvasWindow {
    pub handle: ControlHandle,
    renderer: canvas::CanvasRenderer
}

impl CanvasWindow {

    pub fn builder<'a>() -> CanvasWindowBuilder<'a> {
        CanvasWindowBuilder {
            title: "New Window",
            size: (500, 500),
            position: (300, 300),
            flags: None,
            icon: None,
            parent: None
        }
    }

    /// Return the icon of the window
    pub fn icon(&self) -> Option<Image> {
        use winapi::um::winuser::WM_GETICON;
        use std::mem;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let handle = wh::send_message(handle, WM_GETICON, 0, 0);
        if handle == 0 {
            None
        } else {
            Some(Image{ handle: unsafe{ mem::transmute(handle) } })
        }
    }

    /// Set the icon in the window
    /// - icon: The new icon. If None, the icon is removed
    pub fn set_icon(&self, icon: Option<&Image>) {
        use winapi::um::winuser::WM_SETICON;
        use std::{mem, ptr};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let image_handle = icon.map(|i| i.handle).unwrap_or(ptr::null_mut());
        unsafe {
            wh::send_message(handle, WM_SETICON, 0, mem::transmute(image_handle));
        }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, true) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return window title
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the window title
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
       "NWG_CANVAS"
    }

    // Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_POPUP | WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        0
    }
}


pub struct CanvasWindowBuilder<'a> {
    title: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<CanvasWindowFlags>,
    icon: Option<&'a Image>,
    parent: Option<ControlHandle>
}

impl<'a> CanvasWindowBuilder<'a> {

    pub fn flags(mut self, flags: CanvasWindowFlags) -> CanvasWindowBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn title(mut self, text: &'a str) -> CanvasWindowBuilder<'a> {
        self.title = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> CanvasWindowBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> CanvasWindowBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn icon(mut self, ico: Option<&'a Image>) -> CanvasWindowBuilder<'a> {
        self.icon = ico;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: Option<C>) -> CanvasWindowBuilder<'a> {
        self.parent = p.map(|p2| p2.into());
        self
    }

    pub fn build(self, out: &mut CanvasWindow) -> Result<(), SystemError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .text(self.title)
            .parent(self.parent)
            .build()?;

        if self.icon.is_some() {
            out.set_icon(self.icon);
        }

        unsafe {
            out.renderer = canvas::build_renderer(out.handle.hwnd().unwrap())?;
        }

        Ok(())
    }

}


impl Deref for CanvasWindow {
    type Target = canvas::CanvasRenderer;

    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}
