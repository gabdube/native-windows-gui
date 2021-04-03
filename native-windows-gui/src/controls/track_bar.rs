use winapi::shared::{
    windef::HBRUSH,
    minwindef::{WPARAM, LPARAM}
};
use winapi::um::{
    winuser::{WS_VISIBLE, WS_TABSTOP},
    wingdi::DeleteObject,
};
use winapi::um::commctrl::{TBS_AUTOTICKS, TBS_VERT, TBS_HORZ, TBS_TOP, TBS_BOTTOM, TBS_LEFT, TBS_RIGHT, TBS_NOTICKS, TBS_ENABLESELRANGE};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::{NwgError, RawEventHandler};
use super::{ControlBase, ControlHandle};
use std::cell::RefCell;
use std::ops::Range;

const NOT_BOUND: &'static str = "TrackBar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TrackBar handle is not HWND!";



bitflags! {
    /**
        The track bar  flags
    */
    pub struct TrackBarFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const AUTO_TICK = TBS_AUTOTICKS;
        const VERTICAL = TBS_VERT;
        const HORIZONTAL = TBS_HORZ;
        const TICK_TOP = TBS_TOP;
        const TICK_BOTTOM = TBS_BOTTOM;
        const TICK_LEFT = TBS_LEFT;
        const TICK_RIGHT = TBS_RIGHT;
        const NO_TICK = TBS_NOTICKS;
        const RANGE = TBS_ENABLESELRANGE;
        const TAB_STOP = WS_TABSTOP;
    }
}

/**
A trackbar is a window that contains a slider (sometimes called a thumb) in a channel, and optional tick marks.
When the user moves the slider, using either the mouse or the direction keys, the trackbar sends notification messages to indicate the change.

Requires the `trackbar` feature.

**Builder parameters:**
  * `parent`:           **Required.** The trackbar parent container.
  * `size`:             The trackbar size.
  * `position`:         The trackbar position.
  * `focus`:            The control receive focus after being created
  * `flags`:            A combination of the TrackBarFlags values.
  * `ex_flags`:         A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `range`:            The value range of the trackbar
  * `selected_range`:   The selected value range of the trackbar. Used with `TrackBarFlags::RANGE`
  * `pos`:              The current value of the trackbar
  * `background_color`: The background color the of the trackbar


**Control events:**
  * `OnVerticalScroll`: When the value of a trackbar with the VERTICAL flags is changed
  * `OnHorizontalScroll`: When the value of a trackbar with the HORIZONTAL flags is changed
  * `MousePress(_)`: Generic mouse press events on the button
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event

```rust
use native_windows_gui as nwg;
fn build_trackbar(track: &mut nwg::TrackBar, window: &nwg::Window) {
    nwg::TrackBar::builder()
        .range(Some(0..100))
        .pos(Some(10))
        .parent(window)
        .build(track);
}
```

*/
#[derive(Default)]
pub struct TrackBar {
    pub handle: ControlHandle,
    background_brush: Option<HBRUSH>,
    handler0: RefCell<Option<RawEventHandler>>,
}

impl TrackBar {

    pub fn builder() -> TrackBarBuilder {
        TrackBarBuilder {
            size: (100, 20),
            position: (0, 0),
            focus: false,
            range: None,
            selected_range: None,
            pos: None,
            flags: None,
            ex_flags: 0,
            parent: None,
            background_color: None
        }
    }

    /// Retrieves the current logical position of the slider in a trackbar.
    /// The logical positions are the integer values in the trackbar's range of minimum to maximum slider positions. 
    pub fn pos(&self) -> usize {
        use winapi::um::commctrl::TBM_GETPOS;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TBM_GETPOS, 0, 0) as usize
    }

    /// Sets the current logical position of the slider in a trackbar.
    pub fn set_pos(&self, p: usize) {
        use winapi::um::commctrl::TBM_SETPOSNOTIFY;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TBM_SETPOSNOTIFY, 1, p as LPARAM);
    }

    /// Retrieves the starting and ending position of the current selection range in a trackbar.
    /// Only work for trackbar with the `Range` flags
    pub fn selection_range_pos(&self) -> Range<usize> {
        use winapi::um::commctrl::{TBM_GETSELEND, TBM_GETSELSTART};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let end = wh::send_message(handle, TBM_GETSELEND, 0, 0) as usize;
        let start = wh::send_message(handle, TBM_GETSELSTART, 0, 0) as usize;

        start..end
    }

    /// Sets the range value of the trackbar
    /// Only work for trackbar with the `Range` flags
    pub fn set_selection_range_pos(&self, value: Range<usize>) {
        use winapi::um::commctrl::{TBM_SETSELEND, TBM_SETSELSTART};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        wh::send_message(handle, TBM_SETSELEND, 0, value.end as LPARAM);
        wh::send_message(handle, TBM_SETSELSTART, 1, value.start as LPARAM);
    }

    /// Retrieves the minimum position for the slider in a trackbar. 
    pub fn range_min(&self) -> usize {
        use winapi::um::commctrl::TBM_GETRANGEMIN;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TBM_GETRANGEMIN, 0, 0) as usize
    }

    /// Sets the minium logical position for the slider in a trackbar.
    pub fn set_range_min(&self, min: usize) {
        use winapi::um::commctrl::TBM_SETRANGEMIN;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TBM_SETRANGEMIN, 1, min as LPARAM);
    }

    /// Retrieves the maximum position for the slider in a trackbar. 
    pub fn range_max(&self) -> usize {
        use winapi::um::commctrl::TBM_GETRANGEMAX;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TBM_GETRANGEMAX, 0, 0) as usize
    }

    /// Sets the maximum logical position for the slider in a trackbar.
    pub fn set_range_max(&self, max: usize) {
        use winapi::um::commctrl::TBM_SETRANGEMAX;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TBM_SETRANGEMAX, 1, max as LPARAM);
    }

    /// Retrieves the number of tick marks in a trackbar
    pub fn tics_len(&self) -> usize {
        use winapi::um::commctrl::TBM_GETNUMTICS;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TBM_GETNUMTICS, 0, 0) as usize
    }

    /// Retrieves the logical position of a tick mark in a trackbar.
    /// The logical position can be any of the integer values in the trackbar's range of minimum to maximum slider positions. 
    pub fn tic_value(&self, index: usize) -> usize {
        use winapi::um::commctrl::TBM_GETTIC;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TBM_GETTIC, index as WPARAM, 0) as usize
    }

    //
    // Basic methods
    //

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

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the track bar
    pub fn set_focus(&self) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        winapi::um::commctrl::TRACKBAR_CLASS
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | TBS_AUTOTICKS | WS_TABSTOP
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD};

        WS_CHILD
    }

    /// Change the label background color to transparent.
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

impl Drop for TrackBar {
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


pub struct TrackBarBuilder {
    size: (i32, i32),
    position: (i32, i32),
    focus: bool,
    range: Option<Range<usize>>,
    selected_range: Option<Range<usize>>,
    pos: Option<usize>,
    flags: Option<TrackBarFlags>,
    ex_flags: u32,
    parent: Option<ControlHandle>,
    background_color: Option<[u8; 3]>,
}

impl TrackBarBuilder {

    pub fn flags(mut self, flags: TrackBarFlags) -> TrackBarBuilder {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> TrackBarBuilder {
        self.ex_flags = flags;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> TrackBarBuilder {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> TrackBarBuilder {
        self.position = pos;
        self
    }

    pub fn focus(mut self, focus: bool) -> TrackBarBuilder {
        self.focus = focus;
        self
    }

    pub fn range(mut self, range: Option<Range<usize>>) -> TrackBarBuilder {
        self.range = range;
        self
    }

    pub fn selected_range(mut self, range: Option<Range<usize>>) -> TrackBarBuilder {
        self.selected_range = range;
        self
    }

    pub fn pos(mut self, pos: Option<usize>) -> TrackBarBuilder {
        self.pos = pos;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TrackBarBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> TrackBarBuilder {
        self.background_color = color;
        self
    }

    pub fn build(self, out: &mut TrackBar) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("TrackBar"))
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


        if self.background_color.is_some() {
            out.hook_background_color(self.background_color.unwrap());
        }
    
        if self.focus {
            out.set_focus();
        }

        if let Some(range) = self.range {
            out.set_range_min(range.start);
            out.set_range_max(range.end);
        }

        if let Some(range) = self.selected_range {
            out.set_selection_range_pos(range);
        }

        if let Some(pos) = self.pos {
            out.set_pos(pos);
        }
        

        Ok(())
    }

}

impl PartialEq for TrackBar {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
