use winapi::um::{
    winuser::{WS_VISIBLE, WS_DISABLED, WS_GROUP, WS_TABSTOP},
    wingdi::DeleteObject
};
use winapi::shared::windef::HBRUSH;
use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::{Font, NwgError, RawEventHandler, unbind_raw_event_handler};
use super::{ControlBase, ControlHandle};
use std::cell::RefCell;

const NOT_BOUND: &'static str = "RadioButton is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: RadioButton handle is not HWND!";

bitflags! {
    /**
        The radio button flags

        * VISIBLE:  The radio button is immediatly visible after creation
        * DISABLED: The radio button cannot be interacted with by the user. It also has a grayed out look.
        * TAB_STOP: The radio button can be selected using tab navigation
        * GROUP:    Creates a new radio button group
    */
    pub struct RadioButtonFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;
        const GROUP = WS_GROUP;
    }
}

/// Represents the check status of a radio button
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RadioButtonState {
    Checked,
    Unchecked
}

/**
A radio button (also called option button) consists of a round button and an application-defined label, 
icon, or bitmap that indicates a choice the user can make by selecting the button. An application typically uses radio buttons in a group box to enable the user to choose one of a set of related but mutually exclusive options.

Radiobutton is not behind any features.

Note: Internally, radio buttons are `Button` and as such, they trigger the same events

**Builder parameters:**
  * `parent`:           **Required.** The radio button parent container.
  * `text`:             The radio button text.
  * `size`:             The radio button size.
  * `position`:         The radio button position.
  * `enabled`:          If the radio button can be used by the user. It also has a grayed out look if disabled.
  * `flags`:            A combination of the RadioButtonFlags values.
  * `ex_flags`:         A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi  
  * `font`:             The font used for the radio button text
  * `background_color`: The background color of the radio button. Defaults to the default window background (light gray)
  * `check_state`:      The default check state

**Control events:**
  * `OnButtonClick`: When the adio button is clicked once by the user
  * `OnButtonDoubleClick`: When the adio button is clicked twice rapidly by the user
  * `MousePress(_)`: Generic mouse press events on the adio button
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event


```rust
use native_windows_gui as nwg;

/// Build two group of checkboxes on the same parent with the GROUP flags
fn build_radio_groups(radios: &mut [nwg::RadioButton], parent: &nwg::Window) {
    use nwg::RadioButtonFlags as RadioF;

    // Group 1
    nwg::RadioButton::builder()
      .flags(RadioF::VISIBLE | RadioF::GROUP)
      .parent(parent)
      .build(&mut radios[0]);

    nwg::RadioButton::builder()
      .parent(parent)
      .build(&mut radios[1]);

    // Group 2
    nwg::RadioButton::builder()
      .flags(RadioF::VISIBLE | RadioF::GROUP)
      .parent(parent)
      .build(&mut radios[2]);

    nwg::RadioButton::builder()
      .parent(parent)
      .build(&mut radios[3]);
}


```

```rust
use native_windows_gui as nwg;
fn build_radio(radio: &mut nwg::RadioButton, window: &nwg::Window, font: &nwg::Font) {
    nwg::RadioButton::builder()
        .text("Hello")
        .flags(nwg::RadioButtonFlags::VISIBLE)
        .font(Some(font))
        .parent(window)
        .build(radio);
}
```

*/
#[derive(Default)]
pub struct RadioButton {
    pub handle: ControlHandle,
    background_brush: Option<HBRUSH>,
    handler0: RefCell<Option<RawEventHandler>>,
}

impl RadioButton {

    pub fn builder<'a>() -> RadioButtonBuilder<'a> {
        RadioButtonBuilder {
            text: "A radio button",
            size: (100, 25),
            position: (0, 0),
            focus: false,
            background_color: None,
            check_state: RadioButtonState::Unchecked,
            flags: None,
            ex_flags: 0,
            font: None,
            parent: None
        }
    }

    /// Return the check state of the check box
    pub fn check_state(&self) -> RadioButtonState {
        use winapi::um::winuser::{BM_GETCHECK, BST_CHECKED, BST_UNCHECKED};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        match wh::send_message(handle, BM_GETCHECK, 0, 0) as usize {
            BST_UNCHECKED => RadioButtonState::Unchecked,
            BST_CHECKED => RadioButtonState::Checked,
            _ => unreachable!()
        }
    }

    /// Sets the check state of the check box
    pub fn set_check_state(&self, state: RadioButtonState) {
        use winapi::um::winuser::{BM_SETCHECK, BST_CHECKED, BST_UNCHECKED};
        use winapi::shared::minwindef::WPARAM;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let x = match state {
            RadioButtonState::Unchecked => BST_UNCHECKED,
            RadioButtonState::Checked => BST_CHECKED,
        };

        wh::send_message(handle, BM_SETCHECK, x as WPARAM, 0);
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

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button.
    pub fn set_focus(&self) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
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

    /// Return the size of the radio button in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the radio button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the radio button in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the radio button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the radio button label
    pub fn text(&self) -> String { 
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the radio button label
    pub fn set_text<'a>(&self, v: &'a str) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "BUTTON"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{BS_NOTIFY, WS_CHILD, BS_AUTORADIOBUTTON};

        BS_NOTIFY | WS_CHILD | BS_AUTORADIOBUTTON
    }

    /// Change the radio button background color.
    fn hook_background_color(&mut self, c: [u8; 3]) {
        use crate::bind_raw_event_handler_inner;
        use winapi::um::winuser::{WM_CTLCOLORSTATIC};
        use winapi::shared::{basetsd::UINT_PTR, windef::{HWND}, minwindef::LRESULT};
        use winapi::um::wingdi::{CreateSolidBrush, RGB};

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let parent_handle = ControlHandle::Hwnd(wh::get_window_parent(handle));
        let brush = unsafe { CreateSolidBrush(RGB(c[0], c[1], c[2])) };
        self.background_brush = Some(brush);
        
        let handler = bind_raw_event_handler_inner(&parent_handle, handle as UINT_PTR, move |_hwnd, msg, _w, l| {
            match msg {
                WM_CTLCOLORSTATIC => {
                    let child = l as HWND;
                    if child == handle {
                        return Some(brush as LRESULT);
                    }
                },
                _ => {}
            }

            None
        });

        *self.handler0.borrow_mut() = Some(handler.unwrap());
    }

}

impl Drop for RadioButton {
    fn drop(&mut self) {
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            drop(unbind_raw_event_handler(h));
        }

        if let Some(bg) = self.background_brush {
            unsafe { DeleteObject(bg as _); }
        }

        self.handle.destroy();
    }
}

pub struct RadioButtonBuilder<'a> {
    text: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    focus: bool,
    background_color: Option<[u8; 3]>,
    check_state: RadioButtonState,
    flags: Option<RadioButtonFlags>,
    ex_flags: u32,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}

impl<'a> RadioButtonBuilder<'a> {

    pub fn flags(mut self, flags: RadioButtonFlags) -> RadioButtonBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> RadioButtonBuilder<'a> {
        self.ex_flags = flags;
        self
    }

    pub fn text(mut self, text: &'a str) -> RadioButtonBuilder<'a> {
        self.text = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> RadioButtonBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> RadioButtonBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn focus(mut self, focus: bool) -> RadioButtonBuilder<'a> {
        self.focus = focus;
        self
    }

    pub fn check_state(mut self, check: RadioButtonState) -> RadioButtonBuilder<'a> {
        self.check_state = check;
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> RadioButtonBuilder<'a> {
        self.background_color = color;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> RadioButtonBuilder<'a> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> RadioButtonBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut RadioButton) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("RadioButton"))
        }?;

        *out = Default::default();

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .ex_flags(self.ex_flags)
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

        if self.background_color.is_some() {
            out.hook_background_color(self.background_color.unwrap());
        }

        if self.focus {
            out.set_focus();
        }

        out.set_check_state(self.check_state);

        Ok(())
    }

}

impl PartialEq for RadioButton {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
