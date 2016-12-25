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

use winapi::{UINT, LRESULT, DWORD, HBRUSH, ULONG_PTR, HMENU, BOOL, c_int, MENUITEMINFOW};

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

pub const MIM_STYLE: DWORD = 0x00000010;
pub const MIIM_ID: DWORD = 0x00000002;

pub const MNS_NOTIFYBYPOS: DWORD = 0x08000000;

pub const MF_BYPOSITION: UINT = 0x00000400;
pub const MF_SEPARATOR: UINT = 0x00000800;

pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 0x008;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 0x010;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 0x004;

pub const LB_ADDSTRING: UINT = 384;
pub const LB_INSERTSTRING: UINT = 385;
pub const LB_DELETESTRING: UINT = 386;
pub const LB_SELITEMRANGEEX: UINT = 387;
pub const LB_RESETCONTENT: UINT = 388;
pub const LB_SETSEL: UINT = 389;
pub const LB_SETCURSEL: UINT = 390;
pub const LB_GETCURSEL: UINT = 392;
pub const LB_GETTEXT: UINT = 393;
pub const LB_GETTEXTLEN: UINT = 394;
pub const LB_GETSEL: UINT = 391;
pub const LB_FINDSTRING: UINT = 399;
pub const LB_GETSELCOUNT: UINT = 400;
pub const LB_GETSELITEMS: UINT = 401;
pub const LB_FINDSTRINGEXACT: UINT = 418;

pub const LBS_NOTIFY: UINT = 1;
pub const LBS_NOSEL: UINT = 0x4000;
pub const LBS_HASSTRINGS: UINT = 64;
pub const LBS_MULTIPLESEL: UINT = 8;

pub const LBN_SELCHANGE: UINT = 1;
pub const LBN_DBLCLK: UINT = 2;
pub const LBN_SETFOCUS: UINT = 4;
pub const LBN_KILLFOCUS: UINT = 5;

pub const BN_CLICKED: UINT = 0;
pub const BN_DBLCLK: UINT = 5;
pub const BN_SETFOCUS: UINT = 6;
pub const BN_KILLFOCUS: UINT = 7;

pub const BM_SETCHECK: UINT = 241;
pub const BM_GETCHECK: UINT = 240;

pub const BST_CHECKED: UINT = 1;
pub const BST_INDETERMINATE: UINT = 2;
pub const BST_UNCHECKED: UINT = 0;

pub const SS_NOTIFY: UINT = 256;
pub const SS_RIGHT: UINT = 2;
pub const SS_LEFT: UINT = 0;
pub const SS_CENTER: UINT = 1;   
pub const SS_NOPREFIX: UINT = 128;

pub const CBS_DROPDOWNLIST: UINT = 3;
pub const CBS_HASSTRINGS: UINT = 512;

pub const CB_ADDSTRING: UINT = 323;
pub const CB_RESETCONTENT: UINT = 331;
pub const CB_DELETESTRING: UINT = 324;
pub const CB_INSERTSTRING: UINT = 330;
pub const CB_FINDSTRING: UINT = 332;
pub const CB_FINDSTRINGEXACT: UINT = 344;
pub const CB_GETCURSEL: UINT = 327;
pub const CB_GETDROPPEDSTATE: UINT = 343;
pub const CB_GETLBTEXT: UINT = 328;
pub const CB_GETLBTEXTLEN: UINT = 329;
pub const CB_SETCURSEL: UINT = 334;
pub const CB_SHOWDROPDOWN: UINT = 335;
pub const CB_SETCUEBANNER: UINT = 5891;

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

// System extern
extern "system" {
    pub fn GetMenuItemCount(menu: HMENU) -> c_int;
    pub fn GetSubMenu(hMenu: HMENU, nPos: c_int) -> HMENU;
    pub fn SetMenuInfo(menu: HMENU, info: &mut MENUINFO) -> BOOL;
    pub fn RemoveMenu(menu: HMENU, pos: UINT, flags: UINT) -> BOOL;
    pub fn GetMenuItemID(menu: HMENU, index: c_int) -> UINT;
    pub fn SetMenuItemInfoW(hMenu: HMENU, uItem: UINT, gByPosition: bool, lpmii: &mut MENUITEMINFOW) -> BOOL;
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