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

    // When the list of a combobox is closed
    OnComboBoxClosed,

    // When the list of a combobox is about to be visible
    OnComboBoxDropdown,

    // When a combobox item is clicked twice rapidly
    OnComboBoxDoubleClick,

    // When the current selection of the combobox was changed
    OnComboxBoxSelection,

    // When the user click on a menu item
    OnMenuItemClick,

    /// When a timer delay is elapsed
    OnTimerTick,

    /// When a notice is... noticed
    OnNotice,

    /// When a user click on the X button of a window
    OnWindowClose,

    /// When most control receive keyboard focus
    OnFocus,

    /// When most control lose keyboard focus
    OnFocusLost
}
