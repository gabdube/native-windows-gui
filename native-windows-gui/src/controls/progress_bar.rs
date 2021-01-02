/*!
A push button is a rectangle containing an application-defined text label, an icon, or a bitmap
that indicates what the button does when the user selects it.
*/

use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use winapi::um::commctrl::{PBS_MARQUEE, PBS_VERTICAL};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::NwgError;
use super::{ControlHandle, ControlBase};
use std::ops::Range;

const NOT_BOUND: &'static str = "Progress bar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Progress bar handle is not HWND!";


bitflags! {
    pub struct ProgressBarFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const VERTICAL = PBS_VERTICAL;
        const MARQUEE = PBS_MARQUEE;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ProgressBarState {
    Normal,
    Error,
    Paused
}

/**
A progress bar is a window that an application can use to indicate the progress of a lengthy operation.

Requires the `progress-bar` feature. 

**Builder parameters:**
  * `parent`:         **Required.** The progress bar parent container.
  * `size`:           The progress bar size.
  * `position`:       The progress bar position.
  * `state`:          The initial state of the progress bar.
  * `step`:           The value in which the progress bar value increase when `advance` is used.
  * `pos`:            The initial value of the progress bar.
  * `range`:          The minimum and maximum value in the progress bar.
  * `enabled`:        If the progress bar is enabled. 
  * `flags`:          A combination of the ProgressBarFlags values.
  * `ex_flags`:       A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `marquee`:        Enable of disable the marquee animation (only used with the MARQUEE flags)
  * `marquee_update`: The update interval of the marquee mode

**Control events:**
  * `MousePress(_)`: Generic mouse press events on the progress bar
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event

```rust
use native_windows_gui as nwg;
fn build_progress_bar(bar: &mut nwg::ProgressBar, window: &nwg::Window) {
    nwg::ProgressBar::builder()
        .state(nwg::ProgressBarState::Paused)
        .step(10)
        .range(0..100)
        .parent(window)
        .build(bar);
}
```

*/
#[derive(Default, PartialEq, Eq)]
pub struct ProgressBar {
    pub handle: ControlHandle
}

impl ProgressBar {

    pub fn builder() -> ProgressBarBuilder {
        ProgressBarBuilder {
            size: (100, 40),
            position: (0, 0),
            flags: None,
            ex_flags: 0,
            state: ProgressBarState::Normal,
            step: 1,
            pos: 0,
            range: 0..100,
            marquee_enable: false,
            marquee_update: 0,
            parent: None
        }
    }

    /// Return the current state of the progress bar
    pub fn state(&self) -> ProgressBarState {
        use winapi::um::commctrl::{PBM_GETSTATE, PBST_NORMAL, PBST_ERROR, PBST_PAUSED};
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        match wh::send_message(handle, PBM_GETSTATE, 0, 0) as i32 {
            PBST_NORMAL => ProgressBarState::Normal,
            PBST_ERROR => ProgressBarState::Error,
            PBST_PAUSED => ProgressBarState::Paused,
            _ => panic!("Unkown progress bar state")
        }
    }

    /// Set the state of the progress bar
    pub fn set_state(&self, state: ProgressBarState) {
        use winapi::um::commctrl::{PBM_SETSTATE, PBST_NORMAL, PBST_ERROR, PBST_PAUSED};
        use winapi::shared::minwindef::WPARAM;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let state = match state {
            ProgressBarState::Normal => PBST_NORMAL,
            ProgressBarState::Error => PBST_ERROR,
            ProgressBarState::Paused => PBST_PAUSED
        };

        wh::send_message(handle, PBM_SETSTATE, state as WPARAM, 0);
    }

    /// Increase the bar value by the step value
    pub fn advance(&self) {
        use winapi::um::commctrl::PBM_STEPIT;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, PBM_STEPIT, 0, 0);
    }

    /// Increase the bar value by a value
    pub fn advance_delta(&self, v: u32) {
        use winapi::um::commctrl::PBM_DELTAPOS;
        use winapi::shared::minwindef::WPARAM;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, PBM_DELTAPOS, v as WPARAM, 0);
    }

    /// Return the step of the progress bar.
    pub fn step(&self) -> u32 {
        use winapi::um::commctrl::PBM_GETSTEP;
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, PBM_GETSTEP, 0, 0) as u32
    }

    /// Set the step of the progress bar.
    pub fn set_step(&self, s: u32) {
        use winapi::um::commctrl::PBM_SETSTEP;
        use winapi::shared::minwindef::WPARAM;
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, PBM_SETSTEP, s as WPARAM, 0);
    }

    /// Return the position of the progress bar.
    pub fn pos(&self) -> u32 {
        use winapi::um::commctrl::PBM_GETPOS;
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, PBM_GETPOS, 0, 0) as u32
    }

    /// Set the position of the progress bar. If the value is outside of range
    /// sets the value to the nearest bound.
    pub fn set_pos(&self, p: u32) {
        use winapi::um::commctrl::PBM_SETPOS;
        use winapi::shared::minwindef::WPARAM;
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, PBM_SETPOS, p as WPARAM, 0);
    }

    /// Get the range of the progress bar
    pub fn range(&self) -> Range<u32> {
        use winapi::um::commctrl::PBM_GETRANGE;
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        
        let low = wh::send_message(handle, PBM_GETRANGE, 1, 0) as u32;
        let high = wh::send_message(handle, PBM_GETRANGE, 0, 0) as u32;

        low..high
    }

    /// Set the range of the progress bar
    pub fn set_range(&self, range: Range<u32>) {
        use winapi::um::commctrl::PBM_SETRANGE32;
        use winapi::shared::minwindef::{WPARAM, LPARAM};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, PBM_SETRANGE32, range.start as WPARAM, range.end as LPARAM);
    }

    /// Set the progress bar marquee mode
    pub fn set_marquee(&self, enable: bool, update_interval: u32) {
        use winapi::shared::minwindef::{LPARAM, WPARAM};
        use winapi::um::commctrl::PBM_SETMARQUEE;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, PBM_SETMARQUEE, enable as WPARAM, update_interval as LPARAM);
    }

    /// Updates the flags of the progress bar.
    pub fn add_flags(&self, styles: ProgressBarFlags) {
        let styles = styles.bits() as u32;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let active_styles = wh::get_style(handle);
        
        wh::set_style(handle, active_styles | styles);
    }

    /// Removes flags from the progress bar.
    pub fn remove_flags(&self, styles: ProgressBarFlags) {
        let styles = styles.bits() as u32;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let active_styles = wh::get_style(handle);

        wh::set_style(handle, active_styles & !styles);
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

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "msctls_progress32"
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

}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

pub struct ProgressBarBuilder {
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<ProgressBarFlags>,
    ex_flags: u32,
    state: ProgressBarState,
    step: u32,
    pos: u32,
    range: Range<u32>,
    marquee_enable: bool,
    marquee_update: u32,
    parent: Option<ControlHandle>
}

impl ProgressBarBuilder {

    pub fn flags(mut self, flags: ProgressBarFlags) -> ProgressBarBuilder {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> ProgressBarBuilder {
        self.ex_flags = flags;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ProgressBarBuilder {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> ProgressBarBuilder {
        self.position = pos;
        self
    }

    pub fn state(mut self, state: ProgressBarState) -> ProgressBarBuilder {
        self.state = state;
        self
    }

    pub fn step(mut self, step: u32) -> ProgressBarBuilder {
        self.step = step;
        self
    }

    pub fn pos(mut self, pos: u32) -> ProgressBarBuilder {
        self.pos = pos;
        self
    }

    pub fn range(mut self, range: Range<u32>) -> ProgressBarBuilder {
        self.range = range;
        self
    }

    pub fn marquee(mut self, enable: bool) -> ProgressBarBuilder {
        self.marquee_enable = enable;
        self
    }

    pub fn marquee_update(mut self, update_interval: u32) -> ProgressBarBuilder {
        self.marquee_update = update_interval;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ProgressBarBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut ProgressBar) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("Progress Bar"))
        }?;

        *out = Default::default();

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .ex_flags(self.ex_flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        out.set_state(self.state);
        out.set_step(self.step);
        out.set_pos(self.pos);
        out.set_range(self.range);
        out.set_marquee(self.marquee_enable, self.marquee_update);

        Ok(())
    }

}
