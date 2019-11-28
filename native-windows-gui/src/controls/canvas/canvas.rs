/*!
    A canvas is a control that can be entirely painted by the user.
    See CanvasWindow for the window variant of the control
*/

use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_CHILD};

use crate::win32::window_helper as wh;
use crate::win32::canvas;
use crate::{SystemError};
use super::CanvasDraw;
use super::super::{ControlBase, ControlHandle};
use std::ops::Deref;

const NOT_BOUND: &'static str = "Canvas is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Canvas handle is not HWND!";


bitflags! {
    pub struct CanvasFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}


/**
A canvas is a control that can be entirely painted by the user.
See CanvasWindow for the window variant of the control
*/
#[derive(Default, Debug)]
pub struct Canvas {
    pub handle: ControlHandle,
    renderer: canvas::CanvasRenderer
}

impl Canvas {

    pub fn builder() -> CanvasBuilder {
        CanvasBuilder {
            size: (500, 500),
            position: (300, 300),
            flags: None,
            parent: None
        }
    }

    /// Begin drawing on the canvas. Return a `CanvasDraw` object that contains the draw methods.
    pub fn begin_draw<'a>(&'a self) -> CanvasDraw<'a> {
        CanvasDraw::new(&self.renderer)
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

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
       "NWG_CANVAS"
    }

    // Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        WS_CHILD
    }
}


pub struct CanvasBuilder {
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<CanvasFlags>,
    parent: Option<ControlHandle>
}

impl CanvasBuilder {

    pub fn flags(mut self, flags: CanvasFlags) -> CanvasBuilder {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> CanvasBuilder {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> CanvasBuilder {
        self.position = pos;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: Option<C>) -> CanvasBuilder {
        self.parent = p.map(|p2| p2.into());
        self
    }

    pub fn build(self, out: &mut Canvas) -> Result<(), SystemError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(SystemError::ControlWithoutParent)
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        unsafe {
            out.renderer = canvas::build_renderer(out.handle.hwnd().unwrap())?;
        }

        Ok(())
    }

}


impl Deref for Canvas {
    type Target = canvas::CanvasRenderer;

    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}
