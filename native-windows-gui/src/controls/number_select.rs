use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_TABSTOP, WS_EX_CONTROLPARENT};
use std::cell::RefCell;
use std::rc::Rc;

use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::{NwgError, Font, RawEventHandler, bind_raw_event_handler_inner, unbind_raw_event_handler};
use super::{ControlBase, ControlHandle, TextInput, Button, ButtonFlags, TextInputFlags};

const NOT_BOUND: &'static str = "UpDown is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: UpDown handle is not HWND!";


bitflags! {
    /**
        The NumberSelect flags

        * NONE:     No flags. Equivalent to a invisible blank NumberSelect.
        * VISIBLE:  The NumberSelect is immediatly visible after creation
        * DISABLED: The NumberSelect cannot be interacted with by the user. It also has a grayed out look.
        * TAB_STOP: The control can be selected using tab navigation. 
    */
    pub struct NumberSelectFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;
    }
}

/// The value inside a number select and the limits of that value
#[derive(Copy, Clone, Debug)]
pub enum NumberSelectData {
    Int { value: i64, step: i64, max: i64, min: i64 },
    Float { value: f64, step: f64, max: f64, min: f64, decimals: u8 },
}

impl NumberSelectData {

    pub fn formatted_value(&self) -> String {
        match self {
            NumberSelectData::Int{ value, ..} => format!("{}", value),
            NumberSelectData::Float{ value, decimals, ..} => format!("{:.*}", *decimals as usize, value),
        }
    }

    pub fn decrease(&mut self) {
        match self {
            NumberSelectData::Int{ value, step, min, ..} => {
                *value -= *step;
                *value = i64::max(*value, *min);
            },
            NumberSelectData::Float{ value, step, min, ..} => {
                *value -= *step;
                *value = f64::max(*value, *min);
            }
        }
    }

    pub fn increase(&mut self) {
        match self {
            NumberSelectData::Int{ value, step, max, ..} => {
                *value += *step;
                *value = i64::min(*value, *max);
            },
            NumberSelectData::Float{ value, step, max, ..} => {
                *value += *step;
                *value = f64::min(*value, *max);
            }
        }
    }

}

impl Default for NumberSelectData {
    fn default() -> NumberSelectData {
        NumberSelectData::Int { 
            value: 0,
            step: 1,
            max: i64::max_value(),
            min: i64::min_value(),
        }
    }
}

/**
A NumberSelect control is a pair of arrow buttons that the user can click to increment or decrement a value.
NumberSelect is implemented as a custom control because the one provided by winapi really sucks.

Requires the `number-select` feature. 

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
    data: Rc<RefCell<NumberSelectData>>,
    edit: TextInput,
    btn_up: Button,
    btn_down: Button,
    handler: Option<RawEventHandler>
}

impl NumberSelect {

    pub fn builder<'a>() -> NumberSelectBuilder<'a> {
        NumberSelectBuilder {
            size: (100, 25),
            position: (0, 0),
            data: NumberSelectData::default(),
            enabled: true,
            flags: None,
            font: None,
            parent: None
        }
    }

    /// Returns inner data specifying the possible input of a number select
    /// See [NumberSelectData](enum.NumberSelectData.html) for the possible values
    pub fn data(&self) -> NumberSelectData {
        self.data.borrow().clone()
    }

    /// Sets the inner data specifying the possible input of a number select. Also update the value display.
    /// See [NumberSelectData](enum.NumberSelectData.html) for the possible values
    pub fn set_data(&self, v: NumberSelectData) {
        *self.data.borrow_mut() = v;
        self.edit.set_text(&v.formatted_value());
    }

    /// Returns the font of the control
    pub fn font(&self) -> Option<Font> {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Sets the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Returns true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Sets the keyboard focus on the button.
    pub fn set_focus(&self) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Returns true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Returns true if the control is visible to the user. Will return true even if the 
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

    /// Returns the size of the control in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Sets the size of the control in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Returns the position of the control in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Sets the position of the control in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
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

impl Drop for NumberSelect {

    fn drop(&mut self) {
        if let Some(h) = self.handler.as_ref() {
            drop(unbind_raw_event_handler(h));
        }

        self.handle.destroy();
    }

}

pub struct NumberSelectBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    data: NumberSelectData,
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

    // Int values
    pub fn value_int(mut self, v: i64) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Int { value, .. } => { *value = v; }
            data => *data = NumberSelectData::Int { value: v, step: 1, max: i64::max_value(), min: i64::min_value() }
        }
        self
    }

    pub fn step_int(mut self, v: i64) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Int { step, .. } => { *step = v; }
            data => *data = NumberSelectData::Int { value: 0, step: v, max: i64::max_value(), min: i64::min_value() }
        }
        self
    }

    pub fn max_int(mut self, v: i64) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Int { max, .. } => { *max = v; }
            data => *data = NumberSelectData::Int { value: 0, step: 1, max: v, min: i64::min_value() }
        }
        self
    }

    pub fn min_int(mut self, v: i64) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Int { min, .. } => { *min = v; }
            data => *data = NumberSelectData::Int { value: 0, step: 1, max: i64::max_value(), min: v }
        }
        self
    }

    // Float values
    pub fn value_float(mut self, v: f64) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Float { value, .. } => { *value = v; }
            data => *data = NumberSelectData::Float { value: v, step: 1.0, max: 1000000.0, min: -1000000.0, decimals: 2 }
        }
        self
    }

    pub fn step_float(mut self, v: f64) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Float { step, .. } => { *step = v; }
            data => *data = NumberSelectData::Float { value: 0.0, step: v, max: 1000000.0, min: -1000000.0, decimals: 2 }
        }
        self
    }

    pub fn max_float(mut self, v: f64) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Float { max, .. } => { *max = v; }
            data => *data = NumberSelectData::Float { value: 0.0, step: 1.0, max: v, min: -1000000.0, decimals: 2 }
        }
        self
    }

    pub fn min_float(mut self, v: f64) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Float { min, .. } => { *min = v; }
            data => *data = NumberSelectData::Float { value: 0.0, step: 1.0, max: 1000000.0, min: v, decimals: 2 }
        }
        self
    }

    pub fn decimals(mut self, v: u8) -> NumberSelectBuilder<'a> {
        match &mut self.data {
            NumberSelectData::Float { decimals, .. } => { *decimals = v; }
            data => *data = NumberSelectData::Float { value: 0.0, step: 1.0, max: 1000000.0, min: -1000000.0, decimals: v }
        }
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> NumberSelectBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut NumberSelect) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());
        let (btn_flags, text_flags) = if flags & WS_TABSTOP == WS_TABSTOP {
            (ButtonFlags::VISIBLE | ButtonFlags::TAB_STOP, TextInputFlags::VISIBLE | TextInputFlags::TAB_STOP)
        } else {
            (ButtonFlags::VISIBLE, TextInputFlags::VISIBLE)
        };

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("NumberSelect"))
        }?;

        *out = Default::default();

        let (w, h) = self.size;

        if out.handler.is_some() {
            unbind_raw_event_handler(out.handler.as_ref().unwrap())?;
        }

        *out = NumberSelect::default();
        *out.data.borrow_mut() = self.data;
        
        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .ex_flags(WS_EX_CONTROLPARENT)
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        TextInput::builder()
            .text(&self.data.formatted_value())
            .size((w-19, h))
            .parent(&out.handle)
            .flags(text_flags)
            .build(&mut out.edit)?;

        Button::builder()
            .text("+")
            .size((20, h/2+1))
            .position((w-20, -1))
            .parent(&out.handle)
            .flags(btn_flags)
            .build(&mut out.btn_up)?;

        Button::builder()
            .text("-")
            .size((20, h/2+1))
            .position((w-20, (h/2)-1))
            .parent(&out.handle)
            .flags(btn_flags)    
            .build(&mut out.btn_down)?;

        if self.font.is_some() {
            out.btn_up.set_font(self.font);
            out.btn_down.set_font(self.font);
            out.edit.set_font(self.font);
        } else {
            let font = Font::global_default();
            let font_ref = font.as_ref();
            out.btn_up.set_font(font_ref);
            out.btn_down.set_font(font_ref);
            out.edit.set_font(font_ref);
        }

        let handler_data = out.data.clone();
        let plus_button = out.btn_up.handle.clone();
        let minus_button = out.btn_down.handle.clone();
        let text_handle = out.edit.handle.clone();

        let handler = bind_raw_event_handler_inner(&out.handle, 0x4545, move |_hwnd, msg, w, l| {
            use winapi::shared::windef::HWND;
            use winapi::um::winuser::{WM_COMMAND, BN_CLICKED};
            use winapi::shared::minwindef::HIWORD;
            
            match msg {
                WM_COMMAND => {
                    let handle = ControlHandle::Hwnd(l as HWND);
                    let message = HIWORD(w as u32) as u16;
                    if message == BN_CLICKED && handle == plus_button {
                        let mut data = handler_data.borrow_mut();
                        data.increase();

                        let handle = text_handle.hwnd().unwrap();
                        let text = data.formatted_value();
                        unsafe { wh::set_window_text(handle, &text); }
                    } else if message == BN_CLICKED && handle == minus_button {
                        let mut data = handler_data.borrow_mut();
                        data.decrease();

                        let handle = text_handle.hwnd().unwrap();
                        let text = data.formatted_value();
                        unsafe { wh::set_window_text(handle, &text); }
                    }
                },
                
                _ => {}
            }
            None
        });

        out.handler = Some(handler.unwrap());

        if !self.enabled {
            out.set_enabled(self.enabled);
        }

        Ok(())
    }

}
