/*!
    Combobox control definition
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
use std::mem;

use winapi::{HWND, HFONT};

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::Event;
use low::other_helper::{to_utf16};

/**
    Template that creates a combobox control

    Members:  
    • `collection`: Item collection of the combobox. The item type must implement `Display`  
    • `position`: The start position of the combobox  
    • `size`: The start size of the combobox  
    • `visible`: If the combobox should be visible to the user  
    • `disabled`: If the user can or can't use the combobox   
    • `parent`: The combobox parent  
    • `font`: The combobox font. If None, use the system default  
*/
#[derive(Clone)]
pub struct ComboBoxT<D: Clone+Display+'static, ID: Hash+Clone> {
    pub collection: Vec<D>,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID,
    pub font: Option<ID>,
}

impl<D: Clone+Display+'static, ID: Hash+Clone> ControlT<ID> for ComboBoxT<D, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<ComboBox<D>>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed, Event::SelectionChanged, Event::DoubleClick, Event::Focus]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font, handle_of_window, handle_of_font};
        use low::defs::{CBS_DROPDOWNLIST, CBS_HASSTRINGS, CB_ADDSTRING};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD};
        use user32::SendMessageW;

        let flags: DWORD = WS_CHILD | CBS_HASSTRINGS | CBS_DROPDOWNLIST |
        if self.visible      { WS_VISIBLE }      else { 0 } |
        if self.disabled     { WS_DISABLED }     else { 0 };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a combobox must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a combobox must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: "",
            class_name: "COMBOBOX",
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
                            SendMessageW(h, CB_ADDSTRING, 0, mem::transmute(text.as_ptr()));
                            s.clone() 
                        } 
                    ).collect();
                    Ok( Box::new(ComboBox{handle: h, collection: collection}) )
                }
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A combobox control
*/
pub struct ComboBox<D: Clone+Display> {
    handle: HWND,
    collection: Vec<D>
}

impl<D: Clone+Display> Control for ComboBox<D> {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::ComboBox 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }
}