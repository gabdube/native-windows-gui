/*!
    Error enums used in the native window gui crate
*/

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum SystemError {
    GetModuleHandleFailed,
    SystemClassCreationFailed,
    WindowCreationFailed,
    FontCreationFailed,
    ImageCreationFailed,
    MenuCreationFailed,
    FileDialogCreationFailed,
    SeparatorWithoutMenuParent
}

#[derive(Debug, Clone)]
pub enum UserError {
    FileDialog(String)
}
