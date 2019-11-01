use super::{ControlHandle, Window, Button, CheckBox, RadioButton, TextInput, Label, ComboBox,
StatusBar, ImageFrame, ListBox, TrackBar};
use std::convert::From;
use std::fmt::Display;


impl From<&Window> for ControlHandle {
    fn from(control: &Window) -> Self { control.handle }
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

