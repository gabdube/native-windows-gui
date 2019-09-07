use crate::win32::window_helper as wh;
use super::ControlHandle;
use crate::Image;

const NOT_BOUND: &'static str = "ImageFrame is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ImageFrame handle is not HWND!";


/// Display a bitmap image in the application
#[derive(Default, Debug)]
pub struct ImageFrame {
    pub handle: ControlHandle
}

impl ImageFrame {

    /// Get the image of the image frame
    pub fn image(&self) -> Option<Image> {
        use winapi::um::winuser::{STM_GETIMAGE, IMAGE_BITMAP};
        use std::mem;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let image = wh::send_message(handle, STM_GETIMAGE, IMAGE_BITMAP as usize, 0);
        if image != 0 {
            Some(Image{ handle: unsafe { mem::transmute(image) } })
        } else {
            None
        }
    }

    /// Set the image of the image frame.
    pub fn set_image(&self, image: Option<&Image>) {
        use winapi::um::winuser::{STM_SETIMAGE, IMAGE_BITMAP};
        use winapi::um::wingdi::DeleteObject;
        use std::{mem, ptr};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let new_image = image.map(|i| i.handle).unwrap_or(ptr::null_mut());
        let old_image = wh::send_message(handle, STM_SETIMAGE, IMAGE_BITMAP as usize, new_image as isize);

        if old_image != 0 {
            unsafe {
                DeleteObject(mem::transmute(old_image));
            }
        }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the label in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the label in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the label in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the label in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the label text
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the label text
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("STATIC")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> (u32, u32) {
        use winapi::um::winuser::{SS_BITMAP, SS_CENTERIMAGE, WS_VISIBLE};

        (WS_VISIBLE | SS_BITMAP | SS_CENTERIMAGE, 0)
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{SS_NOTIFY, WS_CHILD};

        WS_CHILD | SS_NOTIFY
    }

}
