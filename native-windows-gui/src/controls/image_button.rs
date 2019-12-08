/*!
A push button is a rectangle containing an application-defined text label, an icon, or a bitmap
that indicates what the button does when the user selects it.

ImageButton use the same event as `Button`.
*/

use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, BS_FLAT};
use crate::win32::window_helper as wh;
use crate::{SystemError, Image};
use super::{ControlBase, ControlHandle};

const NOT_BOUND: &'static str = "ImageButton is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ImageButton handle is not HWND!";


bitflags! {
    pub struct ImageButtonFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const FLAT = BS_FLAT;
    }
}

/**
A push button is a rectangle containing an application-defined icon or bitmap.
Use `Button` if you need to have a button that contains a label.
*/
#[derive(Default, Debug)]
pub struct ImageButton {
    pub handle: ControlHandle
}

impl ImageButton {

    pub fn builder<'a>() -> ImageButtonBuilder<'a> {
        ImageButtonBuilder {
            size: (100, 25),
            position: (0, 0),
            enabled: true,
            flags: None,
            parent: None,
            image: None
        }
    }

    pub fn image(&self) -> Option<Image> {
        use winapi::um::winuser::{BM_GETIMAGE, IMAGE_BITMAP};
        use winapi::shared::minwindef::{WPARAM};
        use winapi::um::winnt::HANDLE;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let image = wh::send_message(handle, BM_GETIMAGE, IMAGE_BITMAP as WPARAM, 0);
        if image == 0 {
            None
        } else {
            Some(Image{ handle: handle as HANDLE } )
        }
    }
    
    pub fn set_image<'a>(&self, image: Option<&'a Image>) {
        use winapi::um::winuser::{BM_SETIMAGE, IMAGE_BITMAP};
        use winapi::shared::minwindef::{WPARAM, LPARAM};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let image_handle = image.map(|i| i.handle as LPARAM).unwrap_or(0);
        wh::send_message(handle, BM_SETIMAGE, IMAGE_BITMAP as WPARAM, image_handle);
    }

    /// Returns true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Sets the keyboard focus on the button.
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Returns true if the control user can interact with the control, return false otherwise
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

    /// Returns true if the control is visible to the user. Will return true even if the 
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

    /// Returns the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Sets the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Returns the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Sets the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "BUTTON"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{BS_NOTIFY, WS_CHILD, BS_BITMAP};

        BS_NOTIFY | WS_CHILD | BS_BITMAP
    }

}

pub struct ImageButtonBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    image: Option<&'a Image>,
    flags: Option<ImageButtonFlags>,
    parent: Option<ControlHandle>
}

impl<'a> ImageButtonBuilder<'a> {

    pub fn flags(mut self, flags: ImageButtonFlags) -> ImageButtonBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ImageButtonBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> ImageButtonBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn enabled(mut self, e: bool) -> ImageButtonBuilder<'a> {
        self.enabled = e;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ImageButtonBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn image(mut self, i: Option<&'a Image>) -> ImageButtonBuilder<'a> {
        self.image = i;
        self
    }

    pub fn build(self, out: &mut ImageButton) -> Result<(), SystemError> {
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

        out.set_enabled(self.enabled);

        Ok(())
    }

}
