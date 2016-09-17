/*!
    Low level window creation utilities
*/

use winapi::{HWND};

pub struct WindowBase {
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub visible: bool,
    pub resizable: bool
}

pub unsafe fn create_base(base: WindowBase) -> Result<HWND, ()> {
    Err(())
}