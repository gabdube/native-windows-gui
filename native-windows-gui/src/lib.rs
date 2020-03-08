#[macro_use]
extern crate bitflags;
extern crate winapi;
pub extern crate stretch;

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
 dispatch_thread_events, dispatch_thread_events_with_callback, stop_thread_dispatch, enable_visual_styles, init_common_controls, 
 window::{
     EventHandler, RawEventHandler,
     full_bind_event_handler, bind_event_handler, unbind_event_handler,
     bind_raw_event_handler, has_raw_handler, unbind_raw_event_handler
 },
 message_box::{MessageButtons, MessageIcons, MessageChoice, MessageParams, message, fatal_message, error_message, simple_message}};

#[cfg(feature="cursor")]
pub use win32::cursor::GlobalCursor;

#[cfg(feature="clipboard")]
pub use win32::clipboard::{Clipboard, ClipboardFormat, ClipboardData};

mod resources;
pub use resources::*;

mod controls;
pub use controls::*;

mod layouts;
pub use layouts::*;

#[cfg(feature = "winnls")]
mod winnls;

#[cfg(feature = "winnls")]
pub use winnls::*;


/**
    PartialUi is a trait that can be implemented over a GUI structure that can be reused.

    The goal of `NativeUi` and `PartialUi` is to provide a common way to define NWG applications.
    Native-windows-derive can automatically implements this trait.

    For an example on how to implement this trait
*/
pub trait PartialUi<D> {
    fn build_partial<W: Into<ControlHandle>>(d: &mut D, parent: Option<W>) -> Result<(), NwgError>;
    fn process_event(&self, _evt: Event, _evt_data: &EventData, _handle: ControlHandle) {}
    fn handles<'a>(&'a self) -> Vec<&'a ControlHandle> { vec![] }
}

/**
    NativeUi is a trait that can be implemented over a GUI structure.

    The goal of `NativeUi` and `PartialUi` is to provide a common way to define NWG applications.
    Native-windows-derive can automatically implements this trait.

    For an example on how to implement this trait, see the **Small application layout** section in the NWG documentation.
*/
pub trait NativeUi<D, UI> {
    fn build_ui(d: D) -> Result<UI, NwgError>;
}


/// Initialize some application wide GUI settings.
/// This includes default styling and common controls resources.
pub fn init() -> std::result::Result<(), errors::NwgError> {
    enable_visual_styles();
    init_common_controls()
}
