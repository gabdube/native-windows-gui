use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_BORDER, WS_CHILD, WS_CLIPCHILDREN, WS_EX_CONTROLPARENT};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::NwgError;
use super::{ControlBase, ControlHandle};

const NOT_BOUND: &'static str = "Frame is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Frame handle is not HWND!";


bitflags! {
    /**
        The frame flags

        * NONE:     No flags. Equivalent to a invisible frame without borders.
        * VISIBLE:  The frame is immediatly visible after creation
        * DISABLED: The frame chidlren cannot be interacted with by the user.
        * BORDER:   The frame has a thin black border
    */
    pub struct FrameFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const BORDER = WS_BORDER;
    }
}

/**
A frame is a rectangle containing children controls. Frame is implemented as a custom control.

Requires the `frame` feature. 

**Builder parameters:**
  * `parent`:   **Required.** The frame parent container.
  * `size`:     The frame size.
  * `position`: The frame position.
  * `enabled`:  If the frame children can be used by the user.
  * `flags`:    A combination of the FrameFlags values.
  * `ex_flags`: A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi

**Control events:**
  * `MousePress(_)`: Generic mouse press events on the button
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event
*/
#[derive(Default, PartialEq, Eq)]
pub struct Frame {
    pub handle: ControlHandle
}

impl Frame {

    pub fn builder() -> FrameBuilder {
        FrameBuilder {
            size: (100, 25),
            position: (0, 0),
            enabled: true,
            flags: None,
            ex_flags: 0,
            parent: None,
        }
    }

    /// Returns true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Sets the keyboard focus on the button.
    pub fn set_focus(&self) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Returns true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Returns true if the control is visible to the user. Will return true even if the 
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

    /// Returns the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Sets the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Returns the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Sets the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "NWG_FRAME"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | WS_BORDER
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        WS_CHILD | WS_CLIPCHILDREN
    }

}

impl Drop for Frame {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}
pub struct FrameBuilder {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    flags: Option<FrameFlags>,
    ex_flags: u32,
    parent: Option<ControlHandle>
}

impl FrameBuilder {

    pub fn flags(mut self, flags: FrameFlags) -> FrameBuilder {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> FrameBuilder {
        self.ex_flags = flags;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> FrameBuilder {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> FrameBuilder {
        self.position = pos;
        self
    }

    pub fn enabled(mut self, e: bool) -> FrameBuilder {
        self.enabled = e;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> FrameBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut Frame) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("Frame"))
        }?;

        *out = Default::default();

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .ex_flags(WS_EX_CONTROLPARENT | self.ex_flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        out.set_enabled(self.enabled);

        Ok(())
    }

}
