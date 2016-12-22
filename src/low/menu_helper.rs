/*!
    Low level menu helping functions
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
use std::ptr;
use std::mem;
use std::hash::Hash;

use winapi::{HMENU, DWORD, HBRUSH, c_int, UINT};

use ui::UiInner;
use controls::AnyHandle;

/**
    List the children of a menu and return a list of their IDs. The function is recursive and so 
    it list the ids for the whole menu tree.
*/
pub unsafe fn list_menu_children<ID: Hash+Clone>(ui: &UiInner<ID>, menu: HMENU) -> Vec<u64> { 
    use low::defs::{GetMenuItemCount, GetSubMenu, GetMenuItemID};

    let mut children = Vec::new();
    let children_count = GetMenuItemCount(menu);

    for i in 0..children_count {
        let sub_menu = GetSubMenu(menu, i as c_int);
        if sub_menu.is_null() {
            // Get a menu item ID
            let handle = AnyHandle::HMENU_ITEM(menu, GetMenuItemID(menu, i));
            children.push( ui.inner_id_from_handle(&handle) );
        } else {
            // Get the menu ID
            children.push( ui.inner_id_from_handle(&AnyHandle::HMENU(sub_menu)) );
            children.append( &mut list_menu_children(ui, sub_menu) );
        }
    }

    children
}

/**
    Return the index of a children menu/menuitem in a parent menu.
*/
#[inline(always)]
pub unsafe fn menu_index_in_parent(h: HMENU, parent_h: HMENU) -> UINT {
    use low::defs::{GetMenuItemCount, GetSubMenu};

    let children_count = GetMenuItemCount(parent_h);
    let mut sub_menu: HMENU;

    for i in 0..children_count {
        sub_menu = GetSubMenu(parent_h, i as c_int);
        if sub_menu.is_null() { continue; }
        else if sub_menu == h { return i as UINT; }
    }

    panic!("Menu/MenuItem not found in parent!")
}

/**
    Remove a submenu from its parent.
*/
pub unsafe fn remove_menu_from_parent(h: HMENU, parent: &AnyHandle) {
    use user32::{GetMenu, DrawMenuBar};
    use low::defs::{RemoveMenu, MF_BYPOSITION};

    match parent {
        &AnyHandle::HWND(parent_h) => {
            let menubar = GetMenu(parent_h);
            if menubar.is_null() { panic!("Tried to remove a menu from its window parent, but the parent do not have a menubar.") }

            let index = menu_index_in_parent(h, menubar);
            RemoveMenu(menubar, index, MF_BYPOSITION);
            DrawMenuBar(parent_h);
        },
        &AnyHandle::HMENU(parent_h) => {
            let index = menu_index_in_parent(h, parent_h);
            RemoveMenu(parent_h, index, MF_BYPOSITION);
        }
        _ => { unreachable!(); /* A menu can only be added to another menu or a window */ }
    }
}

/**
    Remove a menu item from its parent.
*/
pub unsafe fn remove_menu_item_from_parent(parent_h: HMENU, uid: UINT) {
    use low::defs::RemoveMenu;
    RemoveMenu(parent_h, uid, 0);
}

/**
    Configure the menu to use a WM_MENUCOMMAND instead of a WM_COMMAND when its action are triggered.
    Required in order to allow nwg to dispatch the events correctly
*/
#[inline(always)]
pub fn use_menu_command(h: HMENU) {
    use low::defs::{MENUINFO, MNS_NOTIFYBYPOS, MIM_STYLE, SetMenuInfo};

    let mut info = MENUINFO {
        cbSize: mem::size_of::<MENUINFO>() as DWORD,
        fMask: MIM_STYLE,
        dwStyle: MNS_NOTIFYBYPOS,
        cyMax: 0,
        hbrBack: unsafe{ mem::transmute(ptr::null_mut::<HBRUSH>()) } ,
        dwContextHelpID: 0,
        dwMenuData: 0
    };

    unsafe{ SetMenuInfo(h, &mut info) };
}

/**
    Used in the events proc to return the inner id of a menuitem when triggering the clicked action
*/
#[inline(always)]
pub unsafe fn get_menu_id(parent_h: HMENU, index: c_int) -> UINT {
    ::low::defs::GetMenuItemID(parent_h, index)
}