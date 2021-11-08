use crate::controls::ControlHandle;
use crate::win32::window::bind_raw_event_handler_inner;
use crate::win32::window_helper as wh;
use crate::NwgError;
use winapi::shared::windef::{HWND};
use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;


/// A control item in a DynLayout
#[derive(Debug)]
pub struct DynLayoutItem {
    /// The handle to the control in the item
    control: HWND,
    pos_init: (i32, i32),
    size_init: (i32, i32),
    mv: (i32, i32),
    sz: (i32, i32)
}

impl DynLayoutItem {

    /// Initialize a new layout item
    pub fn new<W: Into<ControlHandle>>(c: W, mv: (i32, i32), sz: (i32, i32)) -> DynLayoutItem {
        let control = c.into().hwnd().expect("Child must be a window-like control (HWND handle)");
        let pos_init = (0, 0);
        let size_init = (0, 0);

        DynLayoutItem{ control, pos_init, size_init, mv, sz }
    }

}


/// A layout that lays out widgets
/// This is the inner data shared between the callback and the application
pub struct DynLayoutInner {
    /// The control that holds the layout
    base: HWND,

    /// The children of the control that fit in the layout
    children: Vec<DynLayoutItem>,
}

#[derive(Clone)]
pub struct DynLayout {
    inner: Rc<RefCell<DynLayoutInner>>
}

impl DynLayout {

    pub fn builder() -> DynLayoutBuilder {
        let layout = DynLayoutInner {
            base: ptr::null_mut(),
            children: Vec::new(),
        };

        DynLayoutBuilder { layout }
    }

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(&self, p: W) {
        let mut inner = self.inner.borrow_mut();
        inner.base = p.into().hwnd().expect("Parent must be HWND");
    }

    /**
        Add a children control to the layout.
        This is a simplified interface over `add_child_item`

        Panic:
        - If the layout is not initialized
        - If the control is not window-like (HWND handle)
    */
    pub fn add_child<W: Into<ControlHandle>>(&self, m: (i32, i32), s: (i32, i32), c: W) {
        let hwnd = c.into().hwnd().expect("Child must be a window-like control (HWND handle)");
        let pos = unsafe { wh::get_window_position(hwnd) };
        let size = unsafe { wh::get_window_size(hwnd) };

        let (whost, hhost) = unsafe { wh::get_window_size(self.inner.borrow_mut().base) };

        let xdelta = 0.01 * whost as f32;
        let ydelta = 0.01 * hhost as f32;

        let mut xpos = pos.0;
        if m.0 > 0 { xpos -= (xdelta * m.0 as f32) as i32; }

        let mut ypos = pos.1;
        if m.1 > 0 { ypos -= (ydelta * m.1 as f32) as i32; }

        let mut xsize = size.0 as i32;
        if s.0 > 0 { xsize -= (xdelta * s.0 as f32) as i32; }

        let mut ysize = size.1 as i32;
        if s.1 > 0 { ysize -= (ydelta * s.1 as f32) as i32; }

        let item = DynLayoutItem {
            control: hwnd,
            pos_init: (xpos, ypos),
            size_init: (xsize, ysize),
            mv: m,
            sz: s
        };

        self.add_child_item(item);
    }

    /**
    Add a children control to the layout.

    Panic:
        - If the layout is not initialized
        - If the control is not window-like (HWND handle)
    */
    pub fn add_child_item(&self, i: DynLayoutItem) {
        let base = {
            let mut inner = self.inner.borrow_mut();
            if inner.base.is_null() {
                panic!("DynLayout is not initialized");
            }

            // No need to check the layout item control because it's checked in `DynLayoutItem::new`

            inner.children.push(i);
            inner.base
        };

        let (w, h) = unsafe { wh::get_window_size(base) };
        self.update_layout(w as u32, h as u32);
    }

    /**
        Remove the children control in the layout. See also `remove_child_by_pos`.
        Note that the child control won't be hidden after being removed from the control.

        This method won't do anything if there is no control at the specified position.

        Panic:
        - If the layout is not initialized
    */
    pub fn remove_child<W: Into<ControlHandle>>(&self, c: W) {
        let base = {
            let mut inner = self.inner.borrow_mut();
            if inner.base.is_null() {
                panic!("DynLayout is not initialized");
            }

            let handle = c.into().hwnd().expect("Control must be window-like (HWND handle)");
            let index = inner.children.iter().position(|item| item.control == handle);
            match index {
                Some(i) => { inner.children.remove(i); },
                None => { return; }
            }

            inner.base
        };

        let (w, h) = unsafe { wh::get_window_size(base) };
        self.update_layout(w as u32, h as u32);
    }

    /**
        Check if a window control is a children of the layout

        Panic:
        - If the layout is not initialized
        - If the child is not a window-like control
    */
    pub fn has_child<W: Into<ControlHandle>>(&self, c: W) -> bool {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("DynLayout is not initialized");
        }

        let handle = c.into().hwnd().expect("Children is not a window-like control (HWND handle)");
        inner.children.iter().any(|c| c.control == handle )
    }

    /// Resize the layout as if the parent window had the specified size.
    ///
    /// Arguments:
    ///   w: New width of the layout
    ///   h: New height of the layout
    ///
    ///  Panic:
    ///   - The layout must have been successfully built otherwise this function will panic.
    pub fn resize(&self, w: u32, h: u32) {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("Layout is not bound to a parent control.")
        }
        self.update_layout(w, h);
    }

    /// Resize the layout to fit the parent window size
    ///
    /// Panic:
    ///   - The layout must have been successfully built otherwise this function will panic.
    pub fn fit(&self) {
        let inner = self.inner.borrow();
        if inner.base.is_null() {
            panic!("Layout is not bound to a parent control.")
        }

        let (w, h) = unsafe { wh::get_window_size(inner.base) };
        self.update_layout(w, h);
    }

    fn update_layout(&self, width: u32, height: u32) -> () {
        use winapi::um::winuser::{BeginDeferWindowPos, DeferWindowPos, EndDeferWindowPos};
        use winapi::um::winuser::{HWND_TOP, SWP_NOZORDER, SWP_NOREPOSITION, SWP_NOACTIVATE, SWP_NOCOPYBITS};
        use winapi::ctypes::c_int;

        let inner = self.inner.borrow();
        if inner.base.is_null() || inner.children.len() == 0 {
            return;
        }

        let xdelta = 0.01 * width as f32;
        let ydelta = 0.01 * height as f32;

        unsafe {
            let hdwp = BeginDeferWindowPos(inner.children.len() as c_int);

            let mut last_handle = None;
            for item in inner.children.iter() {
                let mut x = item.pos_init.0;
                if item.mv.0 > 0 { x += (xdelta * item.mv.0 as f32) as i32; }

                let mut y = item.pos_init.1;
                if item.mv.1 > 0 { y += (ydelta * item.mv.1 as f32) as i32; }

                let mut w = item.size_init.0;
                if item.sz.0 > 0 { w += (xdelta * item.sz.0 as f32) as i32; }

                let mut h = item.size_init.1;
                if item.sz.1 > 0 { h += (ydelta * item.sz.1 as f32) as i32; }

                DeferWindowPos(hdwp, item.control, HWND_TOP, x, y, w, h, SWP_NOZORDER | SWP_NOREPOSITION | SWP_NOACTIVATE | SWP_NOCOPYBITS);

                wh::set_window_after(item.control, last_handle);
                last_handle = Some(item.control);
            }

            EndDeferWindowPos(hdwp);
        }
    }
}

impl Default for DynLayout {

    fn default() -> DynLayout {
        let inner = DynLayoutInner {
            base: ptr::null_mut(),
            children: Vec::new(),
        };

        DynLayout {
            inner: Rc::new(RefCell::new(inner))
        }
    }

}


/// Builder for a `DynLayout` struct
pub struct DynLayoutBuilder {
    layout: DynLayoutInner
}

impl DynLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> DynLayoutBuilder {
        self.layout.base = p.into().hwnd().expect("Parent must be HWND");
        self
    }

    /// Add a children to the layout at the position `col` and `row`.
    /// This is a shortcut over `child_item` for item with default span.
    /// The handle must be a window object otherwise the function will panic
    pub fn child<W: Into<ControlHandle>>(mut self, m: (i32, i32), s: (i32, i32), c: W) -> DynLayoutBuilder {
        let hwnd = c.into().hwnd().expect("Child must be HWND");
        let pos = unsafe { wh::get_window_position(hwnd) };
        let size = unsafe { wh::get_window_size(hwnd) };

        self.layout.children.push(DynLayoutItem {
            control: hwnd,
            pos_init: pos,
            size_init: (size.0 as i32, size.1 as i32),
            mv: m,
            sz: s
        });

        self
    }

    /// Add a children to the layout
    /// The handle must be a window object otherwise the function will panic
    pub fn child_item(mut self, item: DynLayoutItem) -> DynLayoutBuilder {
        self.layout.children.push(item);
        self
    }

    /// Build the layout object and bind the callback.
    /// Children must only contains window object otherwise this method will panic.
    pub fn build(self, layout: &DynLayout) -> Result<(), NwgError> {
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::{HIWORD, LOWORD};

        if self.layout.base.is_null() {
            return Err(NwgError::layout_create("DynLayout does not have a parent."));
        }

        let (w, h) = unsafe { wh::get_window_size(self.layout.base) };
        let base_handle = ControlHandle::Hwnd(self.layout.base);

        // Saves the new layout. TODO: should free the old one too (if any)
        {
            let mut layout_inner = layout.inner.borrow_mut();
            *layout_inner = self.layout;
        }

        // Initial layout update
        layout.update_layout(w, h);

        // Bind the event handler
        let event_layout = layout.clone();
        let cb = move |_h, msg, _w, l| {
            if msg == WM_SIZE {
                let size = l as u32;
                let width = LOWORD(size) as i32;
                let height = HIWORD(size) as i32;
                let (w, h) = unsafe { crate::win32::high_dpi::physical_to_logical(width, height) };
                DynLayout::update_layout(&event_layout, w as u32, h as u32);
            }
            None
        };

        /// Keep generating ids so that multiple layouts can be applied to the same parent
        use std::sync::atomic::{AtomicUsize, Ordering};
        static BOX_LAYOUT_ID: AtomicUsize = AtomicUsize::new(0x8FFF);
        bind_raw_event_handler_inner(&base_handle, BOX_LAYOUT_ID.fetch_add(1, Ordering::SeqCst), cb).unwrap();

        Ok(())
    }

}
