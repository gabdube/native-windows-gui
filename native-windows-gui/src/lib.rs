#[macro_use]
extern crate bitflags;
extern crate winapi;

use std::rc::Rc;

#[cfg(test)] mod tests;
#[cfg(test)] mod tests_layout;

mod errors;
pub use errors::{UserError, SystemError};

mod events;
pub use events::Event;

pub(crate) mod win32;
pub use win32::{dispatch_thread_events, stop_thread_dispatch, enable_visual_styles, init_common_controls, 
 window::{bind_event_handler, bind_raw_event_handler},
 message_box::{MessageButtons, MessageIcons, MessageChoice, MessageParams, message, fatal_message, error_message, simple_message}};

mod resources;
pub use resources::*;

mod controls;
pub use controls::*;

mod layouts;
pub use layouts::*;


pub trait PartialUi<D> {
    fn build_partial(d: &mut D, parent: Option<&ControlBase>) -> Result<(), SystemError>;
}

pub trait NativeUi<D, UI> {
    fn build_ui(d: D) -> Result<Rc<UI>, SystemError>;
}
