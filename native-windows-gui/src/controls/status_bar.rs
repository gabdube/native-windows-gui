use crate::win32::window_helper as wh;
use crate::Font;
use super::ControlHandle;

const NOT_BOUND: &'static str = "StatusBar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: StatusBar handle is not HWND!";

#[derive(Default, Debug)]
pub struct StatusBar {
    pub handle: ControlHandle
}


impl StatusBar {

    /// Set the minimum height of the statusbar (in pixels)
    pub fn set_min_height(&self, height: u32) {
        use winapi::um::commctrl::SB_SETMINHEIGHT;
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::WPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, SB_SETMINHEIGHT, height as WPARAM, 0); 
        wh::send_message(handle, WM_SIZE, 0, 0);  // redraw the statusbar
    }

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return the text in one of the region of the status bar
    pub fn text<'a>(&self, index: u8) -> String {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        get_status_text(handle, index)
    }

    /// Set the text in one of the region of the status bar
    pub fn set_text<'a>(&self, index: u8, text: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        set_status_text(handle, index, text);
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("msctls_statusbar32")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD};

        WS_CHILD
    }

    /// Status bar do not resize automatically. Instead, a resize message must be
    /// manually sent by the parent window to trigger the resize action.
    pub fn hook_parent_resize(&self) {
        use winapi::um::winuser::WM_SIZE;
        use crate::bind_raw_event_handler;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let parent_handle = ControlHandle::Hwnd(wh::get_window_parent(handle));
        bind_raw_event_handler(&parent_handle, 0, move |_hwnd, msg, _w, _l| {
            if msg == WM_SIZE {
                wh::send_message(handle, WM_SIZE, 0, 0);
            }

            None
        });
    }

}

use std::mem;
use winapi::shared::windef::HWND;

fn set_status_text<'a>(handle: HWND, index: u8, text: &'a str) {
    use winapi::um::commctrl::{SB_SETTEXTW};
    use winapi::shared::minwindef::WPARAM;
    use crate::win32::base_helper::to_utf16;

    let text = to_utf16(text);
    unsafe {
        wh::send_message(handle, SB_SETTEXTW, index as WPARAM, mem::transmute(text.as_ptr()));
    }
}

fn get_status_text(handle: HWND, index: u8) -> String {
    use winapi::um::commctrl::{SB_GETTEXTLENGTHW, SB_GETTEXTW};
    use winapi::shared::minwindef::{LOWORD, WPARAM};
    use crate::win32::base_helper::from_utf16;

    unsafe {
        let result = wh::send_message(handle, SB_GETTEXTLENGTHW, index as WPARAM, 0);
        let text_length = LOWORD(result as u32) as usize;

        let mut buffer: Vec<u16> = Vec::with_capacity(text_length);
        buffer.set_len(text_length);
        
        wh::send_message(handle, SB_GETTEXTW, index as WPARAM, mem::transmute(buffer.as_mut_ptr()));

        from_utf16(&buffer)
    }
}
