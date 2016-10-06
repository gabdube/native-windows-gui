/*!
    Wrapper over the win32 api constants so the user don't have to use winapi-rs
*/

use winapi::{MK_CONTROL, MK_SHIFT, MK_MBUTTON, MK_RBUTTON, MK_LBUTTON, LRESULT};

// Error codes
#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum Error {
    CONTROL_NOT_FOUND,
    ITEM_NOT_FOUND,
    MUST_HAVE_PARENT,
    TEMPLATE_CREATION,
    CONTROL_EXISTS,
    CALLBACK_NOT_SUPPORTED,
    INDEX_OUT_OF_BOUNDS,
    UNKNOWN
}

// Controls enums
#[derive(PartialEq, Debug, Clone)]
pub enum ControlType {
    Button,
    CheckBox,
    ComboBox,
    GroupBox,
    RadioButton,
    TextInput,
    Window
}

// Event enums

#[derive(PartialEq, Debug)]
pub enum CheckState {
    Checked,
    Unchecked,
    Indeterminate, // Tristate only
}

#[derive(PartialEq, Debug)]
pub enum WindowDisplay {
    Maximised,
    Minimized,
    Normal
}

#[derive(PartialEq, Debug)]
pub enum HTextAlign {
    Left,
    Center,
    Right
}

#[derive(PartialEq, Debug)]
pub enum VTextAlign {
    Top,
    Center,
    Bottom
}

// Event constants

pub const MOD_MOUSE_CTRL: u32 = MK_CONTROL as u32;
pub const MOD_MOUSE_SHIFT: u32 = MK_SHIFT as u32;

pub const BTN_MOUSE_MIDDLE: u32 = MK_MBUTTON as u32;
pub const BTN_MOUSE_RIGHT: u32 = MK_RBUTTON as u32;
pub const BTN_MOUSE_LEFT: u32 = MK_LBUTTON as u32;

// Constants that are not yet in WINAPI

pub const BM_GETSTATE: u32 = 242;
pub const BM_SETCHECK: u32 = 241;
pub const BM_CLICK: u32 = 245;

pub const BST_CHECKED: u32 = 1;
pub const BST_UNCHECKED: u32 = 0;
pub const BST_INDETERMINATE: u32 = 2;

pub const CBS_AUTOHSCROLL: u32 = 64;
pub const CBS_DROPDOWNLIST: u32 = 3;
pub const CBS_HASSTRINGS: u32 = 512;
pub const CBS_SORT: u32 = 256;

pub const CB_ERR: LRESULT = -1;

pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 0x008;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 0x010;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 0x004;

pub const CBN_CLOSEUP: u16 = 8;
pub const CBN_DROPDOWN: u16 = 7;
pub const CBN_SETFOCUS: u16 = 3;
pub const CBN_KILLFOCUS: u16 = 4;