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
