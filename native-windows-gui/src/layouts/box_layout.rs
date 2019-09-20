use crate::controls::ControlHandle;
use crate::win32::window::bind_raw_event_handler;
use crate::win32::window_helper as wh;
use winapi::shared::windef::{HWND};
use std::ptr;


/// A layout that lines up control horizontally
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
    fn update_layout(&self, width: u32, height: u32) -> () {
        let m = self.margins;
        let sp = self.spacing;
        let children_count = self.children.len() as u32;

        let dwidth = width / children_count;
        let m_top_bottom = m[0] + m[2];
        let m_left_right = m[1] + m[3];

        if m_top_bottom > height || m_left_right > dwidth {
            return;
        }

        let item_height = height - m_top_bottom;
        let item_width = dwidth - m_left_right;

        let y = m[0] as i32;

        for &(i, handle) in self.children.iter() {
            let x = (m[3] + (sp*(1+i)) + (item_width * i)) as i32;

            unsafe {
                wh::set_window_position(handle, x, y);
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
    fn update_layout(&self, width: u32, height: u32) -> () {
        let m = self.margins;
        let sp = self.spacing;
        let children_count = self.children.len() as u32;

        let dheight = height / children_count;
        let m_top_bottom = m[0] + m[2];
        let m_left_right = m[1] + m[3];

        if m_left_right > width || m_top_bottom > dheight {
            return;
        }

        let item_height = dheight - m_top_bottom;
        let item_width = width - m_left_right;

        let x = m[3] as i32;

        for &(i, handle) in self.children.iter() {
            let y = (m[0] + (sp*(1+i)) + (item_height * i)) as i32;

            unsafe {
                wh::set_window_position(handle, x, y);
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
    pub fn parent(mut self, p: &ControlHandle) -> BoxLayoutBuilder {
        match &mut self.layout {
            LayoutType::Vertical(layout) => layout.base = p.hwnd().expect("Parent must be HWND"),
            LayoutType::Horizontal(layout) => layout.base = p.hwnd().expect("Parent must be HWND"),
        };
        self
    }

    /// Add a children to the layout at the position `p`. If an item is already at the selected position, the old child will be replaced.
    /// The handle must be a window object otherwise the function will panic
    pub fn child(mut self, p: u32, c: &ControlHandle) -> BoxLayoutBuilder {
        let h = c.hwnd().expect("Child must be HWND");

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
                };

                bind_raw_event_handler(&base_handle, cb);
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
                };

                bind_raw_event_handler(&base_handle, cb);
            }
        };
    }

}
