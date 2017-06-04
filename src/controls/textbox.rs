/*!
    A simple text box control

    This is basically a multiline text input but I want to differenciate the two controls
    because they have a few difference and I want to keep each control in its own file with
    no superclass.
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

use winapi::{HWND, HFONT, WPARAM};
use user32::SendMessageW;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::{Event, Destroyed, Char, KeyUp, KeyDown, MouseDown, MouseUp, Moved, Resized};
use events::textbox::{Focus, ValueChanged};

/**
    A template that creates a multi line textinput control

    Events:  
    `Destroyed, Char, KeyDown, KeyUp, MouseDown, MouseUp, Moved, Resized, ValueChanged, Focus, Any`  

    Members:  
    • `text`: The text of the textbox  
    • `position`: The start position of the textbox  
    • `size`: The start size of the textbox  
    • `visible`: If the textbox should be visible to the user   
    • `disabled`: If the user can or can't click on the textbox  
    • `readonly`: If the user can copty the text but can't edit the textbox content  
    • `limit`: The maximum number of characters that the control can hold  
    • `scrollbars`: A tuple to defined whether to show scrollbars or not (show horizontal, show vertical)
    • `parent`: The textbox parent  
    • `font`: The textbox font. If None, use the system default  
*/
#[derive(Clone)]
pub struct TextBoxT<S1: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S1,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub limit: u32,
    pub scrollbars: (bool, bool),
    pub parent: ID,
    pub font: Option<ID>,
}

impl<S1: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for TextBoxT<S1, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<TextBox>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, Char, KeyDown, KeyUp, MouseDown, MouseUp, Moved, Resized, ValueChanged, Focus, Event::Any]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font, handle_of_window, handle_of_font};
        use low::defs::{ES_AUTOHSCROLL, ES_AUTOVSCROLL, ES_READONLY, EM_LIMITTEXT, ES_MULTILINE};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, WS_BORDER, WS_HSCROLL, WS_VSCROLL};

        let flags: DWORD = WS_CHILD | WS_BORDER | ES_AUTOHSCROLL | ES_MULTILINE | ES_AUTOVSCROLL |
        if self.readonly { ES_READONLY } else { 0 } |
        if self.visible  { WS_VISIBLE }  else { 0 } |
        if self.scrollbars.0 { WS_HSCROLL } else { 0 } |
        if self.scrollbars.1 { WS_VSCROLL } else { 0 } |
        if self.disabled { WS_DISABLED } else { 0 };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a textinput must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a button must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: self.text.clone().into(),
            class_name: "EDIT",
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
                    SendMessageW(h, EM_LIMITTEXT, self.limit as WPARAM, 0);
                };

                Ok( Box::new(TextBox{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A multi line textinput control
*/
pub struct TextBox {
    handle: HWND
}

impl TextBox {

    /// Set or unset the readonly status on the control
    pub fn set_readonly(&self, readonly: bool) {
        use low::window_helper::{set_window_long, get_window_long};
        use low::defs::ES_READONLY;
        use winapi::GWL_STYLE;

        let old_style = get_window_long(self.handle, GWL_STYLE) as usize;
        if readonly {
            set_window_long(self.handle, GWL_STYLE, old_style|(ES_READONLY as usize));
        } else {
            set_window_long(self.handle, GWL_STYLE, old_style&(!ES_READONLY as usize) );
        }
    }
    
    /// Return `true` if the user cannot edit the content of the control or `false` if the user can
    pub fn get_readonly(&self) -> bool {
        use low::window_helper::get_window_long;
        use low::defs::ES_READONLY;
        use winapi::GWL_STYLE;

        let style = get_window_long(self.handle, GWL_STYLE) as u32;

        (style & ES_READONLY) == ES_READONLY
    }

    /// Set the maximum number of characters that the control can hold
    pub fn set_limit(&self, limit: u32) {
        use low::defs::EM_LIMITTEXT;
        unsafe{ SendMessageW(self.handle, EM_LIMITTEXT, limit as WPARAM, 0); }
    }

    /// Return the maximum number of characters that the control can hold
    pub fn get_limit(&self) -> u32 {
        use low::defs::EM_GETLIMITTEXT;
        unsafe{ SendMessageW(self.handle, EM_GETLIMITTEXT, 0, 0) as u32 }
    }

    pub fn get_text(&self) -> String { unsafe{ ::low::window_helper::get_window_text(self.handle) } }
    pub fn set_text<'a>(&self, text: &'a str) { unsafe{ ::low::window_helper::set_window_text(self.handle, text); } }
    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, false); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
}

impl Control for TextBox {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::TextBox 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}