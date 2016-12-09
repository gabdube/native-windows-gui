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

use winapi::{HMENU, DWORD, HBRUSH};

pub enum MenuInfo {
    Style(DWORD),
}

pub fn list_menu_children(menu: HMENU) -> Vec<u64> {
    use low::defs::GetMenuItemCount;

    let mut children = Vec::new();
    let children_count = unsafe{ GetMenuItemCount(menu) };

    for i in 0..children_count {
        
    }


    children
}

pub unsafe fn set_menu_info(h: HMENU, info: &[MenuInfo]) {
    use low::defs::{SetMenuInfo, MENUINFO, MIM_STYLE};

    let (mask, style) = {
        let mut mask = 0;
        let mut style = 0;
        for i in info.iter() {
            match i {
                &MenuInfo::Style(s) => {
                    style |= s;
                    mask |= MIM_STYLE;
                },
            }
        }
        (mask, style)
    };

    let mut info = MENUINFO {
        cbSize: mem::size_of::<MENUINFO>() as DWORD,
        fMask: mask,
        dwStyle: style,
        cyMax: 0,
        hbrBack: mem::transmute(ptr::null_mut::<HBRUSH>()) ,
        dwContextHelpID: 0,
        dwMenuData: 0
    };

    SetMenuInfo(h, &mut info);
}


/**
    Configure the menu to use a WM_MENUCOMMAND instead of a WM_COMMAND when its action are triggered.
    Required in order to allow nwg to dispatch the events correctly
*/
#[inline(always)]
pub unsafe fn use_menu_command(h: HMENU) {
    use low::defs::MNS_NOTIFYBYPOS;
    set_menu_info(h, &[MenuInfo::Style(MNS_NOTIFYBYPOS)])
}