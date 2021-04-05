use std::fmt;
use std::error::Error;

#[cfg(feature = "plotting")]
use crate::win32::plotters_d2d::PlottersError;

/**
    Error enums used in the native window gui crate
*/
#[derive(Debug, Clone)]
pub enum NwgError {
    Unknown,
    
    /// Fatal error raised when calling low level winapi functionalities
    InitializationError(String),

    /// Error raised when creating a control.
    ControlCreationError(String),

    /// Error raised when creating a menu.
    MenuCreationError(String),

    /// Error raised when creating a resource.
    ResourceCreationError(String),

    /// Error raised when the creation of a layout failed
    LayoutCreationError(String),

    /// Error raised when an event handler could not be bound
    EventsBinding(String),

    /// Error raised by the FileDialog object
    #[cfg(feature = "file-dialog")]
    FileDialogError(String),

    /// Error raised by the ImageDecoder feature
    #[cfg(feature = "image-decoder")]
    ImageDecoderError(i32, String),

    /// Error raised by one of the locale functions
    #[cfg(feature = "winnls")]
    BadLocale(String),

    /// Error raised by one of the locale functions
    #[cfg(feature = "plotting")]
    Plotters(PlottersError),
}

impl NwgError {

    pub fn initialization<S: Into<String>>(e: S) -> NwgError {
        NwgError::InitializationError(e.into())
    }

    pub fn control_create<S: Into<String>>(e: S) -> NwgError {
        NwgError::ControlCreationError(e.into())
    }

    pub fn menu_create<S: Into<String>>(e: S) -> NwgError {
        NwgError::MenuCreationError(e.into())
    }

    pub fn resource_create<S: Into<String>>(e: S) -> NwgError {
        NwgError::ResourceCreationError(e.into())
    }

    pub fn layout_create<S: Into<String>>(e: S) -> NwgError {
        NwgError::LayoutCreationError(e.into())
    }

    pub fn events_binding<S: Into<String>>(e: S) -> NwgError {
        NwgError::EventsBinding(e.into())
    }

    #[cfg(feature = "file-dialog")]
    pub fn file_dialog<S: Into<String>>(e: S) -> NwgError {
        NwgError::FileDialogError(e.into())
    }

    #[cfg(feature = "winnls")]
    pub fn bad_locale<S: Into<String>>(e: S) -> NwgError {
        NwgError::BadLocale(e.into())
    }

    #[cfg(feature = "image-decoder")]
    pub fn image_decoder<S: Into<String>>(code: i32, e: S) -> NwgError {
        NwgError::ImageDecoderError(code, e.into())
    }

    pub fn no_parent(name: &'static str) -> NwgError {
        NwgError::ControlCreationError(format!("No parent defined for {:?} control", name))
    }

    pub fn no_parent_menu() -> NwgError {
        NwgError::MenuCreationError("No parent defined for menu".to_string())
    }

}

impl fmt::Display for NwgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NwgError::*;

        match self {
            Unknown => write!(f, "Unknown error. This should never happen"),
            InitializationError(reason) => write!(f, "Failed to initialize NWG: {:?}", reason),
            ControlCreationError(reason) => write!(f, "Failed to create a control: {:?}", reason),
            MenuCreationError(reason) => write!(f, "Failed to create a menu: {:?}", reason),
            ResourceCreationError(reason) => write!(f, "Failed to create a resource: {:?}", reason),
            LayoutCreationError(reason) => write!(f, "Failed to create a layout: {:?}", reason),
            EventsBinding(reason) => write!(f, "Failed to bind events: {:?}", reason),
            
            #[cfg(feature = "file-dialog")]
            FileDialogError(reason) => write!(f, "File dialog actions failed: {:?}", reason),

            #[cfg(feature = "image-decoder")]
            ImageDecoderError(_id, reason) => write!(f, "Image decoder failed: {:?}", reason),

            #[cfg(feature = "winnls")]
            BadLocale(reason) => write!(f, "Windows locale functions failed: {:?}", reason),

            #[cfg(feature = "plotting")]
            Plotters(reason) => write!(f, "Plotting canvas function failed: {}", reason),
        }
        
    }
}

#[cfg(feature = "plotting")]
impl From<PlottersError> for NwgError {
    fn from(e: PlottersError) -> Self {
        NwgError::Plotters(e)
    }
}

impl Error for NwgError {}
