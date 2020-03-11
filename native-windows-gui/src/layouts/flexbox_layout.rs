use crate::controls::ControlHandle;
use crate::win32::window_helper as wh;
use crate::win32::window::{RawEventHandler, unbind_raw_event_handler, bind_raw_event_handler};
use winapi::shared::windef::HWND;
use std::{ptr, rc::Rc, cell::RefCell};

use stretch::{
    number::Number,
    geometry::{Point, Size, Rect},
    node::{Node, Stretch},
    style::*
};


#[derive(Debug)]
pub struct FlexboxLayoutItem {
    /// The handle to the control in the item
    control: HWND,
    style: Style,
}

/// This is the inner data shared between the callback and the application
pub struct FlexboxLayoutInner {
    base: HWND,
    handler: Option<RawEventHandler>,
    style: Style,
    children: Vec<FlexboxLayoutItem>,
}


/**
    A flexbox layout that organizes the children control in a parent control.
    Flexbox uses the stretch library internally ( https://github.com/vislyhq/stretch ).
*/
#[derive(Clone)]
pub struct FlexboxLayout {
    inner: Rc<RefCell<FlexboxLayoutInner>>
}

impl FlexboxLayout {

    pub fn builder() -> FlexboxLayoutBuilder {
        let layout = FlexboxLayoutInner {
            base: ptr::null_mut(),
            handler: None,
            style: Default::default(),
            children: Vec::new()
        };

        FlexboxLayoutBuilder { layout, current_index: None, auto_size: true, auto_spacing: Some(5) }
    }

    fn update_layout(&self, width: u32, height: u32) -> Result<(), stretch::Error> {
        let inner = self.inner.borrow();
        if inner.base.is_null() || inner.children.len() == 0 {
            return Ok(());
        }

        let mut stretch = Stretch::new();
        let mut children: Vec<Node> = Vec::with_capacity(inner.children.len());

        for child in inner.children.iter() {
            children.push(stretch.new_node(child.style, Vec::new())?);
        }

        let mut style = inner.style.clone();
        style.size = Size { width: Dimension::Points(width as f32), height: Dimension::Points(height as f32) };
        let node = stretch.new_node(style, children.clone())?;

        stretch.compute_layout(node, Size::undefined())?;

        for (node, child) in children.into_iter().zip(inner.children.iter()) {
            let layout = stretch.layout(node)?;
            let Point { x, y } = layout.location;
            let Size { width, height } = layout.size;
            
            unsafe {
                wh::set_window_position(child.control, x as i32, y as i32);
                wh::set_window_size(child.control, width as u32, height as u32, false);
            }
        }

        Ok(())
    }

}

pub struct FlexboxLayoutBuilder {
    layout: FlexboxLayoutInner,
    current_index: Option<usize>,
    auto_size: bool,
    auto_spacing: Option<u32>
}

impl FlexboxLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> FlexboxLayoutBuilder {
        self.layout.base = p.into().hwnd().expect("Parent must be HWND");
        self
    }

    /// Add a new child to the layout build.
    /// Panics if `child` is not a window-like control.
    pub fn child<W: Into<ControlHandle>>(mut self, child: W) -> FlexboxLayoutBuilder {
        self.current_index = Some(self.layout.children.len());
        self.layout.children.push(FlexboxLayoutItem {
            control: child.into().hwnd().unwrap(),
            style: Style::default()
        });
        self
    }

    /// Make it so that the children of the layout all have equal size
    /// This flags is erased when `size`, `max_size`, or `min_size` is set on the children.
    pub fn auto_size(mut self, auto: bool) -> FlexboxLayoutBuilder {
        self.auto_size = auto;
        self
    }

    /// Automatically generate padding and margin for the parent layout and the children from the selected value.
    /// This flags is erased when `padding` is called on the layout or when `child_margin` is called on the children
    pub fn auto_spacing(mut self, auto: Option<u32>) -> FlexboxLayoutBuilder {
        self.auto_spacing = auto;
        self
    }

    //
    // Base layout style
    //

    pub fn direction(mut self, value: Direction) -> FlexboxLayoutBuilder {
        self.layout.style.direction = value;
        self
    }

    pub fn flex_direction(mut self, value: FlexDirection) -> FlexboxLayoutBuilder {
        self.layout.style.flex_direction = value;
        self
    }

    pub fn flex_wrap(mut self, value: FlexWrap) -> FlexboxLayoutBuilder {
        self.layout.style.flex_wrap = value;
        self
    }

    pub fn overflow(mut self, value: Overflow) -> FlexboxLayoutBuilder {
        self.layout.style.overflow = value;
        self
    }

    pub fn align_items(mut self, value: AlignItems) -> FlexboxLayoutBuilder {
        self.layout.style.align_items = value;
        self
    }

    pub fn align_content(mut self, value: AlignContent) -> FlexboxLayoutBuilder {
        self.layout.style.align_content = value;
        self
    }

    pub fn justify_content(mut self, value: JustifyContent) -> FlexboxLayoutBuilder {
        self.layout.style.justify_content = value;
        self
    }

    pub fn padding(mut self, value: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.layout.style.padding = value;
        self.auto_spacing = None;
        self
    }

    pub fn border(mut self, value: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.layout.style.border = value;
        self
    }

    pub fn flex_grow(mut self, value: f32) -> FlexboxLayoutBuilder {
        self.layout.style.flex_grow = value;
        self
    }

    pub fn flex_shrink(mut self, value: f32) -> FlexboxLayoutBuilder {
        self.layout.style.flex_shrink = value;
        self
    }

    pub fn flex_basis(mut self, value: Dimension) -> FlexboxLayoutBuilder {
        self.layout.style.flex_basis = value;
        self
    }

    pub fn min_size(mut self, value: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.layout.style.min_size = value;
        self
    }

    pub fn max_size(mut self, value: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.layout.style.max_size = value;
        self
    }

    pub fn aspect_ratio(mut self, value: Number) -> FlexboxLayoutBuilder {
        self.layout.style.aspect_ratio = value;
        self
    }

    //
    // Child layout style
    //

    /// Set the size of of the current child.
    /// Panics if `child` was not called before.
    pub fn child_size(mut self, size: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.current_child().style.size = size;
        self.auto_size = false;
        self
    }

    /// Set the position of the current child.
    /// Panics if `child` was not called before.
    pub fn child_position(mut self, position: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.current_child().style.position = position;
        self
    }

    /// Set the margin of the current child.
    /// Panics if `child` was not called before.
    pub fn child_margin(mut self, value: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.current_child().style.margin = value;
        self.auto_spacing = None;
        self
    }

    /// Set the min size of the current child.
    /// Panics if `child` was not called before.
    pub fn child_min_size(mut self, value: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.current_child().style.min_size = value;
        self.auto_size = false;
        self
    }

    /// Set the max size of the current child.
    /// Panics if `child` was not called before.
    pub fn child_max_size(mut self, value: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.current_child().style.max_size = value;
        self.auto_size = false;
        self
    }


    /**
        Directly set the style parameter of the current child. Panics if `child` was not called before.
        
        If defining style is too verbose, other method such as `size` can be used.
    */
    pub fn style(mut self, style: Style) -> FlexboxLayoutBuilder {
        self.current_child().style = style;
        self
    }

    fn current_child(&mut self) -> &mut FlexboxLayoutItem {
        assert!(self.current_index.is_some(), "No current children");
        &mut self.layout.children[self.current_index.unwrap()]
    }

    /// Build the layout object and bind the callback.
    /// Children must only contains window object otherwise this method will panic.
    pub fn build(mut self, layout: &FlexboxLayout) {
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::{HIWORD, LOWORD};

        let (w, h) = unsafe { wh::get_window_size(self.layout.base) };
        let base_handle = ControlHandle::Hwnd(self.layout.base);

        // Auto compute size if enabled
        if self.auto_size {
            let children_count = self.layout.children.len();
            let size = 1.0f32 / (children_count as f32);
            for child in self.layout.children.iter_mut() {
                match &self.layout.style.flex_direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        child.style.size = Size { width: Dimension::Percent(size), height: Dimension::Auto };
                    },
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        child.style.size = Size { width: Dimension::Auto, height: Dimension::Percent(size) };
                    }
                }
            }
        }

        // Auto spacing if enabled
        if let Some(spacing) = self.auto_spacing {
            let spacing = Dimension::Points(spacing as f32);
            let spacing = Rect { start: spacing, end: spacing, top: spacing, bottom: spacing};
            self.layout.style.padding = spacing;
            for child in self.layout.children.iter_mut() {
                child.style.margin = spacing;
            }
        }

        // Saves the new layout. Free the old layout (if there is one)
        {
            let mut layout_inner = layout.inner.borrow_mut();
            if layout_inner.handler.is_some() {
                unbind_raw_event_handler(layout_inner.handler.as_ref().unwrap());
            }
            
            *layout_inner = self.layout;        
        }

        // Initial layout update
        layout.update_layout(w, h).expect("Failed to compute layout");

        // Fetch a new ID for the layout handler
        static mut FLEX_LAYOUT_ID: usize = 0x9FFF; 
        let handler_id = unsafe { FLEX_LAYOUT_ID += 1; FLEX_LAYOUT_ID };
 
        // Bind the event handler
        let event_layout = layout.clone();
        let cb = move |_h, msg, _w, l| {
            if msg == WM_SIZE {
                let size = l as u32;
                let width = LOWORD(size) as u32;
                let height = HIWORD(size) as u32;
                FlexboxLayout::update_layout(&event_layout, width, height).expect("Failed to compute layout!");
            }
            None
        };

        {
            let mut layout_inner = layout.inner.borrow_mut();
            layout_inner.handler = Some(bind_raw_event_handler(&base_handle, handler_id, cb));
        }
    }
}

impl Default for FlexboxLayout {

    fn default() -> FlexboxLayout {
        let inner = FlexboxLayoutInner {
            base: ptr::null_mut(),
            handler: None,
            children: Vec::new(),
            style: Default::default(),
        };

        FlexboxLayout {
            inner: Rc::new(RefCell::new(inner))
        }
    }

}
