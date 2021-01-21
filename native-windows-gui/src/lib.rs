#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

extern crate winapi;

#[cfg(feature="flexbox")]
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
 message_box::*
};

pub(crate) use win32::window::bind_raw_event_handler_inner;

#[allow(deprecated)]
pub use win32::high_dpi::{set_dpi_awareness, scale_factor, dpi};

pub use win32::monitor::Monitor;

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
    A structure that implements this trait is considered a GUI structure. The structure will hold GUI components and possibly user data.

    A structure that implements `PartialUi` must be part of another UI structure and cannot be used as it is. It will most likely be used
    as the struct member of another struct that implements `NativeUi`.

    The goal of `NativeUi` and `PartialUi` is to provide a common way to define NWG applications.
    Native-windows-derive can automatically implement this trait.

    For an example on how to implement this trait, see the **Small application layout** section in the NWG documentation.
*/
pub trait PartialUi {
    /**
        Should initialize the GUI components. Similar to `NativeUi::build_ui` except it doesn't handle event binding.

        Parameters:
          - `data`: A reference to the struct data from the parent struct
          - `parent`: An optional reference to the parent UI control. If this is defined, the ui controls of the partial should be children of this value.
    */
    fn build_partial<W: Into<ControlHandle>>(data: &mut Self, parent: Option<W>) -> Result<(), NwgError>;

    /**
        Should process the events of the partial. This method will probably be called from an event handler bound in the parent GUI structure.

        Parameters:
          - `base`: A reference to the parent struct data 
          - `evt`: The event raised
          - `evt_data`: The data of the event raised
          - `handle`: Handle of the control that raised the event
    */
    fn process_event(&self, _evt: Event, _evt_data: &EventData, _handle: ControlHandle) {}

    /**
        Should return the handles of the top level parent controls (such as Windows). Those handle should be used to bind
        the default events handler.
    */
    fn handles<'a>(&'a self) -> Vec<&'a ControlHandle> { vec![] }
}

/**
    A structure that implements this trait is considered a GUI structure. The structure will hold GUI components and possibly user data. 

    The goal of `NativeUi` and `PartialUi` is to provide a common way to define NWG applications.
    Native-windows-derive can automatically implement this trait.

    For an example on how to implement this trait, see the **Small application layout** section in the NWG documentation.
*/
pub trait NativeUi<UI> {

    /**
        A constructor for the structure. It should initialize the GUI components and bind GUI events.

        Parameters:
          - `inital_state`: should contain the initial user data. `NativeUi` assumes this data will be wrapped in another type (`UI`).
    */
    fn build_ui(inital_state: Self) -> Result<UI, NwgError>;
}


/// Initializes some application wide GUI settings.
/// This includes default styling and common controls resources.
pub fn init() -> std::result::Result<(), errors::NwgError> {
    if cfg!(not(feature="no-styling")) {
        enable_visual_styles();
    }
    
    init_common_controls()
}
