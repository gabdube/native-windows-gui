use crate::controls::{ControlHandle};
use crate::win32::window::bind_raw_event_handler;
use crate::win32::window_helper as wh;
use winapi::shared::windef::{HWND};
use std::ptr;


/// A layout that lays out widgets in a grid
pub struct GridLayout {
    base: HWND,
    children: Vec<([u32;2], HWND)>,
    margins: [u32; 4],
    spacing: u32
}

impl GridLayout {

    pub fn builder() -> GridLayoutBuilder {
        let layout = GridLayout {
            base: ptr::null_mut(),
            children: Vec::new(),
            margins: [5, 5, 5, 5],
            spacing: 5,
        };

        GridLayoutBuilder { layout }
    }

    fn update_layout(&self, mut width: u32, mut height: u32) -> () {
        let [m_top, m_right, m_bottom, m_left] = self.margins;
        let sp = self.spacing;

        let children = &self.children;
        let column_count = children.iter().map(|([x, _y], _)| *x).max().unwrap_or(0) + 1;
        let row_count = children.iter().map(|([_x, y], _)| *y).max().unwrap_or(0) + 1;

        if width < (m_right + m_left) + ((sp * 2) * column_count) {
            return;
        }

        if height < m_top + m_bottom + ((sp * 2) * row_count) {
            return;
        }

        // Apply margins
        width = width - m_right - m_left;
        height = height - m_top - m_bottom;

        // Apply spacing
        width = width - ((sp * 2) * column_count);
        height = height - ((sp * 2) * row_count);

        let item_width = width / column_count;
        let item_height = height / row_count;

        for &([x, y], handle) in self.children.iter() {
            let x = m_left + (sp + (sp * 2 * x)) + (item_width * x);
            let y = m_top + (sp + (sp * 2 * y)) + (item_height * y);

            unsafe {
                wh::set_window_position(handle, x as i32, y as i32);
                wh::set_window_size(handle, item_width, item_height, false);
            }
        }
    }
}


/// Builder for a `GridLayout` struct
pub struct GridLayoutBuilder {
    layout: GridLayout
}

impl GridLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> GridLayoutBuilder {
        self.layout.base = p.into().hwnd().expect("Parent must be HWND");
        self
    }

    /// Add a children to the layout at the position `p`. If an item is already at the selected position, the old child will be replaced.
    /// The handle must be a window object otherwise the function will panic
    pub fn child(mut self, p1: u32, p2: u32, c: &ControlHandle) -> GridLayoutBuilder {
        let h = c.hwnd().expect("Child must be HWND");
        let pos = [p1, p2];
        let children = &mut self.layout.children;

        if let Some(i) = children.iter().position(|(p2, _)| pos == *p2) {
            children[i] = (pos, h);
        } else {
            children.push((pos, h))
        }

        self
    }

    /// Set the margins of the layout. The four values are in this order: top, right, bottom, left.
    pub fn margin(mut self, m: [u32; 4]) -> GridLayoutBuilder {
        self.layout.margins = m;
        self
    }

    /// Set the size of the space between the children in the layout. Default value is 5.
    pub fn spacing(mut self, sp: u32) -> GridLayoutBuilder {
        self.layout.spacing = sp;
        self
    }

    /// Build the layout object and bind the callback.
    /// Children must only contains window object otherwise this method will panic.
    pub fn build(self) {
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::{HIWORD, LOWORD};
        let layout = self.layout;

        unsafe {
            let (w, h) = wh::get_window_size(layout.base);
            layout.update_layout(w, h);
        }

        let base_handle = ControlHandle::Hwnd(layout.base);
        let cb = move |_h, msg, _w, l| {
            if msg == WM_SIZE {
                let size = l as u32;
                let width = LOWORD(size) as u32;
                let height = HIWORD(size) as u32;
                GridLayout::update_layout(&layout, width, height);
            }
        };

        bind_raw_event_handler(&base_handle, cb);
    }

}
