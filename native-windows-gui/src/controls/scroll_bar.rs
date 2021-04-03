use winapi::um::winuser::{WS_DISABLED, WS_VISIBLE, WS_TABSTOP, WS_CHILD, SBS_HORZ, SBS_VERT};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::{NwgError, RawEventHandler};
use super::{ControlBase, ControlHandle};
use std::{mem, cell::RefCell, ops::Range};

const NOT_BOUND: &'static str = "Scroll bar is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Scroll bar handle is not HWND!";


bitflags! {
    /**
        The scroll bar flags

        * NONE:       No flags. Equivalent to a invisible scroll bar.
        * VISIBLE:    The scroll bar is immediatly visible after creation
        * DISABLED:   The scroll bar cannot be interacted with by the user. It also has a grayed out look.
        * TAB_STOP:   The control can be selected using tab navigation
        * HORIZONTAL: The scroll bar scrolls from top to bottom
        * VERTICAL:   The scroll bar scrolls from left to right
    */
    pub struct ScrollBarFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;
        const HORIZONTAL = SBS_HORZ;
        const VERTICAL = SBS_VERT;
    }
}

/**
A window can display a data object, such as a document or a bitmap, that is larger than the window's client area. When provided with a scroll bar, the user can scroll a data object in the client area to bring into view the portions of the object that extend beyond the borders of the window.

Requires the `scroll-bar` feature. 

**Builder parameters:**
  * `parent`:    **Required.** The scroll bar parent container.
  * `size`:      The scroll bar size.
  * `position`:  The scroll bar position.
  * `focus`:     The control receive focus after being created
  * `flags`:     A combination of the ScrollBarFlags values.
  * `ex_flags`:  A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `range`:     The value range of the scroll bar
  * `pos`:       he current value of the scroll bar


**Control events:**
  * `OnVerticalScroll`: When the value of a scrollbar with the VERTICAL flags is changed
  * `OnHorizontalScroll`: When the value of a scrollbar with the HORIZONTAL flags is changed
  * `MousePress(_)`: Generic mouse press events on the button
  * `OnMouseMove`: Generic mouse event
  * `OnMouseWheel`: Generic mouse wheel event

```rust
use native_windows_gui as nwg;

fn build_scrollbar(button: &mut nwg::ScrollBar, window: &nwg::Window) {
    nwg::ScrollBar::builder()
        .range(Some(0..100))
        .pos(Some(10))
        .parent(window)
        .build(button);
}
```
*/
#[derive(Default)]
pub struct ScrollBar {
    pub handle: ControlHandle,
    handler0: RefCell<Option<RawEventHandler>>,
    handler1: RefCell<Option<RawEventHandler>>,
}

impl ScrollBar {

    pub fn builder<'a>() -> ScrollBarBuilder {
        ScrollBarBuilder {
            size: (25, 100),
            position: (0, 0),
            enabled: true,
            flags: None,
            ex_flags: 0,
            parent: None,
            focus: false,
            range: None,
            pos: None
        }
    }

    /// Retrieves the current logical position of the slider in a scrollbar.
    /// The logical positions are the integer values in the scrollbar's range of minimum to maximum slider positions. 
    pub fn pos(&self) -> usize {
        use winapi::um::winuser::SBM_GETPOS;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, SBM_GETPOS, 0, 0) as usize
    }

    /// Sets the current logical position of the slider in a scrollbar. If the value is out of range he value is rounded up or down to the nearest valid value..
    pub fn set_pos(&self, p: usize) {
        use winapi::um::winuser::SBM_SETPOS;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, SBM_SETPOS, p, 1);
    }

    /// Returns the range of value the scrollbar can have
    pub fn range(&self) -> Range<usize> {
        use winapi::um::winuser::SBM_GETRANGE;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut min: u32 = 0;
        let mut max: u32 = 0;

        wh::send_message(handle, SBM_GETRANGE, 
            &mut min as *mut u32 as _,
            &mut max as *mut u32 as _
        );

        (min as usize)..(max as usize)
    }

    /// Sets the range of value the scrollbar can have
    pub fn set_range(&self, range: Range<usize>) {
        use winapi::um::winuser::SBM_SETRANGE;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, SBM_SETRANGE, 
            range.start as _, 
            range.end as _,
        );
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

    /// Returns the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Sets the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Returns the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Sets the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "SCROLLBAR"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | WS_TABSTOP | SBS_VERT
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        WS_CHILD
    }

    /// Scrollbar are useless on their own. We need to hook them and handle ALL the message ourself. yay windows...
    unsafe fn hook_scrollbar_controls(&self) {
        use crate::bind_raw_event_handler_inner;
        use winapi::um::winuser::{WM_HSCROLL, WM_VSCROLL, SIF_ALL, SB_CTL, SIF_POS, SB_TOP, SB_BOTTOM, SB_PAGEUP, SB_PAGEDOWN,
            SB_LINERIGHT, SB_LINELEFT, SB_PAGELEFT, SB_PAGERIGHT, SB_THUMBTRACK, SB_LINEUP, SB_LINEDOWN, WM_MOUSEWHEEL,
            GET_WHEEL_DELTA_WPARAM, SCROLLINFO, GetScrollInfo, SetScrollInfo};
        use winapi::shared::{minwindef::{TRUE, LOWORD}, windef::HWND};

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        let parent_handle = ControlHandle::Hwnd(wh::get_window_parent(handle));

        let handler1 = bind_raw_event_handler_inner(&parent_handle, handle as _, move |_hwnd, msg, w, l| {
            let mut si: SCROLLINFO = mem::zeroed();
            match msg {
                WM_HSCROLL => {
                    if (l as HWND) != handle { return None; }

                    si.cbSize = mem::size_of::<SCROLLINFO>() as u32;
                    si.fMask  = SIF_ALL;
                    GetScrollInfo(handle, SB_CTL as i32, &mut si);

                    let event = LOWORD(w as u32) as isize;
                    match event {
                        SB_LINELEFT => { si.nPos -= 1; },
                        SB_LINERIGHT => { si.nPos += 1; },
                        SB_PAGELEFT => { si.nPos -= si.nPage as i32; },
                        SB_PAGERIGHT => { si.nPos += si.nPage as i32; },
                        SB_THUMBTRACK => { si.nPos = si.nTrackPos; },
                        _ => {},
                    }

                    si.fMask = SIF_POS;
                    SetScrollInfo(handle, SB_CTL as _, &si, TRUE);
                    //return Some(0);
                },
                WM_VSCROLL => {
                    if (l as HWND) != handle { return None; }

                    si.cbSize = mem::size_of::<SCROLLINFO>() as u32;
                    si.fMask  = SIF_ALL;
                    GetScrollInfo(handle, SB_CTL as i32, &mut si);

                    let event = LOWORD(w as u32) as isize;
                    match event {
                        SB_TOP => {  si.nPos = si.nMin; },
                        SB_BOTTOM => { si.nPos = si.nMax; },
                        SB_LINEUP => { si.nPos -= 1; },
                        SB_LINEDOWN => { si.nPos += 1; },
                        SB_PAGEUP => { si.nPos -= si.nPage as i32; },
                        SB_PAGEDOWN => { si.nPos += si.nPage as i32; },
                        SB_THUMBTRACK => { si.nPos = si.nTrackPos; },
                        _ => {},
                    }

                    si.fMask = SIF_POS;
                    SetScrollInfo(handle, SB_CTL as _, &si, TRUE);
                    //return Some(0);
                },
                _ => {}
            }

            None
        });

        let handler2 = bind_raw_event_handler_inner(&self.handle, 0, move  |_hwnd, msg, w, _l| {
            match msg {
                WM_MOUSEWHEEL => {
                    let mut si: SCROLLINFO = mem::zeroed();

                    si.cbSize = mem::size_of::<SCROLLINFO>() as u32;
                    si.fMask  = SIF_ALL;
                    GetScrollInfo(handle, SB_CTL as i32, &mut si);

                    let delta = GET_WHEEL_DELTA_WPARAM(w);
                    match delta > 0 {
                        true => { si.nPos -= 1;  },
                        false => { si.nPos += 1; }
                    }

                    si.fMask = SIF_POS;
                    SetScrollInfo(handle, SB_CTL as _, &si, TRUE);
                    return Some(0)
                },
                _ => {}
            }

            None
        });

        *self.handler0.borrow_mut() = Some(handler1.unwrap());
        *self.handler1.borrow_mut() = Some(handler2.unwrap());
    }

}

impl Drop for ScrollBar {
    fn drop(&mut self) {
        use crate::unbind_raw_event_handler;
        
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            drop(unbind_raw_event_handler(h));
        }

        let handler = self.handler1.borrow();
        if let Some(h) = handler.as_ref() {
            drop(unbind_raw_event_handler(h));
        }
    
        self.handle.destroy();
    }
}

pub struct ScrollBarBuilder {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    flags: Option<ScrollBarFlags>,
    ex_flags: u32,
    parent: Option<ControlHandle>,
    focus: bool,
    range: Option<Range<usize>>,
    pos: Option<usize>,
}

impl ScrollBarBuilder {
    
    pub fn flags(mut self, flags: ScrollBarFlags) -> ScrollBarBuilder {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> ScrollBarBuilder {
        self.ex_flags = flags;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ScrollBarBuilder {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> ScrollBarBuilder {
        self.position = pos;
        self
    }

    pub fn enabled(mut self, e: bool) -> ScrollBarBuilder {
        self.enabled = e;
        self
    }

    pub fn focus(mut self, focus: bool) -> ScrollBarBuilder {
        self.focus = focus;
        self
    }

    pub fn range(mut self, range: Option<Range<usize>>) -> ScrollBarBuilder {
        self.range = range;
        self
    }

    pub fn pos(mut self, pos: Option<usize>) -> ScrollBarBuilder {
        self.pos = pos;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ScrollBarBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut ScrollBar) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("ScrollBar"))
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

        out.set_enabled(self.enabled);

        if let Some(range) = self.range {
            out.set_range(range);
        }

        if let Some(pos) = self.pos {
            out.set_pos(pos);
        }

        if self.focus {
            out.set_focus();
        }

        unsafe {
            out.hook_scrollbar_controls();
        }

        Ok(())
    }


}

impl PartialEq for ScrollBar {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
