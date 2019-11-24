/*!
    A fancy window is a top level Window control that support transparency.
    In WinApi, it's called a "Layered Window"

    By default, a fancy window has the popup style and do not come with the default Windows decoration.

    Using a layered window can significantly improve performance and visual effects for a window
    that has a complex shape, animates its shape, or wishes to use alpha blending effects.
    The system automatically composes and repaints layered windows and the windows of underlying
    applications. As a result, layered windows are rendered smoothly, without the flickering typical
    of complex window regions. In addition, layered windows can be partially translucent, that is, alpha-blended.
*/

use crate::win32::window_helper as wh;
use crate::Image;
use super::ControlHandle;


const NOT_BOUND: &'static str = "FancyWindow is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: FancyWindow handle is not HWND!";

/**
A fancy window is a top level Window control that support transparency.
In WinApi, it's called a "Layered Window". See the top level module documentation for more information
*/
#[derive(Default, Debug)]
pub struct FancyWindow {
    pub handle: ControlHandle
}


impl FancyWindow {

    pub fn init_layered(&self) {
        use winapi::um::winuser::{WS_EX_LAYERED, GWL_EXSTYLE, LWA_COLORKEY};
        use winapi::um::winuser::SetLayeredWindowAttributes;
        use winapi::um::wingdi::RGB;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let mut style = wh::get_window_long(handle, GWL_EXSTYLE) as usize;
        style |= WS_EX_LAYERED as usize;

        wh::set_window_long(handle, GWL_EXSTYLE, style);

        unsafe {
            let color = RGB(0,0,0);
            SetLayeredWindowAttributes(handle, color, 255, LWA_COLORKEY);
        }
    }

    //
    // Basic functions
    //

    /// Return the icon of the window
    pub fn icon(&self) -> Option<Image> {
        use winapi::um::winuser::WM_GETICON;
        use std::mem;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let handle = wh::send_message(handle, WM_GETICON, 0, 0);
        if handle == 0 {
            None
        } else {
            Some(Image{ handle: unsafe{ mem::transmute(handle) } })
        }
    }

    /// Set the icon in the window
    /// - icon: The new icon. If None, the icon is removed
    pub fn set_icon(&self, icon: Option<&Image>) {
        use winapi::um::winuser::WM_SETICON;
        use std::{mem, ptr};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let image_handle = icon.map(|i| i.handle).unwrap_or(ptr::null_mut());
        unsafe {
            wh::send_message(handle, WM_SETICON, 0, mem::transmute(image_handle));
        }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return window title
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the window title
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "NativeWindowsGuiWindow"
    }

    // Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        use winapi::um::winuser::{WS_VISIBLE, WS_POPUP};

        WS_VISIBLE | WS_POPUP
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::WS_CLIPCHILDREN;

        WS_CLIPCHILDREN
    }
}
