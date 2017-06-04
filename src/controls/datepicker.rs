/*!
    Date picker control definition.
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use std::hash::Hash;
use std::any::TypeId;
use std::mem;

use winapi::{HWND, HFONT, DATETIMEPICKERINFO, SYSTEMTIME};
use user32::SendMessageW;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::{Event, Destroyed, Moved, Resized};
use events::datepicker::DateChanged;
use defs::{HTextAlign, CheckState, PickerDate};
use low::other_helper::to_utf16;

/**
    A template that creates a standard date picker (dtp)  

    About the format string:  
    "d" 	The one- or two-digit day.  
    "dd" 	The two-digit day. Single-digit day values are preceded by a zero.  
    "ddd" 	The three-character weekday abbreviation.  
    "dddd" 	The full weekday name.  
    "M" 	The one- or two-digit month number.  
    "MM" 	The two-digit month number. Single-digit values are preceded by a zero.  
    "MMM" 	The three-character month abbreviation.  
    "MMMM" 	The full month name.  
    "t" 	The one-letter AM/PM abbreviation (that is, AM is displayed as "A").  
    "tt" 	The two-letter AM/PM abbreviation (that is, AM is displayed as "AM").  
    "yy" 	The last two digits of the year (that is, 1996 would be displayed as "96").  
    "yyyy" 	The full year (that is, 1996 would be displayed as "1996").   

    Furthermore, any string enclosed in `'` can be used in the format to display text.  
    For example, to display the current date with the format `'Today is: Tuesday Mar 23, 1996`, the format string is `'Today is: 'dddd MMM dd', 'yyyy`. 

    Events:  
    `Destroyed, Moved, Resized, DateChanged, Any`  

    Members:  
    • `value`: The value of the dtp. If None, either use the current system time or show nothing (if optional is true)
    • `position`: The start position of the dtp  
    • `size`: The start size of the dtp  
    • `visible`: If the dtp should be visible to the user   
    • `disabled`: If the user can or can't edit the value of the dtp  
    • `parent`: The dtp parent  
    • `font`: The dtp font. If None, use the system default  
    • `align`: The alignment of the dtp control,
    • `format`: The dtp format string. See the docs just above for the available formats. If left empty, use the default system locale date format.  
    • `optional`: If the dtp must contain a value (or not)  
*/
#[derive(Clone)]
pub struct DatePickerT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub value: Option<PickerDate>,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID,
    pub font: Option<ID>,
    pub align: HTextAlign,
    pub format: S,
    pub optional: bool,
    pub range: (Option<PickerDate>, Option<PickerDate>)
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for DatePickerT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<DatePicker>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, Moved, Resized, DateChanged, Event::Any]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font, handle_of_window, handle_of_font};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, DTS_SHOWNONE, DTS_RIGHTALIGN};

        let flags: DWORD = WS_CHILD |
        if self.visible  { WS_VISIBLE }   else { 0 } |
        if self.disabled { WS_DISABLED }  else { 0 } |
        if self.optional { DTS_SHOWNONE } else { 0 } |
        match self.align   { 
            HTextAlign::Center=>{ return Err(Error::UserError("The date time picker control do not support centered text.".to_string())); }, 
            HTextAlign::Left=>0, 
            HTextAlign::Right=>DTS_RIGHTALIGN 
        };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a date time picker must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) =>    
                match handle_of_font(ui, &font_id, "The font of a date time picker must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: "",
            class_name: "SysDateTimePick32",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ 
                    set_window_font(h, font_handle, true); 
                }
                let dtp = DatePicker{handle: h};
                dtp.set_format(&self.format);
                dtp.set_value(&self.value);
                dtp.set_align(&self.align);
                Ok( Box::new(dtp) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A simple date picker control. Do not handle timezone. It is recomended to use a specialized date time library to
    use with this control.
*/
pub struct DatePicker {
    handle: HWND,
}

impl DatePicker {

    /// Set the format of the date time picker. Use the format specified in the DTP template.
    pub fn set_format<S: Clone+Into<String>>(&self, format: &S) {
        use winapi::{DTM_SETFORMATW, LPARAM};
        unsafe{
            let format = to_utf16(format.clone().into().as_str());
            SendMessageW(self.handle, DTM_SETFORMATW, 0, format.as_ptr() as LPARAM);
        }
    }

    /**
        Return the check state of the checkbox of the control (if optional was set to true).  
        If the date time picker is not optional, return false.
    */
    pub fn get_checkstate(&self) -> CheckState {
        use low::defs::STATE_SYSTEM_CHECKED;
        let info = unsafe{ get_dtp_info(self.handle) };

        match info.stateCheck {
            STATE_SYSTEM_CHECKED => CheckState::Checked,
            _ => CheckState::Unchecked
        }
    }

    /**
        Return the time set in the control in a `PickerDate` structure.  
        Return None if `optional` was set and the checkbox is not checked.  
        Note: use `get_date_string` to get the text value of the control.
    */
    pub fn get_value(&self) -> Option<PickerDate> {
        use winapi::{DTM_GETSYSTEMTIME, GDT_VALID};
        let mut syst: SYSTEMTIME = unsafe{ mem::uninitialized() };

        let r = unsafe{ SendMessageW(self.handle, DTM_GETSYSTEMTIME, 0, mem::transmute(&mut syst)) };
        match r {
            GDT_VALID => Some(PickerDate{
                year: syst.wYear,
                month: syst.wMonth,
                day: syst.wDay
            }),
            _ => None
        }
    }

    /**
        Set the time set in the control in a `PickerDate` structure.  
        If `None` is passed, clears the checkbox.
    */
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

    /**
        Get the alignment of the calendar popup. 
        For some reason, it is impossible to retrive the TextEdit handle, therefore getting its alignment is impossible.
    */
    pub fn get_align(&self) -> HTextAlign {
        use low::window_helper::get_window_long;
        use winapi::{GWL_STYLE, DTS_RIGHTALIGN};

        let style = get_window_long(self.handle, GWL_STYLE) as u32;

        if (style & DTS_RIGHTALIGN) == DTS_RIGHTALIGN {
            HTextAlign::Right
        } else {
            HTextAlign::Left
        }
    }

    /**
       Change the alignment of the calendar popup. 
       For some reason, it is impossible to retrive the TextEdit handle, therefore setting its alignment is impossible.
    */
    pub fn set_align(&self, align: &HTextAlign) {
         use low::window_helper::{set_window_long, get_window_long};
         use winapi::{GWL_STYLE, DTS_RIGHTALIGN};

         // Set the calendar dropdown align
         let mut old_style = get_window_long(self.handle, GWL_STYLE) as usize;
         let right_align = DTS_RIGHTALIGN as usize;
         old_style = (old_style | right_align) ^ right_align ;

         if *align == HTextAlign::Right {
            set_window_long(self.handle, GWL_STYLE, old_style|right_align);
         } else {
            set_window_long(self.handle, GWL_STYLE, old_style);
         }
    }

    /// Close the calendar popup if it is open  
    pub fn close_calendar(&self) {
        use winapi::{DTM_CLOSEMONTHCAL};
        unsafe{ SendMessageW(self.handle, DTM_CLOSEMONTHCAL, 0, 0); }
    }

    pub fn get_value_string(&self) -> String { unsafe{ ::low::window_helper::get_window_text(self.handle) } }
    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, false); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
}

impl Control for DatePicker {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::DatePicker 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}

// Private functions

#[inline(always)]
unsafe fn get_dtp_info(handle: HWND) -> DATETIMEPICKERINFO {
    use winapi::{DTM_GETDATETIMEPICKERINFO, DWORD};
    let mut dtp_info: DATETIMEPICKERINFO = mem::uninitialized();
    dtp_info.cbSize = mem::size_of::<DATETIMEPICKERINFO>() as DWORD;
    SendMessageW(handle, DTM_GETDATETIMEPICKERINFO, 0, mem::transmute(&mut dtp_info));

    dtp_info
}