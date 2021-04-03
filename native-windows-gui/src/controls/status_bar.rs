use winapi::shared::minwindef::{WPARAM, LPARAM};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::{Font, NwgError, RawEventHandler, unbind_raw_event_handler};
use super::{ControlHandle, ControlBase};
use std::cell::RefCell;

const NOT_BOUND: &'static str = "StatusBar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: StatusBar handle is not HWND!";

/**
A status bar is a horizontal window at the bottom of a parent window in which an application can display various kinds of status information.
Status bar cannot stack, so there must be only one per window.

Requires the `status-bar` feature. 

**Builder parameters:**
  * `parent`:   **Required.** The status bar parent container.
  * `text`:     The status bar text.
  * `font`:     The font used for the status bar text

**Control events:**
  * `MousePress(_)`: Generic mouse press events on the status bar
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event

```rust
use native_windows_gui as nwg;
fn build_status(status: &mut nwg::StatusBar, window: &nwg::Window, font: &nwg::Font) {
    nwg::StatusBar::builder()
        .text("Hello")
        .font(Some(font))
        .parent(window)
        .build(status);
}
```

*/
#[derive(Default)]
pub struct StatusBar {
    pub handle: ControlHandle,
    handler0: RefCell<Option<RawEventHandler>>,
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

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, SB_SETMINHEIGHT, height as WPARAM, 0); 
        wh::send_message(handle, WM_SIZE, 0, 0);  // redraw the statusbar
    }

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return the text in one of the region of the status bar
    pub fn text<'a>(&self, index: u8) -> String {
        use winapi::um::commctrl::{SB_GETTEXTLENGTHW, SB_GETTEXTW};
        use winapi::shared::minwindef::LOWORD;
        use crate::win32::base_helper::from_utf16;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let result = wh::send_message(handle, SB_GETTEXTLENGTHW, index as WPARAM, 0);
        let text_length = (LOWORD(result as u32) as usize) + 1; // +1 for the terminating null character

        let mut buffer: Vec<u16> = Vec::with_capacity(text_length);
        unsafe { buffer.set_len(text_length); }
        
        wh::send_message(handle, SB_GETTEXTW, index as WPARAM, buffer.as_mut_ptr() as LPARAM);

        from_utf16(&buffer)
    }

    /// Set the text in one of the region of the status bar
    pub fn set_text<'a>(&self, index: u8, text: &'a str) {
        use winapi::um::commctrl::SB_SETTEXTW;
        use crate::win32::base_helper::to_utf16;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
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
        use crate::bind_raw_event_handler_inner;

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let parent_handle = ControlHandle::Hwnd(wh::get_window_parent(handle));
        let handler = bind_raw_event_handler_inner(&parent_handle, handle as usize, move |_hwnd, msg, _w, _l| {
            if msg == WM_SIZE {
                wh::send_message(handle, WM_SIZE, 0, 0);
            }

            None
        });

        *self.handler0.borrow_mut() = Some(handler.unwrap());
    }

}

impl Drop for StatusBar {
    fn drop(&mut self) {
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            drop(unbind_raw_event_handler(h));
        }
        self.handle.destroy();
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

        *out = Default::default();

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(out.flags())
            .parent(Some(parent))
            .build()?;

        if self.font.is_some() {
            out.set_font(self.font);
        } else {
            out.set_font(Font::global_default().as_ref());
        }

        out.set_text(0, self.text);
        out.hook_parent_resize();

        Ok(())
    }

}

impl PartialEq for StatusBar {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
