#[macro_use]
extern crate bitflags;
extern crate winapi;

use std::rc::Rc;

#[cfg(feature="all")]
#[cfg(test)]
mod tests;

mod errors;
pub use errors::{NwgError};

mod events;
pub use events::*;

mod common_types;
pub use common_types::*;

pub(crate) mod win32;
pub use win32::{
 dispatch_thread_events, stop_thread_dispatch, enable_visual_styles, init_common_controls, 
 window::{EventHandler, full_bind_event_handler, bind_event_handler, unbind_event_handler, bind_raw_event_handler},
 message_box::{MessageButtons, MessageIcons, MessageChoice, MessageParams, message, fatal_message, error_message, simple_message}};

#[cfg(feature="cursor")]
pub use win32::cursor::GlobalCursor;

#[cfg(feature="clipboard")]
pub use win32::clipboard::Clipboard;

mod resources;
pub use resources::*;

mod controls;
pub use controls::*;

mod layouts;
pub use layouts::*;


pub trait PartialUi<D> {
    fn build_partial<W: Into<ControlHandle>>(d: &mut D, parent: Option<W>) -> Result<(), NwgError>;
    fn process_event(&self, _evt: Event, _evt_data: &EventData, _handle: ControlHandle) {}
    fn handles<'a>(&'a self) -> Vec<&'a ControlHandle> { vec![] }
}

pub trait NativeUi<D, UI> {
    fn build_ui(d: D) -> Result<Rc<UI>, NwgError>;
}


/// Initialize some application wide GUI settings.
/// This includes default styling and common controls resources.
pub fn init() -> std::result::Result<(), errors::NwgError> {
    enable_visual_styles();
    init_common_controls()
}
