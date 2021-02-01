/**
    List of task the UI can do. The main application state generate those tasks when its data is updated.

    The tasks are read by the UI after an action that could modify the application was executed.
*/
#[derive(Debug, Clone)]
pub enum GuiTask {
    /// If the UI should enable/disable the UI controls
    EnableUi(bool),

    /// A new window title should be set
    UpdateWindowTitle(String),
}
