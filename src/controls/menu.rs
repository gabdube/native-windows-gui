/*!
    Menu control definition that integrates with the built-in window type
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

use winapi::HMENU;

use ui::Ui;
use controls::{Control, ControlT, AnyHandle};
use error::Error;
use events::Event;

/**
    A template to create menu controls
*/
pub struct MenuT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub parent: Option<ID>,
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for MenuT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<Menu>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        let handle_result = unsafe { build_menu(ui, self) };
        match handle_result {
            Ok(h) => { Ok( Box::new(Menu{handle: h}) as Box<Control> ) },
            Err(e) => Err(e)
        }
    }
}

/**
    A menu control
*/
#[allow(dead_code)]
pub struct Menu {
    handle: HMENU,
}

impl Control for Menu {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HMENU(self.handle)
    }

    fn free(&mut self) {
        use user32::DestroyMenu;
        unsafe{ DestroyMenu(self.handle) };
    }

}

/*
    Private unsafe menu methods
*/

#[inline(always)]
unsafe fn build_menu<S: Clone+Into<String>, ID: Clone+Hash>(ui: &Ui<ID>, t: &MenuT<S, ID>) -> Result<HMENU, Error> {
    use user32::{CreatePopupMenu, CreateMenu, AppendMenuW};
    use winapi::{MF_STRING, MF_POPUP};
    use low::menu_helper::use_menu_command;
    use low::other_helper::to_utf16;

    match t.parent.as_ref() {
        Some(p) => {
            let ph_result = ui.handle_of(p);
            if ph_result.is_err() { return Err(ph_result.err().unwrap()); }

            match ph_result.unwrap() {
                AnyHandle::HWND(_) => Err(Error::Unimplemented),
                AnyHandle::HMENU(parent_h) => {
                    let h = CreateMenu();
                    let text = to_utf16(t.text.clone().into().as_ref());
                    AppendMenuW(parent_h, MF_STRING|MF_POPUP, mem::transmute(h), text.as_ptr());
                    Ok(h)
                }
            }
        },
        None => {
            // Create a popup menu without parents, t.text is ignored
            let h = CreatePopupMenu();
            use_menu_command(h);
            Ok(h)
        }
    }
}