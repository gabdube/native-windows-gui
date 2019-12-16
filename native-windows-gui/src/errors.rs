/*!
    Error enums used in the native window gui crate
*/


#[derive(Debug, Clone)]
pub enum NwgError {
    Unknown,
    
    /// Fatal error that may happen when calling low level winapi functionalities
    InitializationError(String),

    ControlCreationError(String),
    MenuCreationError(String),
    ResourceCreationError(String),
    FileDialogError(String),

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

    pub fn file_dialog<S: Into<String>>(e: S) -> NwgError {
        NwgError::FileDialogError(e.into())
    }

    pub fn no_parent(name: &'static str) -> NwgError {
        NwgError::ControlCreationError(format!("No parent define for {:?} control", name))
    }

    pub fn no_parent_menu() -> NwgError {
        NwgError::MenuCreationError("No parent define for menu".to_string())
    }

}
