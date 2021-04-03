use winapi::um::{
    winuser::{WS_VISIBLE, WS_DISABLED, BS_AUTOCHECKBOX, BS_AUTO3STATE, BS_PUSHLIKE, WS_TABSTOP},
    wingdi::DeleteObject
};
use winapi::shared::windef::HBRUSH;
use crate::win32::{base_helper::check_hwnd, window_helper as wh};
use crate::{Font, NwgError, RawEventHandler};
use super::{ControlBase, ControlHandle};
use std::cell::RefCell;

const NOT_BOUND: &'static str = "CheckBox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: CheckBox handle is not HWND!";


bitflags! {
    /**
        The CheckBox flags

        * NONE:     No flags. Equivalent to a invisible default checkbox.
        * VISIBLE:  The checkbox is immediatly visible after creation
        * DISABLED: The checkbox cannot be interacted with by the user. It also has a grayed out look.
        * TRISTATE: The checkbox will have a 3rd state
        * PUSHLIKE: The checkbox will look like a regular button
        * TAB_STOP: The control can be selected using tab navigation
    */
    pub struct CheckBoxFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TRISTATE = BS_AUTO3STATE;
        const PUSHLIKE = BS_PUSHLIKE;
        const TAB_STOP = WS_TABSTOP;
    }
}

/// Represents the check status of a checkbox
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CheckBoxState {
    Checked,
    Unchecked,

    /// New state for the tristate checkbox
    Indeterminate
}

/**
A check box consists of a square box and an application-defined labe that indicates a choice the user can make by selecting the button.
Applications typically display check boxes to enable the user to choose one or more options that are not mutually exclusive.

CheckBox is not behind any features.

**Builder parameters:**
  * `parent`:           **Required.** The checkbox parent container.
  * `text`:             The checkbox text.
  * `size`:             The checkbox size.
  * `position`:         The checkbox position.
  * `enabled`:          If the checkbox can be used by the user. It also has a grayed out look if disabled.
  * `flags`:            A combination of the CheckBoxFlags values.
  * `ex_flags`:         A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `font`:             The font used for the checkbox text
  * `background_color`: The background color of the checkbox. Defaults to the default window background (light gray)
  * `check_state`:      The default check state
  * `focus`:            The control receive focus after being created

**Control events:**
  * `OnButtonClick`: When the checkbox is clicked once by the user
  * `OnButtonDoubleClick`: When the checkbox is clicked twice rapidly by the user
  * `MousePress(_)`: Generic mouse press events on the checkbox
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event


```rust
use native_windows_gui as nwg;
fn build_checkbox(button: &mut nwg::CheckBox, window: &nwg::Window, font: &nwg::Font) {
    nwg::CheckBox::builder()
        .text("Hello")
        .flags(nwg::CheckBoxFlags::VISIBLE)
        .font(Some(font))
        .parent(window)
        .build(button);
}
```
*/
#[derive(Default)]
pub struct CheckBox {
    pub handle: ControlHandle,
    background_brush: Option<HBRUSH>,
    handler0: RefCell<Option<RawEventHandler>>,
}

impl CheckBox {

    pub fn builder<'a>() -> CheckBoxBuilder<'a> {
        CheckBoxBuilder {
            text: "A checkbox",
            size: (100, 25),
            position: (0, 0),
            enabled: true,
            focus: false,
            background_color: None,
            check_state: CheckBoxState::Unchecked,
            flags: None,
            ex_flags: 0,
            font: None,
            parent: None,
        }
    }

    /// Return `true` if the checkbox can have a third state or `false` otherwise
    pub fn tristate(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let style = wh::get_style(handle);
        style & BS_AUTO3STATE == BS_AUTO3STATE
    }

    /// Sets or unsets the checkbox as tristate
    pub fn set_tristate(&self, tri: bool) {
        use winapi::um::winuser::{BM_SETSTYLE};
        use winapi::shared::minwindef::WPARAM;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        
        let style = match tri {
            true => BS_AUTO3STATE,
            false => BS_AUTOCHECKBOX
        };

        wh::send_message(handle, BM_SETSTYLE, style as WPARAM, 1);
    }

    /// Return the check state of the check box
    pub fn check_state(&self) -> CheckBoxState {
        use winapi::um::winuser::{BM_GETCHECK, BST_CHECKED, BST_INDETERMINATE, BST_UNCHECKED};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        match wh::send_message(handle, BM_GETCHECK, 0, 0) as usize {
            BST_UNCHECKED => CheckBoxState::Unchecked,
            BST_CHECKED => CheckBoxState::Checked,
            BST_INDETERMINATE => CheckBoxState::Indeterminate,
            _ => unreachable!()
        }
    }

    /// Sets the check state of the check box
    pub fn set_check_state(&self, state: CheckBoxState) {
        use winapi::um::winuser::{BM_SETCHECK, BST_CHECKED, BST_INDETERMINATE, BST_UNCHECKED};
        use winapi::shared::minwindef::WPARAM;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let x = match state {
            CheckBoxState::Unchecked => BST_UNCHECKED,
            CheckBoxState::Checked => BST_CHECKED,
            CheckBoxState::Indeterminate => BST_INDETERMINATE,
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

    /// Return the size of the check box in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the check box in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the check box in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the check box in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the check box label
    pub fn text(&self) -> String { 
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the check box label
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
        WS_VISIBLE | WS_TABSTOP
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{BS_NOTIFY, WS_CHILD};

        BS_NOTIFY | WS_CHILD
    }

    /// Change the checkbox background color.
    fn hook_background_color(&mut self, c: [u8; 3]) {
        use crate::bind_raw_event_handler_inner;
        use winapi::um::winuser::{WM_CTLCOLORSTATIC};
        use winapi::shared::{basetsd::UINT_PTR, windef::HWND, minwindef::LRESULT};
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

impl Drop for CheckBox {
    fn drop(&mut self) {
        use crate::unbind_raw_event_handler;
        
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

pub struct CheckBoxBuilder<'a> {
    text: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    focus: bool,
    background_color: Option<[u8; 3]>,
    check_state: CheckBoxState,
    flags: Option<CheckBoxFlags>,
    ex_flags: u32,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}

impl<'a> CheckBoxBuilder<'a> {

    pub fn flags(mut self, flags: CheckBoxFlags) -> CheckBoxBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> CheckBoxBuilder<'a> {
        self.ex_flags = flags;
        self
    }

    pub fn text(mut self, text: &'a str) -> CheckBoxBuilder<'a> {
        self.text = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> CheckBoxBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> CheckBoxBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn enabled(mut self, e: bool) -> CheckBoxBuilder<'a> {
        self.enabled = e;
        self
    }

    pub fn focus(mut self, focus: bool) -> CheckBoxBuilder<'a> {
        self.focus = focus;
        self
    }

    pub fn check_state(mut self, check: CheckBoxState) -> CheckBoxBuilder<'a> {
        self.check_state = check;
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> CheckBoxBuilder<'a> {
        self.background_color = color;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> CheckBoxBuilder<'a> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> CheckBoxBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut CheckBox) -> Result<(), NwgError> {
        let mut flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());
        if flags & BS_AUTO3STATE == 0 {
            flags |= BS_AUTOCHECKBOX;
        }

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("CheckBox"))
        }?;

        // Drop the old object
        *out = CheckBox::default();

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

        out.set_enabled(self.enabled);

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

impl PartialEq for CheckBox {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
