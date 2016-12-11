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

use winapi::{HMENU, DWORD, HBRUSH, c_int, UINT};

use controls::AnyHandle;

/**
    Struct stored in menu and menuitems by NWG to identify the menu when disptaching events
*/
struct MenuData {
    id: u64
}
type MenuItemData = MenuData;

enum MenuInfo {
    Style(DWORD),
    Data(*mut MenuData)
}

enum MenuItemInfo {
    Data(*mut MenuItemData)
}

unsafe fn get_menu_info(h: HMENU, info: &mut MenuInfo) {
    use low::defs::{GetMenuInfo, MENUINFO, MIM_MENUDATA};

    let mut info_ = MENUINFO {
        cbSize: mem::size_of::<MENUINFO>() as DWORD,
        fMask: MIM_MENUDATA,
        dwStyle: 0,
        cyMax: 0,
        hbrBack: mem::transmute(ptr::null_mut::<HBRUSH>()) ,
        dwContextHelpID: 0,
        dwMenuData: 0
    };

    GetMenuInfo(h, &mut info_);

    *info = MenuInfo::Data(mem::transmute(info_.dwMenuData));
}

unsafe fn set_menu_info(h: HMENU, info: &[MenuInfo]) {
    use low::defs::{SetMenuInfo, MENUINFO, MIM_STYLE, MIM_MENUDATA};

    let (mask, style, data) = {
        let mut mask = 0;
        let mut style = 0;
        let mut data = 0;
        for i in info.iter() {
            match i {
                &MenuInfo::Style(s) => {
                    style |= s;
                    mask |= MIM_STYLE;
                },
                &MenuInfo::Data(d) => {
                    mask |= MIM_MENUDATA;
                    data = mem::transmute(d);
                }
            }
        }
        (mask, style, data)
    };

    let mut info = MENUINFO {
        cbSize: mem::size_of::<MENUINFO>() as DWORD,
        fMask: mask,
        dwStyle: style,
        cyMax: 0,
        hbrBack: mem::transmute(ptr::null_mut::<HBRUSH>()) ,
        dwContextHelpID: 0,
        dwMenuData: data
    };

    SetMenuInfo(h, &mut info);
}

unsafe fn get_menu_item_info(parent_h: HMENU, uid: UINT, info: &mut MenuItemInfo) {
    use low::defs::{MENUITEMINFO, MIIM_DATA, GetMenuItemInfoW};

    let mut info_ = MENUITEMINFO {
        cbSize: mem::size_of::<MENUITEMINFO>() as UINT,
        fMask: MIIM_DATA,
        fType: 0, fState: 0, wID: 0, hSubMenu: ptr::null_mut(),
        hbmpChecked: ptr::null_mut(), hbmpUnchecked: ptr::null_mut(),
        dwItemData: 0,
        dwTypeData: ptr::null_mut(), cch: 0, hbmpItem: ptr::null_mut()
    };

    GetMenuItemInfoW(parent_h, uid, 0, &mut info_);

    *info = MenuItemInfo::Data(mem::transmute(info_.dwItemData));
}

unsafe fn set_menu_item_info(parent_h: HMENU, uid: UINT, info: &[MenuItemInfo]) {
    use low::defs::{MENUITEMINFO, MIIM_DATA, SetMenuItemInfoW};

    let (mask, data) = {
        let mut mask = 0;
        let mut data = 0;
        for i in info.iter() {
            match i {
                &MenuItemInfo::Data(d) => {
                    mask |= MIIM_DATA;
                    data = mem::transmute(d);
                }
            }
        }
        (mask, data)
    };

    let mut info = MENUITEMINFO {
        cbSize: mem::size_of::<MENUITEMINFO>() as UINT,
        fMask: mask,
        fType: 0, fState: 0, wID: 0, hSubMenu: ptr::null_mut(),
        hbmpChecked: ptr::null_mut(), hbmpUnchecked: ptr::null_mut(),
        dwItemData: data,
        dwTypeData: ptr::null_mut(), cch: 0, hbmpItem: ptr::null_mut()
    };

    SetMenuItemInfoW(parent_h, uid, 0, &mut info);
}

/**
    List the children of a menu and return a list of their IDs. The function is recursive and so 
    it list the ids for the whole menu tree.
*/
pub unsafe fn list_menu_children(menu: HMENU) -> Vec<u64> { 
    use low::defs::{GetMenuItemCount, GetSubMenu, GetMenuItemID};

    let mut children = Vec::new();
    let children_count = GetMenuItemCount(menu);

    let mut info = MenuInfo::Data(ptr::null_mut());
    let mut item_info = MenuItemInfo::Data(ptr::null_mut());
    let mut sub_menu: HMENU;

    for i in 0..children_count {
        sub_menu = GetSubMenu(menu, i as c_int);
        if sub_menu.is_null() {

            // Get a menu item ID
            get_menu_item_info(menu, GetMenuItemID(menu, i), &mut item_info);
            match item_info {
                MenuItemInfo::Data(info_ptr) => {
                    if info_ptr.is_null() { continue; }
                    children.push((&mut *info_ptr).id)
                },
                //_ => unreachable!()
            };

            continue;
        }

        // Get the menu ID
        get_menu_info(sub_menu, &mut info);
        match info {
            MenuInfo::Data(info_ptr) => {
                children.push((&mut *info_ptr).id);
                children.append(&mut list_menu_children(sub_menu));
            },
            _ => unreachable!()
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
    Init the private NWG menu data
*/
pub fn init_menu_data(h: HMENU, id: u64) {
    let data: Box<MenuData> = Box::new(MenuData{id: id});
    unsafe{ set_menu_info(h, &[MenuInfo::Data(Box::into_raw(data))]); }
}

/**
    Init the private NWG menu item data.
*/
pub fn init_menu_item_data(parent_h: HMENU, uid: UINT, id: u64) {
    let data: Box<MenuItemData> = Box::new(MenuItemData{id: id});
    unsafe{ set_menu_item_info(parent_h, uid, &[MenuItemInfo::Data(Box::into_raw(data))] ); }
}

/**
    Free the NWG data from the menu
*/
pub fn free_menu_data(h: HMENU) {
    let mut info = MenuInfo::Data(ptr::null_mut());
    
    unsafe{ get_menu_info(h, &mut info) };
    match info {
        MenuInfo::Data(info_ptr) => unsafe { mem::forget(Box::from_raw(info_ptr)) },
        _ => unreachable!()
    }

    unsafe{ set_menu_info(h, &[MenuInfo::Data(ptr::null_mut())]); }
}

/**
    Free the NWG data from the menu item
*/
pub fn free_menu_item_data(parent_h: HMENU, uid: UINT) {
     let mut info = MenuItemInfo::Data(ptr::null_mut());
    
    unsafe{ get_menu_item_info(parent_h, uid, &mut info) };
    match info {
        MenuItemInfo::Data(info_ptr) => unsafe { mem::forget(Box::from_raw(info_ptr)) },
        //_ => unreachable!()
    }

    unsafe{ set_menu_item_info(parent_h, uid, &[MenuItemInfo::Data(ptr::null_mut())]); }
}

/**
    Configure the menu to use a WM_MENUCOMMAND instead of a WM_COMMAND when its action are triggered.
    Required in order to allow nwg to dispatch the events correctly
*/
#[inline(always)]
pub fn use_menu_command(h: HMENU) {
    use low::defs::MNS_NOTIFYBYPOS;
    unsafe{ set_menu_info(h, &[MenuInfo::Style(MNS_NOTIFYBYPOS)]) }
}