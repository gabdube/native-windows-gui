use super::{ControlHandle, Window, Button, CheckBox, RadioButton, TextInput, Label, ComboBox,
StatusBar, ImageFrame, ListBox, TrackBar, Menu, MenuItem, MenuSeparator, TextBox};
use std::convert::From;
use std::fmt::Display;

macro_rules! handles {
    ($control:ty) => {
        impl From<&$control> for ControlHandle {
            fn from(control: &$control) -> Self { control.handle }
        }

        impl PartialEq<ControlHandle> for $control {
            fn eq(&self, other: &ControlHandle) -> bool {
                self.handle == *other
            }
        }

        impl PartialEq<$control> for ControlHandle {
            fn eq(&self, other: &$control) -> bool {
                *self == other.handle
            }
        }
    };
}

handles!(Window);
handles!(Button);
handles!(CheckBox);
handles!(ImageFrame);
handles!(Label);
handles!(RadioButton);
handles!(TextBox);
handles!(TextInput);
handles!(StatusBar);
handles!(TrackBar);

handles!(Menu);
handles!(MenuItem);
handles!(MenuSeparator);


impl<D: Display+Default> From<&ComboBox<D>> for ControlHandle {
    fn from(control: &ComboBox<D>) -> Self { control.handle }
}

impl<D: Display+Default> PartialEq<ControlHandle> for ComboBox<D> {
    fn eq(&self, other: &ControlHandle) -> bool {
        self.handle == *other
    }
}

impl<D: Display+Default> PartialEq<ComboBox<D>> for ControlHandle {
    fn eq(&self, other: &ComboBox<D>) -> bool {
        *self == other.handle
    }
}

impl<D: Display+Default> From<&ListBox<D>> for ControlHandle {
    fn from(control: &ListBox<D>) -> Self { control.handle }
}

impl<D: Display+Default> PartialEq<ControlHandle> for ListBox<D> {
    fn eq(&self, other: &ControlHandle) -> bool {
        self.handle == *other
    }
}

impl<D: Display+Default> PartialEq<ListBox<D>> for ControlHandle {
    fn eq(&self, other: &ListBox<D>) -> bool {
        *self == other.handle
    }
}


#[cfg(feature = "tabs")]
use super::{TabsContainer, Tab};

#[cfg(feature = "tabs")]
handles!(TabsContainer);

#[cfg(feature = "tabs")]
handles!(Tab);

#[cfg(feature = "datetime-picker")]
use super::{DatePicker};

#[cfg(feature = "datetime-picker")]
handles!(DatePicker);

#[cfg(feature = "progress-bar")]
use super::{ProgressBar};

#[cfg(feature = "progress-bar")]
handles!(ProgressBar);
