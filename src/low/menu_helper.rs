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

use winapi::{HMENU, DWORD, HBRUSH, c_int, UINT, BOOL};

use ui::UiInner;
use controls::AnyHandle;

/**
    List the children of a menu and return a list of their IDs. The function is recursive and so 
    it list the ids for the whole menu tree.
*/
pub unsafe fn list_menu_children<ID: Hash+Clone>(ui: &UiInner<ID>, menu: HMENU) -> Vec<u64> { 
    use low::defs::{GetMenuItemCount, GetSubMenu, GetMenuItemID};

    let mut children: Vec<u64> = Vec::new();
    let children_count = GetMenuItemCount(menu);

    for i in 0..children_count {
        let sub_menu = GetSubMenu(menu, i as c_int);
        if sub_menu.is_null() {
            // Get a menu item ID
            let handle = AnyHandle::HMENU_ITEM(menu, GetMenuItemID(menu, i));
            let id = ui.inner_id_from_handle(&handle) .expect("Could not match menu handle to menu control");
            children.push( id );
        } else {
            // Get the menu ID
            let handle = AnyHandle::HMENU(sub_menu);
            let id = ui.inner_id_from_handle(&handle).expect("Could not match menu handle to menu control");
            children.push( id );
            children.append( &mut list_menu_children(ui, sub_menu) );
        }
    }

    children
}


/**
    Return the parent handle of a menu.
*/
#[inline(always)]
unsafe fn resolve_menu_parent(parent: &AnyHandle) -> HMENU {
    use user32::GetMenu;
    match parent {
        &AnyHandle::HWND(parent_h) => {
            let menubar = GetMenu(parent_h);
            if menubar.is_null() { panic!("Tried to resolve a menu parent, but the parent window do not have a menubar.") }
            menubar
        },
        &AnyHandle::HMENU(parent_h) => parent_h,
        _ => { unreachable!(); /* A menu can only be added to another menu or a window */ }
    }
}

/**
    Return the index of a children menu/menuitem in a parent menu.
    Panic if the menu is not found in the parent.
*/
#[inline(always)]
pub unsafe fn menu_index_in_parent(h: HMENU, parent: &AnyHandle) -> UINT {
    use low::defs::{GetMenuItemCount, GetSubMenu};

    let parent_h = resolve_menu_parent(parent);
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

    let index = menu_index_in_parent(h, parent);

    match parent {
        &AnyHandle::HWND(parent_h) => {
            let menubar = GetMenu(parent_h);
            RemoveMenu(menubar, index, MF_BYPOSITION);
            DrawMenuBar(parent_h);
        },
        &AnyHandle::HMENU(parent_h) => {
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
pub unsafe fn use_menu_command(h: HMENU) {
    use low::defs::{MENUINFO, MNS_NOTIFYBYPOS, MIM_STYLE, SetMenuInfo};

    let mut info = MENUINFO {
        cbSize: mem::size_of::<MENUINFO>() as DWORD,
        fMask: MIM_STYLE,
        dwStyle: MNS_NOTIFYBYPOS,
        cyMax: 0,
        hbrBack: mem::transmute(ptr::null_mut::<HBRUSH>()),
        dwContextHelpID: 0,
        dwMenuData: 0
    };

    SetMenuInfo(h, &mut info);
}


/**
    Enable or disable a menuitem at the selected position or using the selected ID. If the position is None and id is None, the last item is selected.
*/
#[inline(always)]
pub unsafe fn enable_menuitem(h: HMENU, pos: Option<UINT>, id: Option<UINT>, enabled: bool) {
    use winapi::MENUITEMINFOW;
    use low::defs::{SetMenuItemInfoW, GetMenuItemCount, MIIM_STATE, MFS_DISABLED, MFS_ENABLED};
    
    let use_position = id.is_none();
    let choice = if use_position { pos } else { id };
    let value = match choice {
        Some(p) => p,
        None => (GetMenuItemCount(h) - 1) as u32
    };

    let state = match enabled {
         true => MFS_ENABLED,
         false => MFS_DISABLED
    };

    let mut info = MENUITEMINFOW { 
        cbSize: mem::size_of::<MENUITEMINFOW>() as UINT,
        fMask: MIIM_STATE, fType: 0, fState: state,
        wID: 0, hSubMenu: ptr::null_mut(), hbmpChecked: ptr::null_mut(),
        hbmpUnchecked: ptr::null_mut(), dwItemData: 0, dwTypeData: ptr::null_mut(),
        cch: 0, hbmpItem: ptr::null_mut()
    };

    SetMenuItemInfoW(h, value, use_position as BOOL, &mut info);
}

/**
    Enable or disable a menu at the selected position or using the selected ID. If the position is None and id is None, the last item is selected.
*/
#[inline(always)]
pub unsafe fn enable_menu(menu: HMENU, parent: &AnyHandle, enabled: bool) {
    let parent_h = resolve_menu_parent(parent);
    let index = menu_index_in_parent(menu, parent);
    enable_menuitem(parent_h, Some(index), None, enabled);
}

/**
    Return the state of a menuitem. Panic if both pos and id are None.
*/
#[inline(always)]
pub unsafe fn is_menuitem_enabled(h: HMENU, pos: Option<UINT>, id: Option<UINT>) -> bool {
    use winapi::MENUITEMINFOW;
    use low::defs::{GetMenuItemInfoW, MIIM_STATE, MFS_DISABLED};

    if id.is_none() && pos.is_none() { panic!("Both pos and id are None"); }

    let use_position = id.is_none();
    let choice = if use_position { pos } else { id };
    let value = match choice {
        Some(p) => p,
        None => unreachable!()
    };

    let mut info = MENUITEMINFOW { 
        cbSize: mem::size_of::<MENUITEMINFOW>() as UINT,
        fMask: MIIM_STATE, fType: 0, fState: 0,
        wID: 0, hSubMenu: ptr::null_mut(), hbmpChecked: ptr::null_mut(),
        hbmpUnchecked: ptr::null_mut(), dwItemData: 0, dwTypeData: ptr::null_mut(),
        cch: 0, hbmpItem: ptr::null_mut()
    };

    GetMenuItemInfoW(h, value, use_position as BOOL, &mut info);

    (info.fState & MFS_DISABLED) != MFS_DISABLED
}

/**
    Return the state of a menu.
*/
#[inline(always)]
pub unsafe fn is_menu_enabled(menu: HMENU, parent: &AnyHandle) -> bool {
    let parent_h = resolve_menu_parent(parent);
    let index = menu_index_in_parent(menu, parent);
    is_menuitem_enabled(parent_h, Some(index), None)
}

/**
    Used in the events proc to return the inner id of a menuitem when triggering the clicked action
*/
#[inline(always)]
pub unsafe fn get_menu_id(parent_h: HMENU, index: c_int) -> UINT {
    ::low::defs::GetMenuItemID(parent_h, index)
}