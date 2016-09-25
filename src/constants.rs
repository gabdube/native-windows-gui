/*!
    Wrapper over the win32 api constants so the user don't have to use winapi-rs
*/

use winapi::{MK_CONTROL, MK_SHIFT, MK_MBUTTON, MK_RBUTTON, MK_LBUTTON};

// Error codes
#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum Error {
    CONTROL_NOT_FOUND,
    MUST_HAVE_PARENT,
    TEMPLATE_CREATION,
    CONTROL_EXISTS,
    CALLBACK_NOT_SUPPORTED
}

// Event constants
pub const MOD_MOUSE_CTRL: u32 = MK_CONTROL as u32;
pub const MOD_MOUSE_SHIFT: u32 = MK_SHIFT as u32;

pub const BTN_MOUSE_MIDDLE: u32 = MK_MBUTTON as u32;
pub const BTN_MOUSE_RIGHT: u32 = MK_RBUTTON as u32;
pub const BTN_MOUSE_LEFT: u32 = MK_LBUTTON as u32;