use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::to_utf16;
use crate::{Font, SystemError};
use super::{ControlBase, ControlHandle};

const NOT_BOUND: &'static str = "DatePicker is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: DatePicker handle is not HWND!";


bitflags! {
    pub struct DatePickerFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/**
    A date struct that can be passed to a date time picker control.
*/
#[derive(Clone, Copy, PartialEq, Debug)]
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

    pub fn builder<'a>() -> DatePickerBuilder<'a> {
        DatePickerBuilder {
            size: (100, 25),
            position: (0, 0),
            flags: None,
            font: None,
            parent: None,
            date: None,
            format: None,
            range: None
        }
    }

    /**
        Sets the date format of the control

        About the format string:  
        - "d" 	    The one- or two-digit day.  
        - "dd" 	    The two-digit day. Single-digit day values are preceded by a zero.  
        - "ddd" 	The three-character weekday abbreviation.  
        - "dddd" 	The full weekday name.  
        - "M" 	    The one- or two-digit month number.  
        - "MM" 	    The two-digit month number. Single-digit values are preceded by a zero.  
        - "MMM" 	The three-character month abbreviation.  
        - "MMMM" 	The full month name.  
        - "t" 	    The one-letter AM/PM abbreviation (that is, AM is displayed as "A").  
        - "tt" 	    The two-letter AM/PM abbreviation (that is, AM is displayed as "AM").  
        - "yy" 	    The last two digits of the year (that is, 1996 would be displayed as "96").  
        - "yyyy" 	The full year (that is, 1996 would be displayed as "1996").   

        - "h" 	The one- or two-digit hour in 12-hour format.
        - "hh" 	The two-digit hour in 12-hour format. Single-digit values are preceded by a zero.
        - "H" 	The one- or two-digit hour in 24-hour format.
        - "HH" 	The two-digit hour in 24-hour format. Single-digit values are preceded by a zero.
        - "m" 	The one- or two-digit minute.
        - "mm" 	The two-digit minute. Single-digit values are preceded by a zero.
        
        Furthermore, any string enclosed in `'` can be used in the format to display text.  
        For example, to display the current date with the format `'Today is: Tuesday Mar 23, 1996`, the format string is `'Today is: 'dddd MMM dd', 'yyyy`. 
    
        If `format` is set to `None`, use the default system format.
    */
    pub fn set_format<'a>(&self, format: Option<&'a str>) {
        use winapi::um::commctrl::DTM_SETFORMATW;
        use winapi::shared::minwindef::LPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let (_format, format_ptr) = if format.is_some() {
            let f = to_utf16(format.unwrap());
            let fptr = f.as_ptr() as LPARAM;
            (f, fptr)
        } else {
            (Vec::new(), 0)
        };

        wh::send_message(handle, DTM_SETFORMATW, 0, format_ptr);
    }

    /**
        Return the check state of the checkbox of the control.  
        If the date time picker is not optional, return false.

        To set the check state of the control, use `set_value` method
    */
    pub fn checked(&self) -> bool {
        use winapi::um::winuser::STATE_SYSTEM_CHECKED;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let info = unsafe{ get_dtp_info(handle) };

        match info.stateCheck {
            STATE_SYSTEM_CHECKED => true,
            _ => false
        }
    }

    /// Close the calendar popup if it is open. Note that there is no way to force the calendar to drop down
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
    pub fn value(&self) -> Option<DatePickerValue> {
        use winapi::um::commctrl::{GDT_VALID, DTM_GETSYSTEMTIME};
        use winapi::um::minwinbase::SYSTEMTIME;
        use std::mem;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let mut syst: SYSTEMTIME = unsafe{ mem::zeroed() };

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
    */
    pub fn set_value(&self, date: Option<DatePickerValue>) {
        use winapi::um::commctrl::{DTM_SETSYSTEMTIME, GDT_VALID, GDT_NONE};
        use winapi::shared::minwindef::{WPARAM, LPARAM};
        use winapi::um::minwinbase::SYSTEMTIME;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        match date {
            Some(date) => {
                let syst: SYSTEMTIME = SYSTEMTIME{ 
                    wYear: date.year, 
                    wMonth: date.month, 
                    wDay: date.day, 
                    wDayOfWeek:0, wHour:0, wMinute:0, wSecond:0, wMilliseconds: 0 
                };

                wh::send_message(handle, DTM_SETSYSTEMTIME, GDT_VALID as WPARAM, &syst as *const SYSTEMTIME as LPARAM);
            },
            None => { 
                wh::send_message(handle, DTM_SETSYSTEMTIME, GDT_NONE as WPARAM, 0); 
            }
        };
    }

    /// Gets the current minimum and maximum allowable system times for a date and time picker control.
    pub fn range(&self) -> [DatePickerValue; 2] {
        use winapi::um::commctrl::DTM_GETRANGE;
        use winapi::um::minwinbase::SYSTEMTIME;
        use winapi::shared::minwindef::{LPARAM};
        use std::mem;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let mut tr: [SYSTEMTIME; 2] = unsafe { mem::zeroed() };

        wh::send_message(handle, DTM_GETRANGE, 0, &mut tr as *mut [SYSTEMTIME; 2] as LPARAM); 
    
        [
            DatePickerValue { year: tr[0].wYear, month: tr[0].wMonth, day: tr[0].wDay },
            DatePickerValue { year: tr[1].wYear, month: tr[1].wMonth, day: tr[1].wDay },
        ]
        
    }
    
    /// Sets the minimum and maximum allowable system times for a date and time picker control. 
    pub fn set_range(&self, r: &[DatePickerValue; 2]) {
        use winapi::um::commctrl::DTM_SETRANGE;
        use winapi::um::commctrl::{GDTR_MIN, GDTR_MAX};
        use winapi::um::minwinbase::SYSTEMTIME;
        use winapi::shared::minwindef::{LPARAM};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let values = [
            SYSTEMTIME { wYear: r[0].year , wMonth: r[0].month, wDayOfWeek: 0, wDay: r[0].day, wHour: 0, wMinute: 0, wSecond: 0, wMilliseconds: 0 },
            SYSTEMTIME { wYear: r[1].year , wMonth: r[1].month, wDayOfWeek: 0, wDay: r[1].day, wHour: 0, wMinute: 0, wSecond: 0, wMilliseconds: 0 }
        ];

        wh::send_message(handle, DTM_SETRANGE, GDTR_MIN | GDTR_MAX, &values as *const [SYSTEMTIME; 2] as LPARAM);
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

    /// Sets the font of the control
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

    /// Sets the keyboard focus on the button.
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

    /// Return the size of the date picker in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the date picker in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the date picker in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the date picker in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }


    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "SysDateTimePick32"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD};
        use winapi::um::commctrl::DTS_SHOWNONE;

        WS_CHILD | DTS_SHOWNONE
    }

}

pub struct DatePickerBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<DatePickerFlags>,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>,
    date: Option<DatePickerValue>,
    format: Option<&'a str>,
    range: Option<[DatePickerValue; 2]>
}

impl<'a> DatePickerBuilder<'a> {

    pub fn flags(mut self, flags: DatePickerFlags) -> DatePickerBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> DatePickerBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> DatePickerBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> DatePickerBuilder<'a> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> DatePickerBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn date(mut self, date: Option<DatePickerValue>) -> DatePickerBuilder<'a> {
        self.date = date;
        self
    }

    pub fn format(mut self, format: Option<&'a str>) -> DatePickerBuilder<'a> {
        self.format = format;
        self
    }

    pub fn range(mut self, range: Option<[DatePickerValue; 2]>) -> DatePickerBuilder<'a> {
        self.range = range;
        self
    }

    pub fn build(self, out: &mut DatePicker) -> Result<(), SystemError> {
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

        if self.font.is_some() {
            out.set_font(self.font);
        }

        if self.date.is_some() {
            out.set_value(self.date)
        }

        if self.range.is_some() {
            out.set_range(&self.range.unwrap());
        }

        if self.format.is_some() {
            out.set_format(self.format);
        }

        Ok(())
    }

}



use winapi::um::commctrl::DATETIMEPICKERINFO;
use winapi::shared::windef::HWND;

unsafe fn get_dtp_info(handle: HWND) -> DATETIMEPICKERINFO {
    use winapi::um::commctrl::DTM_GETDATETIMEPICKERINFO;
    use winapi::shared::minwindef::DWORD;
    use std::mem;

    let mut dtp_info: DATETIMEPICKERINFO = mem::zeroed();
    dtp_info.cbSize = mem::size_of::<DATETIMEPICKERINFO>() as DWORD;
    wh::send_message(handle, DTM_GETDATETIMEPICKERINFO, 0, mem::transmute(&mut dtp_info));

    dtp_info
}
