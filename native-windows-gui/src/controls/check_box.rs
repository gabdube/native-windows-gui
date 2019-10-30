use crate::win32::window_helper as wh;
use crate::Font;
use super::ControlHandle;

const NOT_BOUND: &'static str = "CheckBox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: CheckBox handle is not HWND!";

/// Represents the check status of a checkbox

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CheckBoxState {
    Checked,
    Unchecked,

    /// New state for the tristate checkbox
    Indeterminate
}

/**
A check box consists of a square box and an application-defined label, icon, or bitmap that indicates a choice the user can make by selecting the button.
Applications typically display check boxes to enable the user to choose one or more options that are not mutually exclusive.

Note: Internally, check box are `Button` and as such, they trigger the same events
*/
#[derive(Default, Debug)]
pub struct CheckBox {
    pub handle: ControlHandle
}

impl CheckBox {

    /// Return `true` if the checkbox can have a third state or `false` otherwise
    pub fn tristate(&self) -> bool {
        use winapi::um::winuser::{BS_AUTO3STATE};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let style = wh::get_style(handle);

        style & BS_AUTO3STATE == BS_AUTO3STATE
    }

    /// Sets or unsets the checkbox as tristate
    pub fn set_tristate(&self, tri: bool) {
        use winapi::um::winuser::{BM_SETSTYLE, BS_AUTO3STATE, BS_AUTOCHECKBOX};
        use winapi::shared::minwindef::WPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        let style = match tri {
            true => BS_AUTO3STATE,
            false => BS_AUTOCHECKBOX
        };

        wh::send_message(handle, BM_SETSTYLE, style as WPARAM, 1);
    }

    /// Return the check state of the check box
    pub fn check_state(&self) -> CheckBoxState {
        use winapi::um::winuser::{BM_GETCHECK, BST_CHECKED, BST_INDETERMINATE, BST_UNCHECKED};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        match wh::send_message(handle, BM_GETCHECK, 0, 0) as usize {
            BST_UNCHECKED => CheckBoxState::Unchecked,
            BST_CHECKED => CheckBoxState::Checked,
            BST_INDETERMINATE => CheckBoxState::Indeterminate,
            _ => unreachable!()
        }
    }

    /// Sets the check state of the check box
    pub fn set_check_state(&self, state: CheckBoxState) {
        use winapi::um::winuser::{BM_SETCHECK, BST_CHECKED, BST_INDETERMINATE, BST_UNCHECKED};
        use winapi::shared::minwindef::WPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let x = match state {
            CheckBoxState::Unchecked => BST_UNCHECKED,
            CheckBoxState::Checked => BST_CHECKED,
            CheckBoxState::Indeterminate => BST_INDETERMINATE,
        };

        wh::send_message(handle, BM_SETCHECK, x as WPARAM, 0);
    }

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button.
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
        unsafe { wh::set_window_size(handle, x, y, false) }
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

    /// Return the button label
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the button label
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("BUTTON")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{BS_NOTIFY, WS_CHILD, BS_AUTOCHECKBOX};

        BS_NOTIFY | WS_CHILD | BS_AUTOCHECKBOX
    }

}
