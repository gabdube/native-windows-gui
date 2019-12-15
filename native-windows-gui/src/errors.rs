/*!
    Error enums used in the native window gui crate
*/

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum SystemError {
    Todo,
    GetModuleHandleFailed,
    CoInitializeFailed,
    SystemClassCreationFailed,
    WindowCreationFailed,
    FontCreationFailed,
    ImageCreationFailed,
    MenuCreationFailed,
    CanvasRendererCreationFailed,
    CanvasRenderTargetCreationFailed,
    FileDialogCreationFailed,
    SeparatorWithoutMenuParent,
    PopMenuWithoutParent,
    ControlWithoutParent,
    WrongParentType
}

#[derive(Debug, Clone)]
pub enum UserError {
    FileDialog(String)
}

#[derive(Debug, Clone)]
pub enum NwgError {
    Unknown,
    
    /// Fatal error that may happen when calling low level winapi functionalities
    InitializationError(String),

    /// Error raised when the creation of a control failed.
    ControlCreationError(String),

}

impl NwgError {

    pub fn initialization<S: Into<String>>(e: S) -> NwgError {
        NwgError::InitializationError(e.into())
    }

    pub fn control_create<S: Into<String>>(e: S) -> NwgError {
        NwgError::ControlCreationError(e.into())
    }

    pub fn no_parent(name: &'static str) -> NwgError {
        NwgError::ControlCreationError(format!("No parent define for {:?} control", name))
    }

}
