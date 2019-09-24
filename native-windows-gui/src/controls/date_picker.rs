use crate::win32::window_helper as wh;
use crate::win32::base_helper::to_utf16;
use crate::Font;
use super::ControlHandle;

const NOT_BOUND: &'static str = "DatePicker is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: DatePicker handle is not HWND!";

/// If the date time picker is set as "optional", a checkbox is set next to the input
/// to show that no value is selected in the control.
#[derive(Clone, PartialEq, Debug)]
pub enum DatePickerCheckState {
    Checked,
    Unchecked
}

/**
    A date struct that can be passed to a date time picker control.
*/
#[derive(Clone, PartialEq, Debug)]
pub struct DatePickerValue {
    pub year: u16,
    pub month: u16,
    pub day: u16
}


#[derive(Default, Debug)]
pub struct DatePicker {
    pub handle: ControlHandle
}

impl DatePicker {

    /// Set the format of the date time picker. Use the format specified in the DTP template.
    pub fn set_format<S: Clone+Into<String>>(&self, format: &S) {
        use winapi::um::commctrl::DTM_SETFORMATW;
        use winapi::shared::minwindef::LPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let format = to_utf16(format.clone().into().as_str());

        wh::send_message(handle, DTM_SETFORMATW, 0, format.as_ptr() as LPARAM);
    }

    /**
        Return the check state of the checkbox of the control (if optional was set to true).  
        If the date time picker is not optional, return false.
    */
    pub fn get_checkstate(&self) -> DatePickerCheckState {
        use winapi::um::winuser::STATE_SYSTEM_CHECKED;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let info = unsafe{ get_dtp_info(handle) };

        match info.stateCheck {
            STATE_SYSTEM_CHECKED => DatePickerCheckState::Checked,
            _ => DatePickerCheckState::Unchecked
        }
    }

    /// Close the calendar popup if it is open  
    pub fn close_calendar(&self) {
        use winapi::um::commctrl::DTM_CLOSEMONTHCAL;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        wh::send_message(handle, DTM_CLOSEMONTHCAL, 0, 0);
    }

    /**
        Return the time set in the control in a `PickerDate` structure.  
        Return None if `optional` was set and the checkbox is not checked.  
        Note: use `get_text` to get the text value of the control.
    */
    pub fn get_value(&self) -> Option<DatePickerValue> {
        use winapi::um::commctrl::{GDT_VALID, DTM_GETSYSTEMTIME};
        use winapi::um::minwinbase::SYSTEMTIME;
        use std::mem;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let mut syst: SYSTEMTIME = unsafe{ mem::uninitialized() };

        let r = unsafe{ wh::send_message(handle, DTM_GETSYSTEMTIME, 0, mem::transmute(&mut syst)) };
        match r {
            GDT_VALID => Some(DatePickerValue {
                year: syst.wYear,
                month: syst.wMonth,
                day: syst.wDay
            }),
            _ => None
        }
    }

    /**
        Set the time set in the control in a `PickerDate` structure.  
        If `None` is passed, this clears the checkbox.
    
    pub fn set_value(&self, date: &Option<PickerDate>) {
        use winapi::{DTM_SETSYSTEMTIME, GDT_VALID, GDT_NONE, WPARAM};
        unsafe{
            match date {
                &Some(ref date) => {
                    let syst: SYSTEMTIME = SYSTEMTIME{ 
                        wYear: date.year, 
                        wMonth: date.month, 
                        wDay: date.day, 
                        wDayOfWeek:0, wHour:0, wMinute:0, wSecond:0, wMilliseconds: 0 
                    };
                    SendMessageW(self.handle, DTM_SETSYSTEMTIME, GDT_VALID as WPARAM, mem::transmute(&syst));
                },
                &None => { 
                    SendMessageW(self.handle, DTM_SETSYSTEMTIME, GDT_NONE as WPARAM, 0); 
                }
            };
        }
    }
    */

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
        Some("SysDateTimePick32")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> (u32, u32) {
        (::winapi::um::winuser::WS_VISIBLE, 0)
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD};
        use winapi::um::commctrl::DTS_SHOWNONE;

        WS_CHILD | DTS_SHOWNONE
    }

}


use winapi::um::commctrl::DATETIMEPICKERINFO;
use winapi::shared::windef::HWND;

unsafe fn get_dtp_info(handle: HWND) -> DATETIMEPICKERINFO {
    use winapi::um::commctrl::DTM_GETDATETIMEPICKERINFO;
    use winapi::shared::minwindef::DWORD;
    use std::mem;

    let mut dtp_info: DATETIMEPICKERINFO = mem::uninitialized();
    dtp_info.cbSize = mem::size_of::<DATETIMEPICKERINFO>() as DWORD;
    wh::send_message(handle, DTM_GETDATETIMEPICKERINFO, 0, mem::transmute(&mut dtp_info));

    dtp_info
}
