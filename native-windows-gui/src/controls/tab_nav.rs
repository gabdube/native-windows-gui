#![allow(unused)]

use winapi::shared::minwindef::LRESULT;
use crate::win32::window::{RawEventHandler, bind_raw_event_handler, unbind_raw_event_handler};
use crate::NwgError;
use super::ControlHandle;
use std::{rc::Rc, cell::RefCell};

static mut NAV_ID: usize = 0;


/// A resource that handles tab navigation in a window or a frame
/// Works by binding an raw event handler on every controls pushed into the TabNavigation
#[derive(Default)]
pub struct TabNavigation {
    id: usize,
    children: Rc<RefCell<Vec<ControlHandle>>>,
    handlers: RefCell<Vec<RawEventHandler>>,
}

impl TabNavigation {

    pub fn builder() -> TabNavigationBuilder {
        TabNavigationBuilder {
            parent: None,
            children: Vec::new()
        }
    }

}

pub struct TabNavigationBuilder {
    parent: Option<ControlHandle>,
    children: Vec<ControlHandle>
}

impl TabNavigationBuilder {

    /// The window parent of the tab nav
    pub fn parent<C: Into<ControlHandle>>(mut self, parent: C) -> TabNavigationBuilder {
        let parent = parent.into();
        assert!(parent.hwnd().is_some(), "Parent must be a window-like control.");

        self.parent = Some(parent);

        self
    }

    /// Add a control at the end of the tab navigation. Panics if the control is not a window-like control.
    pub fn push<C: Into<ControlHandle>>(mut self, control: C) -> TabNavigationBuilder {
        let control = control.into();
        assert!(control.hwnd().is_some(), "Control inside tab navigation must be a window-like controls.");
        
        self.children.push(control);
        self
    }

    /// Insert a control in the tab navigation. Panics if the control is not a window-like control.
    pub fn insert<C: Into<ControlHandle>>(mut self, index: usize, control: C) -> TabNavigationBuilder {
        let control = control.into();
        assert!(control.hwnd().is_some(), "Control inside tab navigation must be a window-like controls.");

        self.children.insert(index, control);
        self
    }

    pub fn builder(mut self, out: &mut TabNavigation) -> Result<(), NwgError> {
        use winapi::um::winuser::{WM_KEYDOWN, VK_TAB};

        let id = unsafe { NAV_ID += 1; NAV_ID };

        let parent = self.parent.expect("No parent set for the TabNavigation");
        self.children.push(parent);

        let children_count = self.children.len();

        let children_temp = self.children.clone();
        let children = Rc::new(RefCell::new(self.children));

        let mut handlers: Vec<RawEventHandler> = Vec::with_capacity(children_count);
        for child in children_temp {
            let handler_children = children.clone();
            let handler = bind_raw_event_handler(&child, id, move |hwnd, msg, w, l| {
                match msg {
                    WM_KEYDOWN => if w == (VK_TAB as usize) {
                        unsafe { Some(tab_update(&handler_children)) }
                    } else {
                        None
                    },
                    _ => None
                }
            });

            handlers.push(handler);
        }

        *out = TabNavigation {
            id,
            children,
            handlers: RefCell::new(handlers)
        };

        Ok(())
    }

}

impl Drop for TabNavigation {

    fn drop(&mut self) {
        let handlers = self.handlers.borrow();

        for handler in handlers.iter() {
            unbind_raw_event_handler(handler)
        }
    }

}

/// Low level handler of the tab navigation
unsafe fn tab_update(children: &Rc<RefCell<Vec<ControlHandle>>>) -> LRESULT {
    println!("TEST");

    0
}
