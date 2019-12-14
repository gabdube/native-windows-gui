use crate::controls::{ControlHandle};
use crate::win32::window::bind_raw_event_handler;
use crate::win32::window_helper as wh;
use winapi::shared::windef::{HWND};
use std::ptr;


/// A layout that lines up control horizontally
/// NWG layouts use interior mutability to manage their controls.
#[derive(Debug)]
pub struct HBoxLayout {
    base: HWND,
    children: Vec<(u32, HWND)>,
    margins: [u32; 4],
    spacing: u32
}

impl HBoxLayout {

    pub fn builder() -> BoxLayoutBuilder {
        let layout = HBoxLayout {
            base: ptr::null_mut(),
            children: Vec::new(),
            margins: [10, 5, 10, 5],
            spacing: 5,
        };

        BoxLayoutBuilder { layout: LayoutType::Horizontal(layout) }
    }

    /// Private function that update the layout and the children
    fn update_layout(&self, mut width: u32, mut height: u32) -> () {
        let [m_top, m_right, m_bottom, m_left] = self.margins;
        let sp = self.spacing;

        let column_count = self.children.iter().map(|(i, _)| *i).max().unwrap_or(0) + 1;

        if width < (m_right + m_left) + ((sp * 2) * column_count) {
            return;
        }

        if height < m_top + m_bottom + (sp * 2) {
            return;
        }

        // Apply margins
        width = width - m_right - m_left;
        height = height - m_top - m_bottom;

        // Apply spacing
        width = width - ((sp * 2) * column_count);
        height = height - (sp * 2);

        let item_width = width / column_count;
        let item_height = height;

        let y = (m_top + sp) as i32;

        for &(i, handle) in self.children.iter() {
            let x = m_left + (sp + (sp * 2 * i)) + (item_width * i);

            unsafe {
                wh::set_window_position(handle, x as i32, y);
                wh::set_window_size(handle, item_width, item_height, false);
            }
        }

    } 

}

/// A layout that lines up control vertically
#[derive(Debug)]
pub struct VBoxLayout {
    base: HWND,
    children: Vec<(u32, HWND)>,
    margins: [u32; 4],
    spacing: u32
}


impl VBoxLayout {

    pub fn builder() -> BoxLayoutBuilder {
        let layout = VBoxLayout {
            base: ptr::null_mut(),
            children: Vec::new(),
            margins: [5, 10, 5, 10],
            spacing: 5,
        };

        BoxLayoutBuilder { layout: LayoutType::Vertical(layout) }
    }

    /// Private function that update the layout and the children
    fn update_layout(&self, mut width: u32, mut height: u32) -> () {
        let [m_top, m_right, m_bottom, m_left] = self.margins;
        let sp = self.spacing;

        let row_count = self.children.iter().map(|(i, _)| *i).max().unwrap_or(0) + 1;

        if width < m_right + m_left + (sp * 2) {
            return;
        }

        if height < m_top + m_bottom + ((sp * 2) * row_count) {
            return;
        }

        // Apply margins
        width = width - m_right - m_left;
        height = height - m_top - m_bottom;

        // Apply spacing
        width = width - (sp * 2);
        height = height - ((sp * 2) * row_count);

        let item_width = width;
        let item_height = height / row_count;

        let x = (m_left + sp) as i32;

        for &(i, handle) in self.children.iter() {
            let y = m_top + (sp + (sp * 2 * i)) + (item_height * i);

            unsafe {
                wh::set_window_position(handle, x, y as i32);
                wh::set_window_size(handle, item_width, item_height, false);
            }
        }

    } 

}


enum LayoutType {
    Vertical(VBoxLayout),
    Horizontal(HBoxLayout)
}

pub struct BoxLayoutBuilder {
    layout: LayoutType
}

impl BoxLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> BoxLayoutBuilder {
        let parent = p.into();
        match &mut self.layout {
            LayoutType::Vertical(layout) => layout.base = parent.hwnd().expect("Parent must be HWND"),
            LayoutType::Horizontal(layout) => layout.base = parent.hwnd().expect("Parent must be HWND"),
        };
        self
    }

    /// Add a children to the layout at the position `p`. If an item is already at the selected position, the old child will be replaced.
    /// The handle must be a window object otherwise the function will panic
    pub fn child<W: Into<ControlHandle>>(mut self, p: u32, c: W) -> BoxLayoutBuilder {
        let h = c.into().hwnd().expect("Child must be HWND");

        let children = match &mut self.layout {
            LayoutType::Vertical(layout) => &mut layout.children,
            LayoutType::Horizontal(layout) => &mut layout.children
        };

        if let Some(i) = children.iter().position(|(p2, _)| p == *p2) {
            children[i] = (p, h);
        } else {
            children.push((p, h))
        }

        self
    }

    /// Set the margins of the layout. The four values are in this order: top, right, bottom, left.
    pub fn margin(mut self, m: [u32; 4]) -> BoxLayoutBuilder {
        match &mut self.layout {
            LayoutType::Vertical(layout) => layout.margins = m,
            LayoutType::Horizontal(layout) => layout.margins = m,
        };
        self
    }

    /// Set the size of the space between the children in the layout. Default value is 5.
    pub fn spacing(mut self, sp: u32) -> BoxLayoutBuilder {
        match &mut self.layout {
            LayoutType::Vertical(layout) => layout.spacing = sp,
            LayoutType::Horizontal(layout) => layout.spacing = sp,
        };
        self
    }

    /// Build the layout object and bind the callback.
    /// Children must only contains window object otherwise this method will panic.
    pub fn build(self) {
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::{HIWORD, LOWORD};

        match self.layout {
            LayoutType::Vertical(layout) => {
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
                        VBoxLayout::update_layout(&layout, width, height);
                    }
                    None
                };

                bind_raw_event_handler(&base_handle, 0, cb);
            },

            LayoutType::Horizontal(layout) => {
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
                        HBoxLayout::update_layout(&layout, width, height);
                    }
                    None
                };

                bind_raw_event_handler(&base_handle, 0, cb);
            }
        };
    }

}
