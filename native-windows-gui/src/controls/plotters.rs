use winapi::um::winuser::{WS_CHILD, WS_VISIBLE, WS_CLIPCHILDREN, WS_CLIPSIBLINGS};
use crate::win32::base_helper::check_hwnd;
use crate::win32::window_helper as wh;
use crate::NwgError;
use super::{ControlBase, ControlHandle};

use plotters::prelude::DrawingArea;
use plotters::coord::Shift;
pub use crate::win32::plotters_d2d::{PlottersError, PlottersBackend};
use std::ops::Deref;

const NOT_BOUND: &'static str = "Plotters control is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Plotters control handle is not HWND!";


/**
    An object that can be used as a drawing area by the plotters library.

    This is needed because direct 2D needs to wrap the drawing command between a `begin_draw` and `end_draw` call
    but it is impossible to do that within the DrawingBackend trait. 
*/
pub struct PlottersDrawingArea<'a> {
    inner: &'a Plotters,
    area: DrawingArea<&'a PlottersBackend, Shift>
}

impl<'a> PlottersDrawingArea<'a> {

    pub fn new(inner: &'a Plotters) -> Result<PlottersDrawingArea<'a>, PlottersError> {
        let backend = inner.d2d_backend.as_ref().unwrap();
        
        backend.rebuild(inner.handle.hwnd().unwrap())?;
        backend.begin_draw();
        backend.clear();

        let area = PlottersDrawingArea {
            inner: inner,
            area: backend.into(),
        };

        Ok(area)
    }

}

impl<'a> Deref for PlottersDrawingArea<'a> {
    type Target = DrawingArea<&'a PlottersBackend, Shift>;

    fn deref(&self) -> &Self::Target {
        &self.area
    }
}

impl<'a> Drop for PlottersDrawingArea<'a> {
    fn drop(&mut self) {
        self.inner.d2d_backend.as_ref()
            .unwrap()
            .end_draw();
    }
}

/**
    A canvas-like control that act as a backend for the [plotters](https://docs.rs/plotters/0.3.0/plotters/) library.
    The plotters control use direct2D to render to the canvas.
*/
#[derive(Default)]
pub struct Plotters {
    pub handle: ControlHandle,
    d2d_backend: Option<PlottersBackend>,
}

impl Plotters {

    pub fn builder() -> PlottersBuilder {
        PlottersBuilder {
            size: (500, 500),
            position: (0, 0),
            ex_flags: 0,
            parent: None,
        }
    }

    /// Prepare the plotters canvas for drawing. Returns an object that can be DrawingArea.
    /// This method may fail if an internal error occured during the last draw call
    pub fn draw<'a>(&'a self) -> Result<PlottersDrawingArea<'a>, PlottersError> {
        check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        PlottersDrawingArea::new(self)
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
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

    /// Return the physical size of canvas in pixels considering the dpi scale
    pub fn physical_size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_physical_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, true) }
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
        "NWG_EXTERN_CANVAS"
    }

    // Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_CHILD | WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        WS_CLIPCHILDREN | WS_CLIPSIBLINGS 
    }


}

impl PartialEq for Plotters {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

pub struct PlottersBuilder {
    parent: Option<ControlHandle>,
    size: (i32, i32),
    position: (i32, i32),
    ex_flags: u32,
}

impl PlottersBuilder {

    pub fn ex_flags(mut self, flags: u32) -> PlottersBuilder {
        self.ex_flags = flags;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> PlottersBuilder {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> PlottersBuilder {
        self.position = pos;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> PlottersBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut Plotters) -> Result<(), NwgError> {
        *out = Default::default();
        
        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(out.flags())
            .ex_flags(self.ex_flags)
            .size(self.size)
            .position(self.position)
            .text("")
            .parent(self.parent)
            .build()?;

        let handle = out.handle.hwnd().unwrap();
        match PlottersBackend::init(handle) {
            Ok(b) => {
                out.d2d_backend = Some(b);
                Ok(())
            },
            Err(e) => {
                *out = Default::default();
                Err(NwgError::from(e))
            }
        }
    }

}
