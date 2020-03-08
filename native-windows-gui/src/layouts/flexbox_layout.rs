use crate::controls::ControlHandle;
use crate::win32::window_helper as wh;
use crate::win32::window::{RawEventHandler, unbind_raw_event_handler, bind_raw_event_handler};
use winapi::shared::windef::HWND;
use std::{ptr, rc::Rc, cell::RefCell};

use stretch::{
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

        FlexboxLayoutBuilder { layout, current_index: None }
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

        let node = stretch.new_node(    
            Style {
                size: Size { width: Dimension::Points(width as f32), height: Dimension::Points(height as f32) },
                ..Default::default()
            },
            children.clone()
        )?;

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
    current_index: Option<usize>
}

impl FlexboxLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> FlexboxLayoutBuilder {
        self.layout.base = p.into().hwnd().expect("Parent must be HWND");
        self
    }

    pub fn flex_direction(mut self, value: FlexDirection) -> FlexboxLayoutBuilder {
        self.layout.style.flex_direction = value;
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

    /// Set the size of of the current child.
    /// Panics if `child` was not called before.
    pub fn child_size(mut self, size: Size<Dimension>) -> FlexboxLayoutBuilder {
        self.current_child().style.size = size;
        self
    }

    /// Set the position of the current child.
    /// Panics if `child` was not called before.
    pub fn child_position(mut self, position: Rect<Dimension>) -> FlexboxLayoutBuilder {
        self.current_child().style.position = position;
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
    pub fn build(self, layout: &FlexboxLayout) {
        use winapi::um::winuser::WM_SIZE;
        use winapi::shared::minwindef::{HIWORD, LOWORD};

        let (w, h) = unsafe { wh::get_window_size(self.layout.base) };
        let base_handle = ControlHandle::Hwnd(self.layout.base);

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
