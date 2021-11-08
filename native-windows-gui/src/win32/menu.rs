/*!
Native Windows GUI menu base.
*/
use winapi::shared::windef::{HMENU, HWND};
use winapi::shared::minwindef::UINT;
use super::base_helper::{CUSTOM_ID_BEGIN, to_utf16};
use crate::controls::ControlHandle;
use crate::{NwgError};
use std::{mem, ptr};
use std::sync::atomic::{AtomicU32, Ordering};


static MENU_ITEMS_ID: AtomicU32 = AtomicU32::new(CUSTOM_ID_BEGIN); 


/// Build a system menu
pub unsafe fn build_hmenu_control(text: Option<String>, item: bool, separator: bool, popup: bool, hmenu: Option<HMENU>, hwnd: Option<HWND>) -> Result<ControlHandle, NwgError> {
    use winapi::um::winuser::{CreateMenu, CreatePopupMenu, GetMenu, SetMenu, DrawMenuBar, AppendMenuW};
    use winapi::um::winuser::{MF_STRING, MF_POPUP};

    if separator {
        if hmenu.is_none() {
            return Err(NwgError::menu_create("Separator without parent"));
        }
        return Ok(build_hmenu_separator(hmenu.unwrap()));
    }

    if popup {
        if hwnd.is_none() {
            return Err(NwgError::menu_create("Popup menu without parent"));
        }

        let menu = CreatePopupMenu();
        if menu.is_null() {
            return Err(NwgError::menu_create("Popup menu creation failed"));
        }

        use_menu_command(menu);

        return Ok(ControlHandle::PopMenu(hwnd.unwrap(), menu));
    }

    let mut parent_menu: HMENU = ptr::null_mut();
    let mut menu: HMENU = ptr::null_mut();
    let mut item_id = 0;

    let mut flags = MF_STRING;
    if !item { flags |= MF_POPUP; }

    let text = to_utf16(text.unwrap_or("".to_string()).as_ref());

    if hwnd.is_some() {
        let hwnd = hwnd.unwrap();
        let mut menubar = GetMenu(hwnd);
        if menubar.is_null() {
            // If the window do not have a menu bar, create one
            menubar = CreateMenu();
            use_menu_command(menubar);
            SetMenu(hwnd, menubar);
        }

        if item {
            menu = menubar;
            item_id = MENU_ITEMS_ID.fetch_add(1, Ordering::SeqCst);
            AppendMenuW(menubar, flags, item_id as usize, text.as_ptr());
        } else {
            parent_menu = menubar;
            menu = CreateMenu();
            if menu.is_null() {
                return Err(NwgError::menu_create("Menu without parent"));
            }
            use_menu_command(menu);
            AppendMenuW(menubar, flags, mem::transmute(menu), text.as_ptr());
        }

        // Draw the menu bar to make sure the changes are visible
        DrawMenuBar(hwnd); 
    } else if hmenu.is_some() {
        let parent = hmenu.unwrap();

        if item {
            menu = parent;
            item_id = MENU_ITEMS_ID.fetch_add(1, Ordering::SeqCst);
            AppendMenuW(parent, flags, item_id as usize, text.as_ptr());
        } else {
            parent_menu = parent;
            menu = CreateMenu();
            if menu.is_null() {
                return Err(NwgError::menu_create("Menu without parent"));
            }
            use_menu_command(menu);
            AppendMenuW(parent, flags, mem::transmute(menu), text.as_ptr());
        }
    }

    if item {
        Ok(ControlHandle::MenuItem(menu, item_id))
    } else {
        Ok(ControlHandle::Menu(parent_menu, menu))
    }
}

/**
    Enable or disable a menuitem at the selected position or using the selected ID. If the position is None and id is None, the last item is selected.
*/
pub unsafe fn enable_menuitem(h: HMENU, pos: Option<UINT>, id: Option<UINT>, enabled: bool) {
    use winapi::um::winuser::{MENUITEMINFOW, MIIM_STATE, MFS_DISABLED, MFS_ENABLED};
    use winapi::um::winuser::{SetMenuItemInfoW, GetMenuItemCount};
    use winapi::shared::minwindef::BOOL;
    
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
    Return the state of a menuitem. Panic if both pos and id are None.
*/
pub unsafe fn is_menuitem_enabled(h: HMENU, pos: Option<UINT>, id: Option<UINT>) -> bool {
    use winapi::um::winuser::{MENUITEMINFOW, MIIM_STATE, MFS_DISABLED};
    use winapi::um::winuser::GetMenuItemInfoW;
    use winapi::shared::minwindef::BOOL;

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


/// Set the state of a menuitem
pub unsafe fn enable_menu(parent_menu: HMENU, menu: HMENU, e: bool) {
    let menu_index = menu_index_in_parent(parent_menu, menu);
    enable_menuitem(parent_menu, Some(menu_index), None, e);
}

/// Return the state of a menu.
pub unsafe fn is_menu_enabled(parent_menu: HMENU, menu: HMENU) -> bool {
    let menu_index = menu_index_in_parent(parent_menu, menu);
    is_menuitem_enabled(parent_menu, Some(menu_index), None)
}

pub unsafe fn check_menu_item(parent_menu: HMENU, id: u32, check: bool) {
    use winapi::um::winuser::{CheckMenuItem, MF_BYCOMMAND, MF_CHECKED, MF_UNCHECKED};

    let check = match check {
        true => MF_CHECKED,
        false => MF_UNCHECKED
    };

    CheckMenuItem(parent_menu, id, MF_BYCOMMAND | check);
}

pub unsafe fn menu_item_checked(parent_menu: HMENU, id: u32) -> bool {
    use winapi::um::winuser::{GetMenuState, MF_BYCOMMAND, MF_CHECKED};
    GetMenuState(parent_menu, id, MF_BYCOMMAND) & MF_CHECKED == MF_CHECKED
}


unsafe fn build_hmenu_separator(menu: HMENU) -> ControlHandle {
    use winapi::um::winuser::{GetMenuItemCount, SetMenuItemInfoW, AppendMenuW};
    use winapi::um::winuser::{MENUITEMINFOW, MF_SEPARATOR, MIIM_ID};
    use winapi::shared::minwindef::{BOOL};

    let item_id = MENU_ITEMS_ID.fetch_add(1, Ordering::SeqCst);

    // MF_SEPARATOR ignore the lpNewItem and uIDNewItem parameters, so they must be set using SetMenuItemInfo
    AppendMenuW(menu, MF_SEPARATOR, 0, ptr::null());

    // Set the unique id of the separator
    let pos = GetMenuItemCount(menu) - 1;
    let mut info = MENUITEMINFOW { 
        cbSize: mem::size_of::<MENUITEMINFOW>() as UINT,
        fMask: MIIM_ID, fType: 0, fState: 0,
        wID: item_id,
        hSubMenu: ptr::null_mut(), hbmpChecked: ptr::null_mut(),
        hbmpUnchecked: ptr::null_mut(), dwItemData: 0, dwTypeData: ptr::null_mut(),
        cch: 0, hbmpItem: ptr::null_mut()
    };

    SetMenuItemInfoW(menu, pos as UINT, true as BOOL, &mut info);

    ControlHandle::MenuItem(menu, item_id)
}

/**
    Configure the menu to use a WM_MENUCOMMAND instead of a WM_COMMAND when its action are triggered.
    Required in order to allow nwg to dispatch the events correctly
*/
unsafe fn use_menu_command(h: HMENU) {
    use winapi::um::winuser::{MENUINFO, MNS_NOTIFYBYPOS, MIM_STYLE, SetMenuInfo};
    use winapi::shared::minwindef::DWORD;

    let mut info = MENUINFO {
        cbSize: mem::size_of::<MENUINFO>() as DWORD,
        fMask: MIM_STYLE,
        dwStyle: MNS_NOTIFYBYPOS,
        cyMax: 0,
        hbrBack: ptr::null_mut(),
        dwContextHelpID: 0,
        dwMenuData: 0
    };

    SetMenuInfo(h, &mut info);
}

/**
    Return the index of a children menu/menuitem in a parent menu.
    Panic if the menu is not found in the parent.
*/
pub unsafe fn menu_index_in_parent(parent: HMENU, menu: HMENU) -> UINT {
    use winapi::um::winuser::{GetMenuItemCount, GetSubMenu};

    let children_count = GetMenuItemCount(parent);
    let mut sub_menu: HMENU;

    for i in 0..children_count {
        sub_menu = GetSubMenu(parent, i as i32);
        if sub_menu.is_null() { continue; }
        else if sub_menu == menu { return i as UINT; }
    }

    panic!("Menu/MenuItem not found in parent!")
}
