/*!
    Error enums used in the native window gui crate
*/

#[derive(Debug, Clone, Copy)]
pub enum SystemError {
    GetModuleHandleFailed,
    SystemClassCreationFailed,
    WindowCreationFailed,
    MenuCreationFailed,
    SeparatorWithoutMenuParent
}
