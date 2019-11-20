use crate::controls::{ControlHandle};
use crate::win32::window::bind_raw_event_handler;
use crate::win32::window_helper as wh;
use winapi::shared::windef::{HWND};
use std::ptr;


/// A control item in a GridLayout
#[derive(Debug)]
pub struct GridLayoutItem {
    /// The handle to the control in the item
    control: HWND,

    /// The column position of the control in the layout
    pub col: u32,

    /// The row position of the control in the layout
    pub row: u32,

    /// The number column this item should span. Should be 1 for single column item.
    pub col_span: u32,

    /// The number row this item should span. Should be 1 for single row item.
    pub row_span: u32
}

impl GridLayoutItem {

    /// Initialize a new grid layout item
    pub fn new<W: Into<ControlHandle>>(c: W, col: u32, row: u32, col_span: u32, row_span: u32) -> GridLayoutItem {
        let control = c.into().hwnd().expect("Child must be HWND");

        GridLayoutItem {
            control,
            col,
            row,
            col_span,
            row_span
        }
    }

}


/// A layout that lays out widgets in a grid
pub struct GridLayout {
    /// The control that holds the layout
    base: HWND,

    /// The children of the control that fit in the layout
    children: Vec<GridLayoutItem>,

    /// The top, right, bottom, left space around the layout
    margins: [u32; 4],

    /// The minimum size of the layout. Used if `base` is smaller than `min_size`.
    min_size: [u32; 2],

    /// The maximum size of the layout. Used if `base` is bigger than `min_size`.
    max_size: [u32; 2],

    /// The number of column. If None, compute the value from children.
    column_count: Option<u32>,

    /// The number of row. If None, compute the value from children.
    row_count: Option<u32>, 

    /// The spacing between controls
    spacing: u32
}

impl GridLayout {

    pub fn builder() -> GridLayoutBuilder {
        let layout = GridLayout {
            base: ptr::null_mut(),
            children: Vec::new(),
            margins: [5, 5, 5, 5],
            min_size: [0, 0],
            max_size: [u32::max_value(), u32::max_value()],
            column_count: None,
            row_count: None,
            spacing: 5,
        };

        GridLayoutBuilder { layout }
    }

    fn update_layout(&self, mut width: u32, mut height: u32) -> () {
        let [m_top, m_right, m_bottom, m_left] = self.margins;
        let sp = self.spacing;

        let children = &self.children;

        let [min_w, min_h] = self.min_size;
        if width < min_w { width = min_w; }
        if height < min_h { height = min_h; }

        let [max_w, max_h] = self.max_size;
        if width > max_w { width = max_w; }
        if height > max_h { height = max_h; }

        let column_count = match self.column_count {
            Some(c) => c,
            None => children.iter().map(|item| item.col + item.col_span).max().unwrap_or(1)
        };

        let row_count = match self.row_count {
            Some(c) => c,
            None => children.iter().map(|item| item.row + item.row_span).max().unwrap_or(1)
        };

        if width < (m_right + m_left) + ((sp * 2) * column_count) {
            return;
        }

        if height < (m_top + m_bottom) + ((sp * 2) * row_count) {
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
        let sp2 = sp * 2;

        for item in self.children.iter() {
            let x = m_left + (sp + (sp2 * item.col)) + (item_width * item.col);
            let y = m_top + (sp + (sp2 * item.row)) + (item_height * item.row);

            let local_width = (item_width * item.col_span) + (sp2 * (item.col_span - 1));
            let local_height = (item_height * item.row_span) + (sp2 * (item.row_span - 1));

            unsafe {
                wh::set_window_position(item.control, x as i32, y as i32);
                wh::set_window_size(item.control, local_width, local_height, false);
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

    /// Add a children to the layout at the position `col` and `row`.
    /// This is a shortcut over `child_item` for item with default span.
    /// The handle must be a window object otherwise the function will panic
    pub fn child<W: Into<ControlHandle>>(mut self, col: u32, row: u32, c: W) -> GridLayoutBuilder {
        let h = c.into().hwnd().expect("Child must be HWND");
        self.layout.children.push(GridLayoutItem {
            control: h,
            col,
            row,
            col_span: 1,
            row_span: 1,
        });

        self
    }

    /// Add a children to the layout
    /// The handle must be a window object otherwise the function will panic
    pub fn child_item(mut self, item: GridLayoutItem) -> GridLayoutBuilder {
        self.layout.children.push(item);
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

    /// Sets the minimum size of the layout
    pub fn min_size(mut self, sz: [u32; 2]) -> GridLayoutBuilder {
        self.layout.min_size = sz;
        self
    }

    /// Sets the maximum size of the layout
    pub fn max_size(mut self, sz: [u32; 2]) -> GridLayoutBuilder {
        self.layout.max_size = sz;
        self
    }

    /// Set the number of column in the layout
    pub fn max_column(mut self, count: Option<u32>) -> GridLayoutBuilder {
        self.layout.column_count = count;
        self
    }

    /// Set the number of row in the layout
    pub fn max_row(mut self, count: Option<u32>) -> GridLayoutBuilder {
        self.layout.row_count = count;
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
            None
        };

        bind_raw_event_handler(&base_handle, 0, cb);
    }

}
