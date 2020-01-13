use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use crate::win32::window_helper as wh;
use crate::{NwgError, Font, bind_raw_event_handler};
use super::{ControlBase, ControlHandle, TextInput, Button};

const NOT_BOUND: &'static str = "UpDown is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: UpDown handle is not HWND!";


bitflags! {
    /**
        The NumberSelect flags

        * NONE:     No flags. Equivalent to a invisible blank NumberSelect.
        * VISIBLE:  The NumberSelect is immediatly visible after creation
        * DISABLED: The NumberSelect cannot be interacted with by the user. It also has a grayed out look.
    */
    pub struct NumberSelectFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/**
A NumberSelect control is a pair of arrow buttons that the user can click to increment or decrement a value.
NumberSelect is implemented as a custom control.

**Builder parameters:**
  * `parent`:   **Required.** The number select parent container.
  * `value`:    The default value of the number select
  * `size`:     The number select size.
  * `position`: The number select position.
  * `enabled`:  If the number select can be used by the user. It also has a grayed out look if disabled.
  * `flags`:    A combination of the NumberSelectFlags values.
  * `font`:     The font used for the number select text

**Control events:**
  * `MousePress(_)`: Generic mouse press events on the button
  * `OnMouseMove`: Generic mouse mouse event

```rust
use native_windows_gui as nwg;
fn build_number_select(num_select: &mut nwg::NumberSelect, window: &nwg::Window, font: &nwg::Font) {
    nwg::NumberSelect::builder()
        .font(Some(font))
        .parent(window)
        .build(num_select);
}
```

*/
#[derive(Default)]
pub struct NumberSelect {
    pub handle: ControlHandle,
    edit: TextInput,
    btn_up: Button,
    btn_down: Button,
}

impl NumberSelect {

    pub fn builder<'a>() -> NumberSelectBuilder<'a> {
        NumberSelectBuilder {
            size: (100, 25),
            position: (0, 0),
            enabled: true,
            flags: None,
            font: None,
            parent: None
        }
    }

    /// Returns the font of the control
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

    /// Sets the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
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

    /// Returns the size of the control in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Sets the size of the control in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Returns the position of the control in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Sets the position of the control in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "NativeWindowsGuiWindow"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_BORDER, WS_CHILD, WS_CLIPCHILDREN};
        WS_CHILD | WS_BORDER | WS_CLIPCHILDREN
    }

}

pub struct NumberSelectBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    flags: Option<NumberSelectFlags>,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}

impl<'a> NumberSelectBuilder<'a> {

    pub fn flags(mut self, flags: NumberSelectFlags) -> NumberSelectBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> NumberSelectBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> NumberSelectBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn enabled(mut self, e: bool) -> NumberSelectBuilder<'a> {
        self.enabled = e;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> NumberSelectBuilder<'a> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> NumberSelectBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut NumberSelect) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("NumberSelect"))
        }?;

        let (w, h) = self.size;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        TextInput::builder()
            .size((w-19, h))
            .parent(&out.handle)
            .build(&mut out.edit)?;

        Button::builder()
            .text("+")
            .size((20, h/2+1))
            .position((w-20, -1))
            .parent(&out.handle)
            .build(&mut out.btn_up)?;

        Button::builder()
            .text("-")
            .size((20, h/2+1))
            .position((w-20, (h/2)-1))
            .parent(&out.handle)
            .build(&mut out.btn_down)?;

        if self.font.is_some() {
            out.btn_up.set_font(self.font);
            out.btn_down.set_font(self.font);
            out.edit.set_font(self.font);
        }

        bind_raw_event_handler(&out.handle, 0, move |_hwnd, msg, _w, _l| {
            match msg {
                _ => None,
            }
        });

        Ok(())
    }

}
