use crate::controls::ControlHandle;
use crate::win32::window_helper as wh;
use crate::win32::window::{RawEventHandler, unbind_raw_event_handler, bind_raw_event_handler};
use winapi::shared::windef::HWND;
use std::{ptr, rc::Rc, cell::RefCell};


/// This is the inner data shared between the callback and the application
pub struct FlexboxLayoutInner {
    base: HWND,
    handler: Option<RawEventHandler>,
}


/**
    A flexbox layout that organizes the children control in a parent control
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
        };

        FlexboxLayoutBuilder { layout }
    }

    fn update_layout(&self, mut width: u32, mut height: u32) -> Result<(), stretch::Error> {
        use stretch::{
            geometry::Size,
            node::Stretch,
            style::{Style, Dimension, JustifyContent}
        };

        let mut stretch = Stretch::new();

        let node = stretch.new_node(    
            Style {
                size: Size { width: Dimension::Points(width as f32), height: Dimension::Points(height as f32) },
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            Vec::new()
        )?;

        stretch.compute_layout(node, Size::undefined())?;
        let test = stretch.layout(node);
        println!("{:?}", test);

        Ok(())
    }

}

pub struct FlexboxLayoutBuilder {
    layout: FlexboxLayoutInner
}

impl FlexboxLayoutBuilder {

    /// Set the layout parent. The handle must be a window object otherwise the function will panic
    pub fn parent<W: Into<ControlHandle>>(mut self, p: W) -> FlexboxLayoutBuilder {
        self.layout.base = p.into().hwnd().expect("Parent must be HWND");
        self
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
            if !layout_inner.handler.is_some() {
                unbind_raw_event_handler(layout_inner.handler.as_ref().unwrap());
            }
            
            *layout_inner = self.layout;        
        }

        // Initial layout update
        layout.update_layout(w, h).unwrap();

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
                FlexboxLayout::update_layout(&event_layout, width, height).unwrap();
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
        };

        FlexboxLayout {
            inner: Rc::new(RefCell::new(inner))
        }
    }

}
