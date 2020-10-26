use winapi::um::winuser::{WS_VISIBLE};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::win32::richedit as rich;
use crate::{Font, Cursor, OemCursor, NwgError, RawEventHandler, unbind_raw_event_handler};
use super::{ControlBase, ControlHandle, RichTextBox, CharFormat};

use std::mem;
use std::cell::RefCell;


const NOT_BOUND: &'static str = "RichLabel is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: RichLabel handle is not HWND!";

bitflags! {
    /**
        The rich label flags

        * VISIBLE:  The rich text box is immediatly visible after creation
    */
    pub struct RichLabelFlags: u32 {
        const VISIBLE = WS_VISIBLE;
    }
}


/**
A rich label is a label that supports rich text. This control is built on top of the rich text box control and as such
require the `rich-textbox` feature.

Unlike the basic `Label`, this version supports:

* Colored text
* Multiple fonts
* Styled text such as bold, underscore, strikeout, etc
* Bullet point list
* Paragraph with custom indent/offset
* Custom line spacing

**Builder parameters:**
  * `parent`:           **Required.** The label parent container.
  * `text`:             The label text.

**Control events:**

  * `MousePress(_)`: Generic mouse press events on the label
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event

** Example **

```rust
use native_windows_gui as nwg;
fn build_label(label: &mut nwg::RichLabel, window: &nwg::Window, font: &nwg::Font) {
    nwg::RichLabel::builder()
        .text("Hello")
        .font(Some(font))
        .parent(window)
        .build(label);
}

*/
#[derive(Default)]
pub struct RichLabel {
    pub handle: ControlHandle,
    handler0: RefCell<Option<RawEventHandler>>,
}

impl RichLabel {

    pub fn builder<'a>() -> RichLabelBuilder<'a> {
        RichLabelBuilder {
            text: "A rich label",
            size: (130, 25),
            position: (0, 0),
            flags: None,
            font: None,
            parent: None,
        }
    }

    /// Sets the background color for a rich edit control.
    /// You cannot get the background color of a rich label
    pub fn set_background_color(&self, color: [u8; 3]) {
        use winapi::um::wingdi::RGB;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let color = RGB(color[0], color[1], color[2]);
        wh::send_message(handle, rich::EM_SETBKGNDCOLOR, 0, color as _);
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

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the text displayed in the TextInput
    pub fn text(&self) -> String { 
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the text displayed in the TextInput
    pub fn set_text<'a>(&self, v: &'a str) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "RICHEDIT50W"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{ES_READONLY, WS_CHILD};
        
        ES_READONLY | WS_CHILD
    }

    fn override_events(&self) {
        use crate::bind_raw_event_handler_inner;
        use winapi::um::winuser::{SetCursor, WM_MOUSEFIRST, WM_MOUSELAST, WM_MOUSEACTIVATE, WM_KEYUP, WM_KEYDOWN, WM_SETFOCUS};
        use winapi::shared::windef::HCURSOR;

        let cursor = Cursor::from_system(OemCursor::Normal);
        let handler0 = bind_raw_event_handler_inner(&self.handle, 0, move |_hwnd, msg, _w, _l| {
            unsafe { SetCursor(cursor.handle as HCURSOR); }
            match msg {
                WM_MOUSEFIRST..=WM_MOUSELAST => Some(0),
                WM_MOUSEACTIVATE | WM_KEYUP | WM_KEYDOWN | WM_SETFOCUS => Some(0),
                _ => None
            }            
        });

        *self.handler0.borrow_mut() = Some(handler0.unwrap());
    }

}

impl PartialEq for RichLabel {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Drop for RichLabel {
    fn drop(&mut self) {
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            drop(unbind_raw_event_handler(h));
        }

        self.handle.destroy();
    }
}

pub struct RichLabelBuilder<'a> {
    text: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<RichLabelFlags>,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}

impl<'a> RichLabelBuilder<'a> {

    pub fn text(mut self, text: &'a str) -> RichLabelBuilder<'a> {
        self.text = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> RichLabelBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> RichLabelBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> RichLabelBuilder<'a> {
        self.font = font;
        self
    }

    pub fn flags(mut self, flags: RichLabelFlags) -> RichLabelBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> RichLabelBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut RichLabel) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("RichTextBox"))
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .text(self.text)
            .parent(Some(parent))
            .build()?;

        if self.font.is_some() {
            out.set_font(self.font);
        } else {
            out.set_font(Font::global_default().as_ref());
        }

        if let Ok(color) = wh::get_background_color(parent.hwnd().unwrap()) {
            out.set_background_color(color);
        }

        out.override_events();

        /*{
            let rich = RichTextBox { handle: out.handle };
            let mut fmt = CharFormat::default();
            fmt.text_color = Some([0,0,0]);
            rich.set_char_format(&fmt);
            mem::forget(rich);

        }*/
        
        Ok(())
    }

}
