use super::{ControlHandle, Window, Button, CheckBox, RadioButton, TextInput, Label, ComboBox,
StatusBar, ImageFrame, ListBox, TrackBar, Menu, MenuItem, MenuSeparator, TextBox};
use std::convert::From;
use std::fmt::Display;


impl From<&Window> for ControlHandle {
    fn from(control: &Window) -> Self { control.handle }
}

impl PartialEq<ControlHandle> for Window {
    fn eq(&self, other: &ControlHandle) -> bool {
        self.handle == *other
    }
}

impl From<&Button> for ControlHandle {
    fn from(control: &Button) -> Self { control.handle }
}

impl From<&CheckBox> for ControlHandle {
    fn from(control: &CheckBox) -> Self { control.handle }
}

impl From<&RadioButton> for ControlHandle {
    fn from(control: &RadioButton) -> Self { control.handle }
}

impl From<&TextInput> for ControlHandle {
    fn from(control: &TextInput) -> Self { control.handle }
}

impl From<&Label> for ControlHandle {
    fn from(control: &Label) -> Self { control.handle }
}

impl<D: Display+Default> From<&ComboBox<D>> for ControlHandle {
    fn from(control: &ComboBox<D>) -> Self { control.handle }
}

impl From<&StatusBar> for ControlHandle {
    fn from(control: &StatusBar) -> Self { control.handle }
}

impl From<&ImageFrame> for ControlHandle {
    fn from(control: &ImageFrame) -> Self { control.handle }
}

impl<D: Display+Default> From<&ListBox<D>> for ControlHandle {
    fn from(control: &ListBox<D>) -> Self { control.handle }
}

impl From<&TrackBar> for ControlHandle {
    fn from(control: &TrackBar) -> Self { control.handle }
}

impl From<&TextBox> for ControlHandle {
    fn from(control: &TextBox) -> Self { control.handle }
}

impl From<&Menu> for ControlHandle {
    fn from(control: &Menu) -> Self { control.handle }
}

impl From<&MenuItem> for ControlHandle {
    fn from(control: &MenuItem) -> Self { control.handle }
}

impl From<&MenuSeparator> for ControlHandle {
    fn from(control: &MenuSeparator) -> Self { control.handle }
}


#[cfg(feature = "tabs")]
use super::{TabsContainer, Tab};

#[cfg(feature = "tabs")]
impl From<&TabsContainer> for ControlHandle {
    fn from(control: &TabsContainer) -> Self { control.handle }
}

#[cfg(feature = "tabs")]
impl From<&Tab> for ControlHandle {
    fn from(control: &Tab) -> Self { control.handle }
}

#[cfg(feature = "datetime-picker")]
use super::{DatePicker};

#[cfg(feature = "datetime-picker")]
impl From<&DatePicker> for ControlHandle {
    fn from(control: &DatePicker) -> Self { control.handle }
}

#[cfg(feature = "progress-bar")]
use super::{ProgressBar};

#[cfg(feature = "progress-bar")]
impl From<&ProgressBar> for ControlHandle {
    fn from(control: &ProgressBar) -> Self { control.handle }
}
