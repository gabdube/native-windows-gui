use winapi::shared::minwindef::{WPARAM, LPARAM};
use crate::win32::window_helper as wh;
use crate::{Font, NwgError};
use super::{ControlHandle, ControlBase};

const NOT_BOUND: &'static str = "StatusBar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: StatusBar handle is not HWND!";

#[derive(Default, Debug)]
pub struct StatusBar {
    pub handle: ControlHandle
}


impl StatusBar {

    pub fn builder<'a>() -> StatusBarBuilder<'a> {
        StatusBarBuilder {
            text: "",
            font: None,
            parent: None
        }
    }

    /// Set the minimum height of the statusbar (in pixels)
    pub fn set_min_height(&self, height: u32) {
        use winapi::um::commctrl::SB_SETMINHEIGHT;
        use winapi::um::winuser::WM_SIZE;

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
        use winapi::um::commctrl::{SB_GETTEXTLENGTHW, SB_GETTEXTW};
        use winapi::shared::minwindef::{LOWORD};
        use crate::win32::base_helper::from_utf16;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        let result = wh::send_message(handle, SB_GETTEXTLENGTHW, index as WPARAM, 0);
        let text_length = LOWORD(result as u32) as usize;

        let mut buffer: Vec<u16> = Vec::with_capacity(text_length);
        unsafe { buffer.set_len(text_length); }
        
        wh::send_message(handle, SB_GETTEXTW, index as WPARAM, buffer.as_mut_ptr() as LPARAM);

        from_utf16(&buffer)
    }

    /// Set the text in one of the region of the status bar
    pub fn set_text<'a>(&self, index: u8, text: &'a str) {
        use winapi::um::commctrl::{SB_SETTEXTW};
        use crate::win32::base_helper::to_utf16;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        let text = to_utf16(text);
        wh::send_message(handle, SB_SETTEXTW, index as WPARAM, text.as_ptr() as LPARAM);
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "msctls_statusbar32"
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

pub struct StatusBarBuilder<'a> {
    text: &'a str,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}

impl<'a> StatusBarBuilder<'a> {

    pub fn text(mut self, text: &'a str) -> StatusBarBuilder<'a> {
        self.text = text;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> StatusBarBuilder<'a> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> StatusBarBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut StatusBar) -> Result<(), NwgError> {
        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("StatusBar"))
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(out.flags())
            .parent(Some(parent))
            .build()?;

        out.set_text(0, self.text);
        out.hook_parent_resize();

        Ok(())
    }

}
