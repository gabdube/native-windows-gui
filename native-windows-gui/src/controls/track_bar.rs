use winapi::shared::minwindef::{WPARAM, LPARAM};
use winapi::um::winuser::WS_VISIBLE;
use winapi::um::commctrl::{TBS_AUTOTICKS, TBS_VERT, TBS_HORZ, TBS_TOP, TBS_BOTTOM, TBS_LEFT, TBS_RIGHT, TBS_NOTICKS, TBS_ENABLESELRANGE};
use crate::win32::window_helper as wh;
use super::ControlHandle;
use std::ops::Range;

const NOT_BOUND: &'static str = "TrackBar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TrackBar handle is not HWND!";



bitflags! {
    pub struct TrackBarFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const AUTO_TICK = TBS_AUTOTICKS;
        const VERTICAL = TBS_VERT;
        const HORIZONTAL = TBS_HORZ;
        const TICK_TOP = TBS_TOP;
        const TICK_BOTTOM = TBS_BOTTOM;
        const TICK_LEFT = TBS_LEFT;
        const TICK_RIGHT = TBS_RIGHT;
        const NO_TICK = TBS_NOTICKS;
        const RANGE = TBS_ENABLESELRANGE;
    }
}

/**
A trackbar is a window that contains a slider (sometimes called a thumb) in a channel, and optional tick marks.
When the user moves the slider, using either the mouse or the direction keys, the trackbar sends notification messages to indicate the change.
*/
#[derive(Default, Debug)]
pub struct TrackBar {
    pub handle: ControlHandle
}

impl TrackBar {

    /// Retrieves the current logical position of the slider in a trackbar.
    /// The logical positions are the integer values in the trackbar's range of minimum to maximum slider positions. 
    pub fn pos(&self) -> usize {
        use winapi::um::commctrl::TBM_GETPOS;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_GETPOS, 0, 0) as usize
    }

    /// Sets the current logical position of the slider in a trackbar.
    pub fn set_pos(&self, p: usize) {
        use winapi::um::commctrl::TBM_SETPOSNOTIFY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_SETPOSNOTIFY, 1, p as LPARAM);
    }

    /// Retrieves the starting and ending position of the current selection range in a trackbar.
    /// Only work for trackbar with the `Range` flags
    pub fn selection_range_pos(&self) -> Range<usize> {
        use winapi::um::commctrl::{TBM_GETSELEND, TBM_GETSELSTART};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let end = wh::send_message(handle, TBM_GETSELEND, 0, 0) as usize;
        let start = wh::send_message(handle, TBM_GETSELSTART, 0, 0) as usize;

        start..end
    }

    /// Sets the range value of the trackbar
    /// Only work for trackbar with the `Range` flags
    pub fn set_selection_range_pos(&self, value: Range<usize>) {
        use winapi::um::commctrl::{TBM_SETSELEND, TBM_SETSELSTART};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_SETSELEND, 0, value.end as LPARAM);
        wh::send_message(handle, TBM_SETSELSTART, 1, value.start as LPARAM);
    }

    /// Retrieves the minimum position for the slider in a trackbar. 
    pub fn range_min(&self) -> usize {
        use winapi::um::commctrl::TBM_GETRANGEMIN;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_GETRANGEMIN, 0, 0) as usize
    }

    /// Sets the minium logical position for the slider in a trackbar.
    pub fn set_range_min(&self, min: usize) {
        use winapi::um::commctrl::TBM_SETRANGEMIN;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_SETRANGEMIN, 1, min as LPARAM);
    }

    /// Retrieves the maximum position for the slider in a trackbar. 
    pub fn range_max(&self) -> usize {
        use winapi::um::commctrl::TBM_GETRANGEMAX;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_GETRANGEMAX, 0, 0) as usize
    }

    /// Sets the maximum logical position for the slider in a trackbar.
    pub fn set_range_max(&self, max: usize) {
        use winapi::um::commctrl::TBM_SETRANGEMAX;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_SETRANGEMAX, 1, max as LPARAM);
    }

    /// Retrieves the number of tick marks in a trackbar
    pub fn tics_len(&self) -> usize {
        use winapi::um::commctrl::TBM_GETNUMTICS;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_GETNUMTICS, 0, 0) as usize
    }

    /// Retrieves the logical position of a tick mark in a trackbar.
    /// The logical position can be any of the integer values in the trackbar's range of minimum to maximum slider positions. 
    pub fn tic_value(&self, index: usize) -> usize {
        use winapi::um::commctrl::TBM_GETTIC;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TBM_GETTIC, index as WPARAM, 0) as usize
    }

    //
    // Basic methods
    //

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
        use winapi::um::commctrl::TRACKBAR_CLASS;
        Some(TRACKBAR_CLASS)
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
