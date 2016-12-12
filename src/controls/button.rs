/*!
    Button control definition
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
use controls::{Control, ControlT, AnyHandle};
use error::Error;
use events::Event;

#[derive(Clone)]
pub struct ButtonT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID,
    pub font: Option<ID>,
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for ButtonT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<Button>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed, Event::Clicked]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, BS_NOTIFY};

        let flags: DWORD = WS_CHILD | BS_NOTIFY |
        if self.visible    { WS_VISIBLE }   else { 0 } |
        if self.disabled   { WS_DISABLED }  else { 0 };

        // Get the parent handle
        let parent: HWND = match ui.handle_of(&self.parent) {
            Ok(AnyHandle::HWND(h)) => h,
            Ok(_) => { return Err(Error::BadParent("The parent of a button must be a window-like control.".to_string()) ); },
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match ui.handle_of(font_id) {
                    Ok(AnyHandle::HFONT(h)) => Some(h),
                    Ok(_) => { return Err(Error::BadResource("The font value of a button must be a font resource.".to_string()) ); },
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: self.text.clone().into(),
            class_name: "BUTTON",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ set_window_font(h, font_handle, true); }
                Ok( Box::new(Button{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

pub struct Button {
    handle: HWND
}

impl Control for Button {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}