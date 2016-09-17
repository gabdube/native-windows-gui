/*!
    A blank custom control.
*/

use std::hash::Hash;
use controls::ControlTemplate;
use controls::base::{WindowBase, create_base};
use winapi::{CW_USEDEFAULT, HWND};

/**
    Configuration properties to create a window
*/
pub struct Window {
    pub caption: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub visible: bool,
    pub resizable: bool
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for Window {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let base = WindowBase {
            text: self.caption.clone(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: self.visible,
            resizable: self.resizable
        };

        unsafe { create_base(base) }
    }

}

impl Default for Window {
    fn default() -> Window {
        Window { 
            caption: "Control".to_string(),
            size: (CW_USEDEFAULT as u32, CW_USEDEFAULT as u32),
            position: (CW_USEDEFAULT, CW_USEDEFAULT),
            visible: true,
            resizable: true
        }
    }
}