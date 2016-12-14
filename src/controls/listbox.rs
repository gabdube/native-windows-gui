/*!
    Simple listbox control definition
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
use std::fmt::Display;

use winapi::{HWND, HFONT};

use ui::Ui;
use controls::{Control, ControlT, AnyHandle};
use error::Error;
use events::Event;


#[derive(Clone)]
pub struct ListBoxT<D: Clone+Display+'static, ID: Hash+Clone> {
    pub collection: Vec<D>,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID,
    pub font: Option<ID>,
}

impl<D: Clone+Display+'static, ID: Hash+Clone> ControlT<ID> for ListBoxT<D, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<ListBox<D>>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font, handle_of_window, handle_of_font};
        use low::other_helper::to_utf16;
        use low::defs::{LB_ADDSTRING, LBS_HASSTRINGS};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, WS_BORDER};
        use user32::{SendMessageW};

        let flags: DWORD = WS_CHILD | WS_BORDER | LBS_HASSTRINGS |
        if self.visible    { WS_VISIBLE }   else { 0 } |
        if self.disabled   { WS_DISABLED }  else { 0 };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a listbox must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a listbox must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: "",
            class_name: "LISTBOX",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ 
                    set_window_font(h, font_handle, true); 
                    let collection: Vec<D> = self.collection.iter().map(
                        |s|{  
                            let text = to_utf16(format!("{}", s).as_str());
                            SendMessageW(h, LB_ADDSTRING, 0, ::std::mem::transmute(text.as_ptr()));
                            s.clone() 
                        } 
                    ).collect();
                    Ok( Box::new(ListBox{handle: h, collection: collection}) )
                }
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A standard button
*/
pub struct ListBox<D: Clone+Display> {
    handle: HWND,
    collection: Vec<D>
}

impl<D: Clone+Display> Control for ListBox<D> {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}