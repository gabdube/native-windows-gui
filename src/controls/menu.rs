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

use winapi::{HMENU, UINT};

use ui::Ui;
use controls::{Control, ControlT, AnyHandle};
use error::Error;
use events::Event;

static mut menu_items_id: UINT = 0; 

/**
    A template to create menu controls

    Members:  
      • `text` : The menu text
      • `parent` : ID of the parent control to add the menu to
*/
pub struct MenuT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub parent: ID,
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for MenuT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<Menu>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        let handle_result = unsafe { build_menu(ui, self) };
        match handle_result {
            Ok((h, parent)) => { Ok( Box::new(Menu{handle: h, parent: parent}) as Box<Control> ) },
            Err(e) => Err(e)
        }
    }
}

/**
    A menu control
*/
pub struct Menu {
    handle: HMENU,
    parent: AnyHandle
}

impl Control for Menu {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HMENU(self.handle)
    }

    fn free(&mut self) {
        use user32::DestroyMenu;
        use low::menu_helper::remove_menu_from_parent;

        unsafe{ remove_menu_from_parent(self.handle, &self.parent) };

        unsafe{ DestroyMenu(self.handle) };
    }

}


/**
    A template to create menuitems

    Members:  
      • `text` : The menu text
      • `parent` : ID of the parent control to add the menu to
*/
pub struct MenuItemT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub parent: ID,
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for MenuItemT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<MenuItem>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed, Event::Clicked]
    }

   #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        let handle_result = unsafe { build_menu_item(ui, self) };
        match handle_result {
            Ok((parent, uid)) => { Ok( Box::new(MenuItem{parent: parent, unique_id: uid}) as Box<Control> ) },
            Err(e) => Err(e)
        }
    }
}

/**
    A menu item control
*/ 
pub struct MenuItem {
    parent: HMENU,
    unique_id: UINT
}


/**
    A menu item control
*/
impl Control for MenuItem {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HMENU_ITEM(self.parent, self.unique_id)
    }

    fn free(&mut self) {
        use low::menu_helper::remove_menu_item_from_parent;
        unsafe{ remove_menu_item_from_parent(self.parent, self.unique_id) };
    }
}

/*
    Private unsafe menu/menuitem methods
*/

#[inline(always)]
unsafe fn build_menu<S: Clone+Into<String>, ID: Clone+Hash>(ui: &Ui<ID>, t: &MenuT<S, ID>) -> Result<(HMENU, AnyHandle), Error> {
    use user32::{CreateMenu, AppendMenuW, GetMenu, SetMenu, DrawMenuBar};
    use winapi::{MF_STRING, MF_POPUP};
    use low::menu_helper::use_menu_command;
    use low::other_helper::to_utf16;

    let ph_result = ui.handle_of(&t.parent);
    if ph_result.is_err() { return Err(ph_result.err().unwrap()); }

    match ph_result.unwrap() {
        AnyHandle::HWND(parent_h) => {
            let mut menubar = GetMenu(parent_h);
            if menubar.is_null() {
                // If the window do not have a menu bar, create one
                menubar = CreateMenu();
                use_menu_command(menubar);
                SetMenu(parent_h, menubar);
            }

            let h = CreateMenu();
            use_menu_command(h);

            let text = to_utf16(t.text.clone().into().as_ref());
            AppendMenuW(menubar, MF_STRING|MF_POPUP, mem::transmute(h), text.as_ptr());

            DrawMenuBar(parent_h); // Draw the menu bar to make sure the changes are visible
            Ok( ( h, AnyHandle::HWND(parent_h)) )
        },
        AnyHandle::HMENU(parent_h) => {
            let h = CreateMenu();
            use_menu_command(h);

            let text = to_utf16(t.text.clone().into().as_ref());
            AppendMenuW(parent_h, MF_STRING|MF_POPUP, mem::transmute(h), text.as_ptr());
            Ok( ( h, AnyHandle::HMENU(parent_h) ) )
        },
        AnyHandle::HMENU_ITEM(_, _) => Err(Error::BadParent("Window or menu parent required, got MenuItem".to_string())),
        AnyHandle::HFONT(_) =>  Err(Error::BadParent("Window or menu parent required, got Font".to_string())),
   }
}

#[inline(always)]
unsafe fn build_menu_item<S: Clone+Into<String>, ID: Clone+Hash>(ui: &Ui<ID>, t: &MenuItemT<S, ID>) -> Result<(HMENU, UINT), Error> {
    use user32::{AppendMenuW, CreateMenu, GetMenu, SetMenu, DrawMenuBar};
    use winapi::{MF_STRING, UINT_PTR};
    use low::other_helper::to_utf16;

    let ph_result = ui.handle_of(&t.parent);
    if ph_result.is_err() { return Err(ph_result.err().unwrap()); }

    match ph_result.unwrap() {
        AnyHandle::HWND(parent_h) => {
            let mut menubar = GetMenu(parent_h);
            if menubar.is_null() {
                // If the window do not have a menu bar, create one
                menubar = CreateMenu();
                SetMenu(parent_h, menubar);
            }

            let text = to_utf16(t.text.clone().into().as_ref());
            menu_items_id += 1;
            AppendMenuW(menubar, MF_STRING, menu_items_id as UINT_PTR, text.as_ptr());

            DrawMenuBar(parent_h); // Draw the menu bar to make sure the changes are visible
        
            Ok( (menubar, menu_items_id) )
        },
        AnyHandle::HMENU(parent_h) => {
            let text = to_utf16(t.text.clone().into().as_ref());
            menu_items_id += 1;
            AppendMenuW(parent_h, MF_STRING, menu_items_id as UINT_PTR, text.as_ptr());
            Ok( (parent_h, menu_items_id) )
        },
        AnyHandle::HMENU_ITEM(_, _) => Err(Error::BadParent("Window or menu parent required, got MenuItem".to_string())),
        AnyHandle::HFONT(_) =>  Err(Error::BadParent("Window or menu parent required, got Font".to_string())),
    }
}