/*!
    Types, constants and extern functions used in the low-level part of NWG
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
use std::any::{Any, TypeId};

use winapi::{UINT, LRESULT, DWORD, HBRUSH, ULONG_PTR, HMENU, BOOL, c_int, HBITMAP, LPWSTR};

use events::{Event, EventCallback};
use controls::ControlT;
use resources::ResourceT;

// Custom message proc definitions

pub const NWG_CUSTOM_MIN:        UINT = 0x400;  /// Minimum custom event value
pub const NWG_PACK_USER_VALUE:   UINT = 0x400;  /// Message sent when packing a user value
pub const NWG_PACK_CONTROL:      UINT = 0x401;  /// Message sent when packing a control
pub const NWG_UNPACK:            UINT = 0x402;  /// Message sent when removing an element from the ui
pub const NWG_BIND:              UINT = 0x403;  /// Message sent when binding an event to a control
pub const NWG_UNBIND:            UINT = 0x404;  /// Message sent when unbinding an event from a control
pub const NWG_PACK_RESOURCE:     UINT = 0x405;  /// Message sent when packing a resource
pub const NWG_CUSTOM_MAX:        UINT = 0x406;  /// Maximum custom event value

// Value returned by a window proc if the message execution failed/succeeded

pub const COMMIT_SUCCESS: LRESULT = 0;
pub const COMMIT_FAILED: LRESULT = 5555;

// Constants not included in winapi-rs

pub const MIM_MENUDATA: DWORD = 0x00000008;
pub const MIM_STYLE: DWORD = 0x00000010;

pub const MIIM_DATA: DWORD = 0x00000020;

pub const MNS_NOTIFYBYPOS: DWORD = 0x08000000;

pub const MF_BYPOSITION: UINT = 0x00000400;

pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 0x008;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 0x010;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 0x004;


// System structs
#[repr(C)]
#[allow(non_snake_case)]
pub struct MENUINFO {
    pub cbSize: DWORD,
    pub fMask: DWORD,
    pub dwStyle: DWORD,
    pub cyMax: UINT,
    pub hbrBack: HBRUSH,
    pub dwContextHelpID: DWORD,
    pub dwMenuData: ULONG_PTR
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct MENUITEMINFO {
    pub cbSize: UINT,
    pub fMask: UINT,
    pub fType: UINT,
    pub fState: UINT,
    pub wID: UINT,
    pub hSubMenu: HMENU,
    pub hbmpChecked: HBITMAP,
    pub hbmpUnchecked: HBITMAP,
    pub dwItemData: ULONG_PTR,
    pub dwTypeData: LPWSTR,
    pub cch: UINT,
    pub hbmpItem: HBITMAP
}


// System extern
extern "system" {
    pub fn GetMenuItemCount(menu: HMENU) -> c_int;
    pub fn GetSubMenu(hMenu: HMENU, nPos: c_int) -> HMENU;
    pub fn SetMenuInfo(menu: HMENU, info: &mut MENUINFO) -> BOOL;
    pub fn GetMenuInfo(menu: HMENU, info: &mut MENUINFO) -> BOOL;
    pub fn RemoveMenu(menu: HMENU, pos: UINT, flags: UINT) -> BOOL;
    pub fn SetMenuItemInfoW(menu: HMENU, item: UINT, by_position: BOOL, lpmii: *mut MENUITEMINFO) -> BOOL;
    pub fn GetMenuItemInfoW(menu: HMENU, item: UINT, by_position: BOOL, lpmii: *mut MENUITEMINFO) -> BOOL;
    pub fn GetMenuItemID(menu: HMENU, index: c_int) -> UINT;
}

// Arguments passed to the NWG custom events 

pub struct PackUserValueArgs<ID: Hash+Clone> {
    pub id: ID,
    pub tid: TypeId,
    pub value: Box<Any>
}

pub struct UnpackArgs {
    pub id: u64
}

pub struct PackControlArgs<ID: Hash+Clone> {
    pub id: ID,
    pub value: Box<ControlT<ID>>
}

pub struct PackResourceArgs<ID: Hash+Clone> {
    pub id: ID,
    pub value: Box<ResourceT<ID>>
}

pub struct BindArgs<ID: Hash+Clone+'static> {
    pub id: u64,
    pub cb_id: u64,
    pub event: Event,
    pub cb: Box<EventCallback<ID>>
}

pub struct UnbindArgs {
    pub id: u64,
    pub cb_id: u64,
    pub event: Event
}