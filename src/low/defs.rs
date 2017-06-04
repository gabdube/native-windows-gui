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
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::hash::Hash;
use std::any::{Any, TypeId};

use winapi::{UINT, LRESULT, DWORD, HBRUSH, ULONG_PTR, HMENU, BOOL, WORD, MENUITEMINFOW, IShellItem, HRESULT, IUnknownVtbl,
 IUnknown, PCWSTR, IBindCtx, REFIID, D2D1_FACTORY_TYPE, D2D1_FACTORY_OPTIONS, ID2D1Factory, c_void, c_int};
use std::ops::{Deref, DerefMut};


use events::{Event, EventCallback, EventArgs};
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
pub const NWG_TRIGGER:           UINT = 0x406;  /// Message sent when triggering an event
pub const NWG_CUSTOM_MAX:        UINT = 0x407;  /// Maximum custom event value

pub const NWG_DESTROY:           UINT = 0x420;  /// NWG `Destroy` event identifier

// Value returned by a window proc if the message execution failed/succeeded

pub const COMMIT_SUCCESS: LRESULT = 0;
pub const COMMIT_FAILED: LRESULT = 5555;

// Constants not included in winapi-rs

pub const MIM_STYLE: DWORD = 0x00000010;

pub const MIIM_STATE: DWORD = 0x00000001;
pub const MIIM_ID: DWORD = 0x00000002;

pub const MNS_NOTIFYBYPOS: DWORD = 0x08000000;

pub const MFS_DISABLED: DWORD = 0x00000003;
pub const MFS_ENABLED: DWORD = 0x00000000;

pub const MF_BYPOSITION: UINT = 0x00000400;
pub const MF_SEPARATOR: UINT = 0x00000800;

pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 0x008;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 0x010;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 0x004;

pub const CBN_SETFOCUS: WORD = 3;
pub const CBN_KILLFOCUS: WORD = 4;
pub const CBN_SELCHANGE: WORD = 1;

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

pub const LBN_SELCHANGE: WORD = 1;
pub const LBN_DBLCLK: WORD = 2;
pub const LBN_SETFOCUS: WORD = 4;
pub const LBN_KILLFOCUS: WORD = 5;

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

pub const ES_AUTOHSCROLL: UINT = 128;
pub const ES_AUTOVSCROLL: UINT = 64;
pub const ES_PASSWORD: UINT = 32;
pub const ES_READONLY: UINT = 0x800;
pub const ES_MULTILINE: UINT = 4;

pub const EM_LIMITTEXT: UINT = 197;
pub const EM_GETLIMITTEXT: UINT = 213;

pub const EN_SETFOCUS: WORD = 256;
pub const EN_KILLFOCUS: WORD = 512;
pub const EN_UPDATE: WORD = 1024;
pub const EN_MAXTEXT: WORD = 1281;

pub const STN_CLICKED: WORD = 0;
pub const STN_DBLCLK: WORD = 1;

pub const IDABORT: i32 = 3;
pub const IDCANCEL: i32 = 2;
pub const IDCONTINUE: i32 = 11;
pub const IDIGNORE: i32 = 5;
pub const IDNO: i32 = 7;
pub const IDOK: i32 = 1;
pub const IDRETRY: i32 = 4;
pub const IDTRYAGAIN: i32 = 10;
pub const IDYES: i32 = 6;

pub const SFGAO_FOLDER: u32 = 0x20000000;

pub const STATE_SYSTEM_CHECKED: u32 = 0x10;
pub const STATE_SYSTEM_INVISIBLE: u32 = 0x8000;

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

// COM interfaces
// Unused functions have an empty signature

// MACRO taken from winapi. Original author: Peter Atashian (retep998)
macro_rules! RIDL {
    (interface $interface:ident ($vtbl:ident) {$(
        fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
    ),+}) => (
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr),+
        }
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        RIDL!{@impl $interface {$(fn $method(&mut self $(,$p: $t)*) -> $rtr),+}}
    );
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {
    }) => (
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $pvtbl
        }
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { &*(self as *const _ as *const _) }
            }
        }
        impl DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { &mut *(self as *mut _ as *mut _) }
            }
        }
    );
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {$(
        fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
    ),+}) => (
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        RIDL!{@impl $interface {$(fn $method(&mut self $(,$p: $t)*) -> $rtr),+}}
        impl Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { &*(self as *const _ as *const _) }
            }
        }
        impl DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { &mut *(self as *mut _ as *mut _) }
            }
        }
    );
    (@impl $interface:ident {$(
        fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
    ),+}) => (
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
    );
}

RIDL!(
interface IShellItemArray(IShellItemArrayVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(&mut self) -> (),
    fn GetPropertyStore(&mut self) -> (),
    fn GetPropertyDescriptionList(&mut self) -> (),
    fn GetAttributes(&mut self) -> (),
    fn GetCount(&mut self, pdwNumItems: *mut DWORD) -> HRESULT,
    fn GetItemAt(&mut self, dwIndex: DWORD, ppsi: *mut *mut IShellItem) -> HRESULT,
    fn EnumItems(&mut self) -> ()
}
);

// System extern
extern "system" {
    pub fn GetMenuItemCount(menu: HMENU) -> c_int;
    pub fn GetSubMenu(hMenu: HMENU, nPos: c_int) -> HMENU;
    pub fn SetMenuInfo(menu: HMENU, info: &mut MENUINFO) -> BOOL;
    pub fn RemoveMenu(menu: HMENU, pos: UINT, flags: UINT) -> BOOL;
    pub fn GetMenuItemID(menu: HMENU, index: c_int) -> UINT;
    pub fn SetMenuItemInfoW(hMenu: HMENU, uItem: UINT, gByPosition: BOOL, lpmii: &mut MENUITEMINFOW) -> BOOL;
    pub fn GetMenuItemInfoW(hMenu: HMENU, uItem: UINT, gByPosition: BOOL, lpmii: &mut MENUITEMINFOW) -> BOOL;

    pub fn SHCreateItemFromParsingName(pszPath: PCWSTR, pbc: *mut IBindCtx, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT;

    pub fn D2D1CreateFactory(
        factoryType: D2D1_FACTORY_TYPE,
		riid: REFIID, 
		pFactoryOptions: *const D2D1_FACTORY_OPTIONS,
        ppIFactory: *mut *mut ID2D1Factory
    ) -> HRESULT;
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

pub struct TriggerArgs {
    pub id: u64,
    pub event: Event,
    pub args: EventArgs
}