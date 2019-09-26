use crate::win32::window_helper as wh;
use super::ControlHandle;
use std::ops::Range;

const NOT_BOUND: &'static str = "Progress bar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Progress bar handle is not HWND!";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ProgressBarState {
    Normal,
    Error,
    Paused
}

#[derive(Default, Debug)]
pub struct ProgressBar {
    pub handle: ControlHandle
}

impl ProgressBar {

    /// Return the current state of the progress bar
    pub fn state(&self) -> ProgressBarState {
        use winapi::um::commctrl::{PBM_GETSTATE, PBST_NORMAL, PBST_ERROR, PBST_PAUSED};
        
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        match wh::send_message(handle, PBM_GETSTATE, 0, 0) as i32 {
            PBST_NORMAL => ProgressBarState::Normal,
            PBST_ERROR => ProgressBarState::Error,
            PBST_PAUSED => ProgressBarState::Paused,
            _ => panic!("Unkown progress bar state")
        }
    }

    /// Set the state of the progress bar
    pub fn set_state(&self, state: ProgressBarState) {
        use winapi::um::commctrl::{PBM_SETSTATE, PBST_NORMAL, PBST_ERROR, PBST_PAUSED};
        use winapi::shared::minwindef::WPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let state = match state {
            ProgressBarState::Normal => PBST_NORMAL,
            ProgressBarState::Error => PBST_ERROR,
            ProgressBarState::Paused => PBST_PAUSED
        };

        wh::send_message(handle, PBM_SETSTATE, state as WPARAM, 0);
    }

    /// Increase the bar value by the step value
    pub fn advance(&self) {
        use winapi::um::commctrl::PBM_STEPIT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, PBM_STEPIT, 0, 0);
    }

    /// Increase the bar value by a value
    pub fn advance_delta(&self, v: u32) {
        use winapi::um::commctrl::PBM_DELTAPOS;
        use winapi::shared::minwindef::WPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, PBM_DELTAPOS, v as WPARAM, 0);
    }

    /// Return the step of the progress bar.
    pub fn step(&self) -> u32 {
        use winapi::um::commctrl::PBM_GETSTEP;
        
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, PBM_GETSTEP, 0, 0) as u32
    }

    /// Set the step of the progress bar.
    pub fn set_step(&self, s: u32) {
        use winapi::um::commctrl::PBM_SETSTEP;
        use winapi::shared::minwindef::WPARAM;
        
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, PBM_SETSTEP, s as WPARAM, 0);
    }

    /// Return the position of the progress bar.
    pub fn pos(&self) -> u32 {
        use winapi::um::commctrl::PBM_GETPOS;
        
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, PBM_GETPOS, 0, 0) as u32
    }

    /// Set the position of the progress bar. If the value is outside of range
    /// sets the value to the nearest bound.
    pub fn set_pos(&self, p: u32) {
        use winapi::um::commctrl::PBM_SETPOS;
        use winapi::shared::minwindef::WPARAM;
        
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, PBM_SETPOS, p as WPARAM, 0);
    }

    /// Get the range of the progress bar
    pub fn range(&self) -> Range<u32> {
        use winapi::um::commctrl::PBM_GETRANGE;
        
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        let low = wh::send_message(handle, PBM_GETRANGE, 1, 0) as u32;
        let high = wh::send_message(handle, PBM_GETRANGE, 0, 0) as u32;

        low..high
    }

    /// Set the range of the progress bar
    pub fn set_range(&self, range: Range<u32>) {
        use winapi::um::commctrl::PBM_SETRANGE32;
        use winapi::shared::minwindef::{WPARAM, LPARAM};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, PBM_SETRANGE32, range.start as WPARAM, range.end as LPARAM);
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

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("msctls_progress32")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> (u32, u32) {
        (::winapi::um::winuser::WS_VISIBLE, 0)
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD};

        WS_CHILD
    }

}
