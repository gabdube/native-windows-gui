/*!
An image frame is a control that display an image resource. It also accept mouse clicks.
*/

use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use crate::win32::window_helper as wh;
use super::{ControlBase, ControlHandle};
use crate::{Image, SystemError};

const NOT_BOUND: &'static str = "ImageFrame is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ImageFrame handle is not HWND!";


bitflags! {
    pub struct ImageFrameFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/// Display a bitmap image in the application
#[derive(Default, Debug)]
pub struct ImageFrame {
    pub handle: ControlHandle
}

impl ImageFrame {

    pub fn builder<'a>() -> ImageFrameBuilder<'a> {
        ImageFrameBuilder {
            size: (100, 100),
            position: (0, 0),
            flags: None,
            image: None,
            parent: None
        }
    }

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

    /// Return the size of the image frame in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the image frame in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the image frame in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the image frame in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("STATIC")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        use winapi::um::winuser::{SS_BITMAP, SS_CENTERIMAGE};
        WS_VISIBLE | SS_BITMAP | SS_CENTERIMAGE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{SS_NOTIFY, WS_CHILD};

        WS_CHILD | SS_NOTIFY
    }

}

pub struct ImageFrameBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<ImageFrameFlags>,
    image: Option<&'a Image>,
    parent: Option<ControlHandle>
}

impl<'a> ImageFrameBuilder<'a> {

    pub fn flags(mut self, flags: ImageFrameFlags) -> ImageFrameBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ImageFrameBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> ImageFrameBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn image(mut self, image: Option<&'a Image>) -> ImageFrameBuilder<'a> {
        self.image = image;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ImageFrameBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut ImageFrame) -> Result<(), SystemError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(SystemError::ControlWithoutParent)
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        if self.image.is_some() {
            out.set_image(self.image);
        }

        Ok(())
    }

}
