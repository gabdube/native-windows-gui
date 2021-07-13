use super::{ControlHandle, Window, Button, CheckBox, RadioButton, TextInput, Label, ImageFrame};
use std::convert::From;

#[allow(unused)]
use std::fmt::Display;

macro_rules! handles {
    ($control:ty) => {
        #[allow(deprecated)]
        impl From<&$control> for ControlHandle {
            fn from(control: &$control) -> Self { control.handle }
        }

        #[allow(deprecated)]
        impl From<&mut $control> for ControlHandle {
            fn from(control: &mut $control) -> Self { control.handle }
        }

        #[allow(deprecated)]
        impl PartialEq<ControlHandle> for $control {
            fn eq(&self, other: &ControlHandle) -> bool {
                self.handle == *other
            }
        }

        #[allow(deprecated)]
        impl PartialEq<$control> for ControlHandle {
            fn eq(&self, other: &$control) -> bool {
                *self == other.handle
            }
        }
    };
}

/**
Automatically implements the functionnalities required to process an external struct as a NWG control

```rust
#[macro_use] extern crate native_windows_gui as nwg;

pub struct TestControl {
    edit: nwg::TextInput,
    custom_data: String,
}

subclass_control!(TestControl, TextInput, edit);
```
*/
#[macro_export]
macro_rules! subclass_control {
    ($ty:ident, $base_type:ident, $field: ident) => {
        impl ::std::ops::Deref for $ty {
            type Target = $crate::$base_type;
            fn deref(&self) -> &$crate::$base_type { &self.$field }
        }
        
        impl ::std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {&mut self.$field }
        }
        
        impl Into<$crate::ControlHandle> for &$ty {
            fn into(self) -> $crate::ControlHandle { self.$field.handle.clone() }
        }

        impl Into<$crate::ControlHandle> for &mut $ty {
            fn into(self) -> $crate::ControlHandle { self.$field.handle.clone() }
        }
        
        impl PartialEq<$ty> for $crate::ControlHandle {
            fn eq(&self, other: &$ty) -> bool {
                *self == other.$field.handle
            }
        }
        
    }
}

handles!(Window);
handles!(Button);
handles!(ImageFrame);
handles!(Label);
handles!(CheckBox);
handles!(RadioButton);
handles!(TextInput);


#[cfg(feature = "textbox")]
use super::TextBox;

#[cfg(feature = "textbox")]
handles!(TextBox);

#[cfg(feature = "status-bar")]
use super::StatusBar;

#[cfg(feature = "status-bar")]
handles!(StatusBar);

#[cfg(feature = "tooltip")]
use super::Tooltip;

#[cfg(feature = "tooltip")]
handles!(Tooltip);

#[cfg(feature = "trackbar")]
use super::TrackBar;

#[cfg(feature = "trackbar")]
handles!(TrackBar);

#[cfg(feature = "menu")]
use super::{Menu, MenuItem, MenuSeparator};

#[cfg(feature = "menu")]
handles!(Menu);
#[cfg(feature = "menu")]
handles!(MenuItem);
#[cfg(feature = "menu")]
handles!(MenuSeparator);

#[cfg(feature = "combobox")]
use super::ComboBox;

#[cfg(feature = "combobox")]
impl<D: Display+Default> From<&ComboBox<D>> for ControlHandle {
    fn from(control: &ComboBox<D>) -> Self { control.handle }
}

#[cfg(feature = "combobox")]
impl<D: Display+Default> PartialEq<ControlHandle> for ComboBox<D> {
    fn eq(&self, other: &ControlHandle) -> bool {
        self.handle == *other
    }
}

#[cfg(feature = "combobox")]
impl<D: Display+Default> PartialEq<ComboBox<D>> for ControlHandle {
    fn eq(&self, other: &ComboBox<D>) -> bool {
        *self == other.handle
    }
}

#[cfg(feature = "listbox")]
use super::ListBox;

#[cfg(feature = "listbox")]
impl<D: Display+Default> From<&ListBox<D>> for ControlHandle {
    fn from(control: &ListBox<D>) -> Self { control.handle }
}

#[cfg(feature = "listbox")]
impl<D: Display+Default> PartialEq<ControlHandle> for ListBox<D> {
    fn eq(&self, other: &ControlHandle) -> bool {
        self.handle == *other
    }
}

#[cfg(feature = "listbox")]
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
use super::ProgressBar;

#[cfg(feature = "progress-bar")]
handles!(ProgressBar);

#[cfg(feature = "tree-view")]
use super::TreeView;

#[cfg(feature = "tree-view")]
handles!(TreeView);

#[cfg(feature = "tray-notification")]
use super::TrayNotification;

#[cfg(feature = "tray-notification")]
handles!(TrayNotification);

#[cfg(feature = "message-window")]
use super::MessageWindow;

#[cfg(feature = "message-window")]
handles!(MessageWindow);

#[cfg(feature = "timer")]
#[allow(deprecated)]
use super::Timer;

#[cfg(feature = "timer")]
handles!(Timer);

#[cfg(feature = "animation-timer")]
use super::AnimationTimer;

#[cfg(feature = "animation-timer")]
handles!(AnimationTimer);

#[cfg(feature = "notice")]
use super::Notice;

#[cfg(feature = "notice")]
handles!(Notice);

#[cfg(feature = "list-view")]
use super::ListView;

#[cfg(feature = "list-view")]
handles!(ListView);

#[cfg(feature = "extern-canvas")]
use super::ExternCanvas;

#[cfg(feature = "extern-canvas")]
handles!(ExternCanvas);


#[cfg(feature = "frame")]
use super::Frame;

#[cfg(feature = "frame")]
handles!(Frame);


#[cfg(feature = "rich-textbox")]
use super::RichTextBox;

#[cfg(feature = "rich-textbox")]
handles!(RichTextBox);

#[cfg(feature = "rich-textbox")]
use super::RichLabel;

#[cfg(feature = "rich-textbox")]
handles!(RichLabel);

#[cfg(feature = "scroll-bar")]
use super::ScrollBar;

#[cfg(feature = "scroll-bar")]
handles!(ScrollBar);

#[cfg(feature = "number-select")]
use super::NumberSelect;

#[cfg(feature = "number-select")]
handles!(NumberSelect);

#[cfg(feature = "plotting")]
use super::Plotters;

#[cfg(feature = "plotting")]
handles!(Plotters);
