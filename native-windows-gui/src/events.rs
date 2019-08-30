/// All the events that can be dispatched by the built-in controls of native-windows-gui
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// Undefined / not implemented event
    Unknown,

    /// When a button is clicked. Similar to a MouseUp event
    OnButtonClick,

    /// When a button is clicked twice rapidly
    OnButtonDoubleClick,

    /// When a TextInput value is changed
    OnTextInput,

    /// When a user click on the X button of a window
    OnWindowClose,

    /// When most control receive keyboard focus
    OnFocus,

    /// When most control lose keyboard focus
    OnFocusLost
}
