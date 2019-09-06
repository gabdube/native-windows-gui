use crate::win32::window_helper as wh;
use super::ControlHandle;

const NOT_BOUND: &'static str = "StatusBar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: StatusBar handle is not HWND!";

#[derive(Default, Debug)]
pub struct StatusBar {
    pub handle: ControlHandle
}


impl StatusBar {

    /// Return the text in one of the region of the status bar
    pub fn text<'a>(&self, index: u8) -> String {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        "".to_string()
    }

    /// Set the text in one of the region of the status bar
    pub fn set_text<'a>(&self, index: u8, text: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        set_status_text(handle, index, text)
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("msctls_statusbar32")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> (u32, u32) {
        (::winapi::um::winuser::WS_VISIBLE, 0)
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
        bind_raw_event_handler(&parent_handle, move |_hwnd, _msg, _w, _l| {
            if msg == WM_SIZE {
                wh::send_message(handle, WM_SIZE, 0, 0);
            }
        });
    }

}

use std::mem;
use winapi::shared::windef::HWND;

fn set_status_text<'a>(handle: HWND, index: u8, text: &'a str) {
    use winapi::um::commctrl::{SB_SIMPLEID, SB_SETTEXTW};
    use winapi::shared::minwindef::WPARAM;
    use crate::win32::base_helper::to_utf16;

    let text = to_utf16(text);
    unsafe {
        wh::send_message(handle, SB_SETTEXTW, index as WPARAM, mem::transmute(text.as_ptr()));
    }
}
