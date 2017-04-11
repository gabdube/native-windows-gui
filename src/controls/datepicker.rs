/*!
    Date time picker control definition
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

use winapi::{HWND, HFONT};
use user32::SendMessageW;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::Event;
use defs::HTextAlign;
use low::other_helper::to_utf16;

/**
    A template that creates a standard date picker (dtp)  

    About the format string:  
    "d" 	The one- or two-digit day.  
    "dd" 	The two-digit day. Single-digit day values are preceded by a zero.  
    "ddd" 	The three-character weekday abbreviation.  
    "dddd" 	The full weekday name.  
    "h" 	The one- or two-digit hour in 12-hour format.  
    "hh" 	The two-digit hour in 12-hour format. Single-digit values are preceded by a zero.  
    "H" 	The one- or two-digit hour in 24-hour format.  
    "HH" 	The two-digit hour in 24-hour format. Single-digit values are preceded by a zero.  
    "m" 	The one- or two-digit minute.  
    "mm" 	The two-digit minute. Single-digit values are preceded by a zero.  
    "M" 	The one- or two-digit month number.  
    "MM" 	The two-digit month number. Single-digit values are preceded by a zero.  
    "MMM" 	The three-character month abbreviation.  
    "MMMM" 	The full month name.  
    "t" 	The one-letter AM/PM abbreviation (that is, AM is displayed as "A").  
    "tt" 	The two-letter AM/PM abbreviation (that is, AM is displayed as "AM").  
    "yy" 	The last two digits of the year (that is, 1996 would be displayed as "96").  
    "yyyy" 	The full year (that is, 1996 would be displayed as "1996").   

    Furthermore, any string enclosed in `'` can be used in the format to display text.  
    For example, to display the current date with the format `'Today is: 04:22:31 Tuesday Mar 23, 1996`, the format string is `'Today is: 'hh':'m':'s dddd MMM dd', 'yyyy`. 

    Members:  
    • `value`: The value of the dtp, must match `format`. If left empty and `optional` is false, use the current date.
    • `position`: The start position of the dtp  
    • `size`: The start size of the dtp  
    • `visible`: If the dtp should be visible to the user   
    • `disabled`: If the user can or can't edit the value of the dtp  
    • `parent`: The dtp parent  
    • `font`: The dtp font. If None, use the system default  
    • `format`: The dtp format string. See the docs above for the available formats. If left empty, use the system locale date format.  
    • `optional`: If the dtp must contain a value (or not)  
*/
#[derive(Clone)]
pub struct DatePickerT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub value: S,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID,
    pub font: Option<ID>,
    pub align: HTextAlign,
    pub format: S,
    pub optional: bool
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for DatePickerT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<DatePicker>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed, Event::Moved, Event::Resized, Event::Raw]
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
                    set_dtp_format(h, &self.format);
                }
                Ok( Box::new(DatePicker{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

pub struct DatePicker {
    handle: HWND
}

impl DatePicker {

    /// Set the format of the date time picker. Use the format specified in the DTP template.
    pub fn set_format<S: Clone+Into<String>>(&self, format: &S) {
        unsafe{ set_dtp_format(self.handle, format); }
    }

    /// Close the calendar popup if it is open  
    pub fn close_calendar(&self) {
        use winapi::{DTM_CLOSEMONTHCAL, LPARAM};
        unsafe{ SendMessageW(self.handle, DTM_CLOSEMONTHCAL, 0, 0); }
    }

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
unsafe fn set_dtp_format<S: Clone+Into<String>>(handle: HWND, format: &S) {
    use winapi::{DTM_SETFORMATW, LPARAM};
    let format = to_utf16(format.clone().into().as_str());
    SendMessageW(handle, DTM_SETFORMATW, 0, format.as_ptr() as LPARAM);
}