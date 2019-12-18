use crate::controls::{ControlHandle};
use crate::win32::window::bind_raw_event_handler;
use crate::win32::window_helper as wh;
use winapi::shared::windef::{HWND};
use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;


/// The orientation of a box layout
#[derive(Copy, Clone, Debug)]
pub enum BoxLayoutType {
    Vertical,
    Horizontal
}

/// A control item in a BoxLayout
#[derive(Debug)]
pub struct BoxLayoutItem {
    /// The handle to the control in the item
    control: HWND,

    /// The position of the control in the layout
    pub cell: u32,

    /// The number column/row this item should span. Should be 1 for single cell item.
    pub span: u32,
}


/// A layout that lays out widgets in a line
/// This is the inner data shared between the callback and the application
pub struct BoxLayoutInner {
    /// The control that holds the layout
    base: HWND,

    /// The orientation of the layout
    ty: BoxLayoutType,

    /// The children of the control that fit in the layout
    children: Vec<BoxLayoutItem>,

    /// The top, right, bottom, left space around the layout
    margins: [u32; 4],

    /// The minimum size of the layout. Used if `base` is smaller than `min_size`.
    min_size: [u32; 2],

    /// The maximum size of the layout. Used if `base` is bigger than `min_size`.
    max_size: [u32; 2],

    /// The number of column/row. If None, compute the value from children.
    /// The column or row is determined by the layout type.
    cell_count: Option<u32>,

    /// The spacing between controls
    spacing: u32
}


/// A layout that lines up control horizontally or vertically
/// NWG layouts use interior mutability to manage their controls.
#[derive(Clone)]
pub struct BoxLayout {
    inner: Rc<RefCell<BoxLayoutInner>>
}

impl BoxLayout {

    pub fn builder() -> BoxLayoutBuilder {
        let layout = BoxLayoutInner {
            base: ptr::null_mut(),
            ty: BoxLayoutType::Vertical,
            children: Vec::new(),
            margins: [5, 10, 5, 10],
            spacing: 5,
            min_size: [0, 0],
            max_size: [u32::max_value(), u32::max_value()],
            cell_count: None,
        };

        BoxLayoutBuilder { layout: layout }
    }

    /// Private function that update the layout and the children
    fn update_layout(&self, mut width: u32, mut height: u32) -> () {
        let inner = self.inner.borrow();
        if inner.base.is_null() || inner.children.len() == 0 {
            return;
        }
        
        let [m_top, m_right, m_bottom, m_left] = inner.margins;
        let sp = inner.spacing;

        let cell_count = match inner.cell_count {
            Some(c) => c,
            None => inner.children.iter().map(|item| item.cell + item.span).max().unwrap_or(1)
        };

        if width < (m_right + m_left) + ((sp * 2) * cell_count) {
            return;
        }

        if height < m_top + m_bottom + (sp * 2) {
            return;
        }

        // Apply margins
        width = width - m_right - m_left;
        height = height - m_top - m_bottom;

        // Apply spacing
        width = width - ((sp * 2) * cell_count);
        height = height - (sp * 2);

        let _item_width = width / cell_count;
        let _item_height = height / cell_count;
        let sp2 = sp * 2;

        let _y = (m_top + sp) as i32;
        let _x = (m_left + sp) as i32;

        for item in inner.children.iter() {
            let x = m_left + (sp + (sp2 * item.cell)) + (_item_width * item.cell);
            let y = m_top + (sp + (sp2 * item.cell)) + (_item_height * item.cell);

            let local_width = (_item_width * item.span) + (sp2 * (item.span - 1));
            let local_height = (_item_height * item.span) + (sp2 * (item.span - 1));

            match inner.ty {
                BoxLayoutType::Horizontal => unsafe {
                    wh::set_window_position(item.control, x as i32, _y);
                    wh::set_window_size(item.control, local_width, _item_height, false);
                },
                BoxLayoutType::Vertical => unsafe {
                    wh::set_window_position(item.control, _x, y as i32);
                    wh::set_window_size(item.control, width, local_height, false);
                },
            }
        }

    } 

}

pub struct BoxLayoutBuilder {
    layout: BoxLayoutInner
}

impl BoxLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> BoxLayoutBuilder {
        self.layout.base = p.into().hwnd().expect("Parent must be HWND");
        self
    }

    pub fn layout_type(mut self, ty: BoxLayoutType) -> BoxLayoutBuilder {
        self.layout.ty = ty;
        self
    }

    /// Add a children to the layout at the position `col` and `row`.
    /// This is a shortcut over `child_item` for item with default span.
    /// The handle must be a window object otherwise the function will panic
    pub fn child<W: Into<ControlHandle>>(mut self, cell: u32, c: W) -> BoxLayoutBuilder {
        let h = c.into().hwnd().expect("Child must be HWND");
        self.layout.children.push(BoxLayoutItem {
            control: h,
            cell,
            span: 1,
        });

        self
    }

    /// Add a children to the layout
    /// The handle must be a window object otherwise the function will panic
    pub fn child_item(mut self, item: BoxLayoutItem) -> BoxLayoutBuilder {
        self.layout.children.push(item);
        self
    }

    /// Set the margins of the layout. The four values are in this order: top, right, bottom, left.
    pub fn margin(mut self, m: [u32; 4]) -> BoxLayoutBuilder {
        self.layout.margins = m;
        self
    }

    /// Set the size of the space between the children in the layout. Default value is 5.
    pub fn spacing(mut self, sp: u32) -> BoxLayoutBuilder {
        self.layout.spacing = sp;
        self
    }

    /// Sets the minimum size of the layout
    pub fn min_size(mut self, sz: [u32; 2]) -> BoxLayoutBuilder {
        self.layout.min_size = sz;
        self
    }

    /// Sets the maximum size of the layout
    pub fn max_size(mut self, sz: [u32; 2]) -> BoxLayoutBuilder {
        self.layout.max_size = sz;
        self
    }

    /// Set the number of cell in the layout
    /// Cells without children control will be left blank
    pub fn cell_count(mut self, count: Option<u32>) -> BoxLayoutBuilder {
        self.layout.cell_count = count;
        self
    }

    /// Build the layout object and bind the callback.
    /// Children must only contains window object otherwise this method will panic.
    pub fn build(self, layout: &BoxLayout) {
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::{HIWORD, LOWORD};

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
                let width = LOWORD(size) as u32;
                let height = HIWORD(size) as u32;
                BoxLayout::update_layout(&event_layout, width, height);
            }
            None
        };

        bind_raw_event_handler(&base_handle, 0, cb);
    }

}

impl Default for BoxLayout {

    fn default() -> BoxLayout {
        let inner = BoxLayoutInner {
            base: ptr::null_mut(),
            ty: BoxLayoutType::Vertical,
            children: Vec::new(),
            margins: [5, 5, 5, 5],
            min_size: [0, 0],
            max_size: [u32::max_value(), u32::max_value()],
            cell_count: None,
            spacing: 5,
        };

        BoxLayout {
            inner: Rc::new(RefCell::new(inner))
        }
    }

}

