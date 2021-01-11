use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_TABSTOP};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16, check_hwnd};
use crate::{Font, NwgError};
use super::{ControlBase, ControlHandle};

const NOT_BOUND: &'static str = "DatePicker is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: DatePicker handle is not HWND!";


bitflags! {

    /**
        The DatePickerFlags flags

        * NONE:     No flags. Equivalent to a invisible date picker.
        * VISIBLE:  The date picker is immediatly visible after creation
        * DISABLED: The date picker cannot be interacted with by the user. It also has a grayed out look.
        * TAB_STOP: The control can be selected using tab navigation
    */
    pub struct DatePickerFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;
    }
}

/**
    A date struct that can be passed to a date time picker control.
    Fields are self explanatory.
*/
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DatePickerValue {
    pub year: u16,
    pub month: u16,
    pub day: u16
}



/**
A date and time picker (DTP) control provides a simple and intuitive interface through which to exchange date and time information with a user.
For example, with a DTP control you can ask the user to enter a date and then easily retrieve the selection.

Requires the `datetime-picker` feature. 

**Builder parameters:**
  * `parent`:   **Required.** The dtp parent container.
  * `size`:     The dtp size.
  * `position`: The dtp position.
  * `enabled`:  If the dtp can be used by the user. It also has a grayed out look if disabled.
  * `flags`:    A combination of the DatePickerFlags values.
  * `ex_flags`: A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `font`:     The font used for the dtp text
  * `date`:     The default date as a `DatePickerValue` value
  * `format`:   The format of the date. See the `set_format` method.
  * `range`:    The accepted range of dates. The value is inclusive.
  * `focus`:    The control receive focus after being created

**Control events:**
  * `OnDatePickerClosed`: When the datepicker dropdown is closed
  * `OnDatePickerDropdown`: When the datepicker dropdown is opened
  * `OnDatePickerChanged`: When a new value in a datepicker is choosen
  * `MousePress(_)`: Generic mouse press events on the checkbox
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event

```rust
use native_windows_gui as nwg;
fn build_dtp(date: &mut nwg::DatePicker, window: &nwg::Window) {
    let v = nwg::DatePickerValue { year: 2000, month: 10, day: 5 };
    let v1 = nwg::DatePickerValue { year: 2000, month: 10, day: 5 };
    let v2 = nwg::DatePickerValue { year: 2012, month: 10, day: 5 };
    
    nwg::DatePicker::builder()
        .size((200, 300))
        .position((0, 0))
        .date(Some(v))
        .format(Some("'YEAR: 'yyyy"))
        .range(Some([v1, v2]))
        .parent(window)
        .build(date);
}
```
*/
#[derive(Default, PartialEq, Eq)]
pub struct DatePicker {
    pub handle: ControlHandle
}

impl DatePicker {

    pub fn builder<'a>() -> DatePickerBuilder<'a> {
        DatePickerBuilder {
            size: (100, 25),
            position: (0, 0),
            focus: false,
            flags: None,
            ex_flags: 0,
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
        - `d` 	    The one- or two-digit day.  
        - `dd` 	    The two-digit day. Single-digit day values are preceded by a zero.  
        - `ddd` 	The three-character weekday abbreviation.  
        - `dddd` 	The full weekday name.  
        - `M` 	    The one- or two-digit month number.  
        - `MM` 	    The two-digit month number. Single-digit values are preceded by a zero.  
        - `MMM` 	The three-character month abbreviation.  
        - `MMMM` 	The full month name.  
        - `t` 	    The one-letter AM/PM abbreviation (that is, AM is displayed as "A").  
        - `tt` 	    The two-letter AM/PM abbreviation (that is, AM is displayed as "AM").  
        - `yy` 	    The last two digits of the year (that is, 1996 would be displayed as "96").  
        - `yyyy` 	The full year (that is, 1996 would be displayed as "1996").   

        - `h` 	The one- or two-digit hour in 12-hour format.
        - `hh` 	The two-digit hour in 12-hour format. Single-digit values are preceded by a zero.
        - `H` 	The one- or two-digit hour in 24-hour format.
        - `HH` 	The two-digit hour in 24-hour format. Single-digit values are preceded by a zero.
        - `m` 	The one- or two-digit minute.
        - `mm` 	The two-digit minute. Single-digit values are preceded by a zero.
        
        Furthermore, any string enclosed in `'` can be used in the format to display text.  
        For example, to display the current date with the format `'Today is: Tuesday Mar 23, 1996`, the format string is `'Today is: 'dddd MMM dd', 'yyyy`. 
    
        If `format` is set to `None`, use the default system format.
    */
    pub fn set_format<'a>(&self, format: Option<&'a str>) {
        use winapi::um::commctrl::DTM_SETFORMATW;
        use winapi::shared::minwindef::LPARAM;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

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

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let info = unsafe{ get_dtp_info(handle) };

        match info.stateCheck {
            STATE_SYSTEM_CHECKED => true,
            _ => false
        }
    }

    /// Close the calendar popup if it is open. Note that there is no way to force the calendar to drop down
    pub fn close_calendar(&self) {
        use winapi::um::commctrl::DTM_CLOSEMONTHCAL;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
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

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

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

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

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

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

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

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let values = [
            SYSTEMTIME { wYear: r[0].year , wMonth: r[0].month, wDayOfWeek: 0, wDay: r[0].day, wHour: 0, wMinute: 0, wSecond: 0, wMilliseconds: 0 },
            SYSTEMTIME { wYear: r[1].year , wMonth: r[1].month, wDayOfWeek: 0, wDay: r[1].day, wHour: 0, wMinute: 0, wSecond: 0, wMilliseconds: 0 }
        ];

        wh::send_message(handle, DTM_SETRANGE, GDTR_MIN | GDTR_MAX, &values as *const [SYSTEMTIME; 2] as LPARAM);
    }

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Sets the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Sets the keyboard focus on the button.
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

    /// Return the size of the date picker in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the date picker in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the date picker in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the date picker in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }


    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "SysDateTimePick32"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | WS_TABSTOP
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD};
        use winapi::um::commctrl::DTS_SHOWNONE;

        WS_CHILD | DTS_SHOWNONE
    }

}

impl Drop for DatePicker {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

pub struct DatePickerBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<DatePickerFlags>,
    ex_flags: u32,
    font: Option<&'a Font>,
    focus: bool,
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

    pub fn ex_flags(mut self, flags: u32) -> DatePickerBuilder<'a> {
        self.ex_flags = flags;
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

    pub fn focus(mut self, focus: bool) -> DatePickerBuilder<'a> {
        self.focus = focus;
        self
    }

    pub fn build(self, out: &mut DatePicker) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("DatePicker"))
        }?;

        *out = Default::default();

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .ex_flags(self.ex_flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        if self.font.is_some() {
            out.set_font(self.font);
        } else {
            out.set_font(Font::global_default().as_ref());
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

        if self.focus {
            out.set_focus();
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
