/*!
    A simple text input control
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

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::Event;

/**
    A template that creates a single line textinput control
*/
#[derive(Clone)]
pub struct TextInputT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub password: bool,
    pub parent: ID,
    pub font: Option<ID>,
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for TextInputT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<TextInput>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font, handle_of_window, handle_of_font};
        use low::defs::{ES_AUTOHSCROLL, ES_READONLY, ES_PASSWORD};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, WS_BORDER};

        let flags: DWORD = WS_CHILD | WS_BORDER | ES_AUTOHSCROLL | 
        if self.readonly { ES_READONLY } else { 0 } |
        if self.password { ES_PASSWORD } else { 0 } |
        if self.visible  { WS_VISIBLE }  else { 0 } |
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
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ set_window_font(h, font_handle, true); }
                Ok( Box::new(TextInput{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A single line textinput control
*/
pub struct TextInput {
    handle: HWND
}

impl TextInput {

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
    
    pub fn get_readonly(&self) -> bool {
        use low::window_helper::get_window_long;
        use low::defs::ES_READONLY;
        use winapi::GWL_STYLE;

        let style = get_window_long(self.handle, GWL_STYLE) as u32;

        (style & ES_READONLY) == ES_READONLY
    }

    pub fn set_password(&self, password: bool) {
        use low::window_helper::{set_window_long, get_window_long};
        use low::defs::ES_PASSWORD;
        use winapi::GWL_STYLE;

        let old_style = get_window_long(self.handle, GWL_STYLE) as usize;
        if password {
            set_window_long(self.handle, GWL_STYLE, old_style|(ES_PASSWORD as usize));
        } else {
            set_window_long(self.handle, GWL_STYLE, old_style&(!ES_PASSWORD as usize) );
        }
    }

    pub fn get_password(&self) -> bool {
        use low::window_helper::get_window_long;
        use low::defs::ES_PASSWORD;
        use winapi::GWL_STYLE;

        let style = get_window_long(self.handle, GWL_STYLE) as u32;

        (style & ES_PASSWORD) == ES_PASSWORD
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

impl Control for TextInput {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::TextInput 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}