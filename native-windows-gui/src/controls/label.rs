use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use crate::win32::window_helper as wh;
use crate::{Font, NwgError, HTextAlign, VTextAlign, RawEventHandler, unbind_raw_event_handler};
use super::{ControlBase, ControlHandle};
use std::cell::RefCell;

const NOT_BOUND: &'static str = "Label is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Label handle is not HWND!";

bitflags! {
    /**
        The label flags

        * NONE:     No flags. Equivalent to a invisible blank label.
        * VISIBLE:  The label is immediatly visible after creation
        * DISABLED: The label cannot be interacted with by the user. It also has a grayed out look.
    */
    pub struct LabelFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/**
A label is a single line of static text
*/
#[derive(Default)]
pub struct Label {
    pub handle: ControlHandle,
    handler0: RefCell<Option<RawEventHandler>>,
    handler1: RefCell<Option<RawEventHandler>>,
}

impl Label {

    pub fn builder<'a>() -> LabelBuilder<'a> {
        LabelBuilder {
            text: "A label",
            size: (130, 25),
            position: (0, 0),
            flags: None,
            font: None,
            parent: None,
            h_align: HTextAlign::Left,
            v_align: VTextAlign::Top,
            background_color: None
        }
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

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button.
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
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

    /// Return true if the control is visible to the user. Will return true even if the 
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

    /// Return the size of the label in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the label in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the label in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the label in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the label text
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the label text
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "STATIC"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        use winapi::um::winuser::{SS_NOPREFIX, SS_LEFT, SS_WORDELLIPSIS};

        WS_VISIBLE | SS_NOPREFIX | SS_LEFT | SS_WORDELLIPSIS
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{SS_NOTIFY, WS_CHILD};

        WS_CHILD | SS_NOTIFY
    }

    /// Center the text vertically.
    fn hook_non_client_size(&self, bg: Option<[u8; 3]>, v_align: VTextAlign) {
        use crate::bind_raw_event_handler;
        use winapi::shared::windef::{HWND, HGDIOBJ, RECT, HBRUSH, POINT};
        use winapi::shared::{basetsd::UINT_PTR, minwindef::LRESULT};
        use winapi::um::winuser::{WM_CTLCOLORSTATIC, WM_NCCALCSIZE, WM_NCPAINT, WM_SIZE, DT_CALCRECT, DT_LEFT, NCCALCSIZE_PARAMS, COLOR_WINDOW};
        use winapi::um::winuser::{SWP_NOOWNERZORDER, SWP_NOSIZE, SWP_NOMOVE, SWP_FRAMECHANGED};
        use winapi::um::winuser::{GetDC, DrawTextW, ReleaseDC, GetClientRect, GetWindowRect, FillRect, ScreenToClient, SetWindowPos};
        use winapi::um::wingdi::{SelectObject, CreateSolidBrush, RGB};
        use std::{mem, ptr};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let parent_handle = ControlHandle::Hwnd(wh::get_window_parent(handle));

        let brush = match bg {
            Some(c) => unsafe { CreateSolidBrush(RGB(c[0], c[1], c[2])) },
            None => COLOR_WINDOW as HBRUSH
        };

        match v_align {
            VTextAlign::Top => {},
            VTextAlign::Center => {},
            VTextAlign::Bottom => {},
        };

        unsafe {

        if bg.is_some() {
            let handler0 = bind_raw_event_handler(&parent_handle, handle as UINT_PTR, move |_hwnd, msg, _w, l| {
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

            *self.handler0.borrow_mut() = Some(handler0);
        }

        let handler1 = bind_raw_event_handler(&self.handle, 0, move |hwnd, msg, w, l| {
            match msg {
                WM_NCCALCSIZE  => {
                    if w == 0 { return None }

                    // Calculate client area height needed for a font
                    let font_handle = wh::get_window_font(hwnd);
                    let mut r: RECT = mem::zeroed();
                    let dc = GetDC(hwnd);
                    
                    let old = SelectObject(dc, font_handle as HGDIOBJ);
                    let calc: [u16;2] = [75, 121];
                    DrawTextW(dc, calc.as_ptr(), 2, &mut r, DT_CALCRECT | DT_LEFT);

                    let client_height = r.bottom;

                    SelectObject(dc, old);
                    ReleaseDC(hwnd, dc);

                    // Calculate NC area to center text.
                    let mut client: RECT = mem::zeroed();
                    let mut window: RECT = mem::zeroed();
                    GetClientRect(hwnd, &mut client);
                    GetWindowRect(hwnd, &mut window);

                    let window_height = window.bottom - window.top;
                    let center = ((window_height - client_height) / 2) - 1;
                    
                    // Save the info
                    let info_ptr: *mut NCCALCSIZE_PARAMS = l as *mut NCCALCSIZE_PARAMS;
                    let info = &mut *info_ptr;

                    info.rgrc[0].top += center;
                    info.rgrc[0].bottom -= center;
                },
                WM_NCPAINT  => {
                    let mut window: RECT = mem::zeroed();
                    let mut client: RECT = mem::zeroed();
                    GetWindowRect(hwnd, &mut window);
                    GetClientRect(hwnd, &mut client);

                    let mut pt1 = POINT {x: window.left, y: window.top};
                    ScreenToClient(hwnd, &mut pt1);

                    let mut pt2 = POINT {x: window.right, y: window.bottom};
                    ScreenToClient(hwnd, &mut pt2);

                    let top = RECT {
                        left: 0,
                        top: pt1.y,
                        right: client.right,
                        bottom: client.top
                    };

                    let bottom = RECT {
                        left: 0,
                        top: client.bottom,
                        right: client.right,
                        bottom: pt2.y
                    };

                    let dc = GetDC(hwnd);
                    FillRect(dc, &top, brush);
                    FillRect(dc, &bottom, brush);
                    ReleaseDC(hwnd, dc);
                },
                WM_SIZE => {
                    SetWindowPos(hwnd, ptr::null_mut(), 0, 0, 0, 0, SWP_NOOWNERZORDER | SWP_NOSIZE | SWP_NOMOVE | SWP_FRAMECHANGED);
                },
                _ => {}
            }

            None
        });

        *self.handler1.borrow_mut() = Some(handler1);

        }
    }

}


impl Drop for Label {
    fn drop(&mut self) {
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            unbind_raw_event_handler(h);
        }

        let handler = self.handler1.borrow();
        if let Some(h) = handler.as_ref() {
            unbind_raw_event_handler(h);
        }
    }
}

pub struct LabelBuilder<'a> {
    text: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    background_color: Option<[u8; 3]>,
    flags: Option<LabelFlags>,
    font: Option<&'a Font>,
    h_align: HTextAlign,
    v_align: VTextAlign,
    parent: Option<ControlHandle>
}

impl<'a> LabelBuilder<'a> {

    pub fn flags(mut self, flags: LabelFlags) -> LabelBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn text(mut self, text: &'a str) -> LabelBuilder<'a> {
        self.text = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> LabelBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> LabelBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> LabelBuilder<'a> {
        self.font = font;
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> LabelBuilder<'a> {
        self.background_color = color;
        self
    }

    pub fn h_align(mut self, align: HTextAlign) -> LabelBuilder<'a> {
        self.h_align = align;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> LabelBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut Label) -> Result<(), NwgError> {
        use winapi::um::winuser::{SS_LEFT, SS_RIGHT, SS_CENTER};

        let mut flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        match self.h_align {
            HTextAlign::Left => { flags |= SS_LEFT; },
            HTextAlign::Right => { flags |= SS_RIGHT; },
            HTextAlign::Center => { flags |= SS_CENTER; },
        }

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("Label"))
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
        }

        out.hook_non_client_size(self.background_color, self.v_align);

        Ok(())
    }

}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
