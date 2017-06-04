/*!
    Radio button control definition
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
use events::{Event, Destroyed, Moved, Resized};
use events::radiobutton::{Click, DoubleClick, Focus};
use defs::CheckState;

/**
    A template that creates a standard radio button

    Events:  
    `Destroyed, Moved, Resized, Click, DoubleClick, Focus, Any`

    Members:  
    • `text`: The text of the radio button  
    • `position`: The start position of the radio button  
    • `size`: The start size of the radio button  
    • `visible`: If the radio button should be visible to the user32  
    • `disabled`: If the user can or can't click on the radio button  
    • `parent`: The radio button parent  
    • `checkstate`: The starting checkstate  
    • `tristate`: If the radio button should have three states  
    • `font`: The radio button font. If None, use the system default  
*/
#[derive(Clone)]
pub struct RadioButtonT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID,
    pub checkstate: CheckState,
    pub font: Option<ID>,
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for RadioButtonT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<RadioButton>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, Moved, Resized, Click, DoubleClick, Focus, Event::Any]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font, handle_of_window, handle_of_font};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, BS_NOTIFY, BS_AUTORADIOBUTTON, BS_TEXT};

        let flags: DWORD = WS_CHILD | BS_NOTIFY | BS_TEXT | BS_AUTORADIOBUTTON |
        if self.visible    { WS_VISIBLE }   else { 0 } |
        if self.disabled   { WS_DISABLED }  else { 0 } ;

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a checkbox must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a checkbox must be a font resource.") {
                    Ok(h) => Some(h),
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
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ 
                    set_window_font(h, font_handle, true); 
                    set_checkstate(h, &self.checkstate);
                }
                Ok( Box::new(RadioButton{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A standard radio button
*/
pub struct RadioButton {
    handle: HWND
}

impl RadioButton {

    /**
        Get the checkstate of the radio button
    */
    pub fn get_checkstate(&self) -> CheckState {
        use low::defs::{BM_GETCHECK, BST_CHECKED};
        match unsafe{ SendMessageW(self.handle, BM_GETCHECK, 0 , 0) as u32 } {
            BST_CHECKED => CheckState::Checked,
            _ => CheckState::Unchecked
        }
    }

    /**
        Set the checkstate of the radio button. A radio button control cannot have an `Indeterminate` check state.
        If one is passed, it is evaluated as checked.
    */
    pub fn set_checkstate(&self, check: CheckState) {
        let check = if check == CheckState::Indeterminate { CheckState::Checked } else { check }; 
        unsafe{ set_checkstate(self.handle, &check); }
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

impl Control for RadioButton {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::RadioButton 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}

/// Private checkbox methods

#[inline(always)]
unsafe fn set_checkstate(handle: HWND, check: &CheckState) {
    use low::defs::{BM_SETCHECK, BST_CHECKED, BST_INDETERMINATE, BST_UNCHECKED};
    use user32::SendMessageW;
    use winapi::WPARAM;

    let check_state = match check {
        &CheckState::Checked => BST_CHECKED,
        &CheckState::Indeterminate => BST_INDETERMINATE,
        &CheckState::Unchecked => BST_UNCHECKED
    };
    SendMessageW(handle, BM_SETCHECK, check_state as WPARAM, 0);
}