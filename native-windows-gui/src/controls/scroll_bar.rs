use winapi::um::winuser::{WS_DISABLED, WS_VISIBLE, WS_TABSTOP, WS_CHILD, SBS_HORZ, SBS_VERT};
use crate::win32::window_helper as wh;
use crate::NwgError;
use super::{ControlBase, ControlHandle};

const NOT_BOUND: &'static str = "Scroll bar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Scroll bar handle is not HWND!";


bitflags! {
    /**
        The scroll bar flags

        * NONE:       No flags. Equivalent to a invisible scroll bar.
        * VISIBLE:    The scroll bar is immediatly visible after creation
        * DISABLED:   The scroll bar cannot be interacted with by the user. It also has a grayed out look.
        * TAB_STOP:   The control can be selected using tab navigation
        * HORIZONTAL: The scroll bar scrolls from top to bottom
        * VERTICAL:   The scroll bar scrolls from left to right
    */
    pub struct ScrollBarFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;
        const HORIZONTAL = SBS_HORZ;
        const VERTICAL = SBS_VERT;
    }
}

/**
A window can display a data object, such as a document or a bitmap, that is larger than the window's client area. When provided with a scroll bar, the user can scroll a data object in the client area to bring into view the portions of the object that extend beyond the borders of the window.

Requires the `scroll-bar` feature. 

*/
#[derive(Default, Eq, PartialEq)]
pub struct ScrollBar {
    pub handle: ControlHandle
}

impl ScrollBar {

    pub fn builder<'a>() -> ScrollBarBuilder {
        ScrollBarBuilder {
            size: (25, 100),
            position: (0, 0),
            enabled: true,
            flags: None,
            parent: None,
            focus: false
        }
    }

    /// Returns true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Sets the keyboard focus on the button.
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Returns true if the control user can interact with the control, return false otherwise
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

    /// Returns true if the control is visible to the user. Will return true even if the 
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

    /// Returns the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Sets the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Returns the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Sets the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "SCROLLBAR"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | WS_TABSTOP | SBS_VERT
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        WS_CHILD
    }

}

pub struct ScrollBarBuilder {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    flags: Option<ScrollBarFlags>,
    parent: Option<ControlHandle>,
    focus: bool,
}

impl ScrollBarBuilder {
    
    pub fn flags(mut self, flags: ScrollBarFlags) -> ScrollBarBuilder {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ScrollBarBuilder {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> ScrollBarBuilder {
        self.position = pos;
        self
    }

    pub fn enabled(mut self, e: bool) -> ScrollBarBuilder {
        self.enabled = e;
        self
    }

    pub fn focus(mut self, focus: bool) -> ScrollBarBuilder {
        self.focus = focus;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ScrollBarBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut ScrollBar) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("ScrollBar"))
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        out.set_enabled(self.enabled);

        if self.focus {
            out.set_focus();
        }

        Ok(())
    }


}
