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

    /// Reload the data displayed in the ObjectInspector
    ReloadProjectSettings,

    /// If the deps of the project do not include nwg, ask the user if the app can add them
    AskUserUpdateDependencies,

    /// Clear all data from the gui
    ClearData,
}
