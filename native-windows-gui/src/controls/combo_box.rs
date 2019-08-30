use super::ControlHandle;
use std::fmt::Display;


#[derive(Default, Debug)]
pub struct ComboBox<D: Clone+Display+Default> {
    pub handle: ControlHandle,
    collection: Vec<D>
}

impl<D: Clone+Display+Default> ComboBox<D> {

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("COMBOBOX")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> (u32, u32) {;
        (::winapi::um::winuser::WS_VISIBLE, 0)
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{CBS_DROPDOWNLIST, WS_BORDER, WS_CHILD};
        CBS_DROPDOWNLIST | WS_CHILD | WS_BORDER
    }

}
