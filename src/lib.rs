/*!
    A very high level native gui library for Windows.
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
#![cfg(windows)]

extern crate comctl32;
extern crate gdi32;
extern crate kernel32;
extern crate ole32;
extern crate user32;
extern crate winapi;

mod controls;
mod defs;
mod error;
mod events;
mod low;
mod resources;
mod ui;

pub mod templates;

pub mod custom {
    /*!
        Custom control creation resources
    */
    pub use controls::{AnyHandle, Control, ControlT};
    pub use low::window_helper::{
        build_sysclass, build_window, get_window_enabled, get_window_long, get_window_position,
        get_window_size, get_window_text, get_window_visibility, set_window_enabled,
        set_window_long, set_window_position, set_window_size, set_window_text,
        set_window_visibility, SysclassParams, WindowParams,
    };
    pub use resources::{Resource, ResourceT};
}

pub mod constants {
    /*!
        Controls constants
    */
    pub use controls::ControlType;
    pub use defs::*;
}

pub use controls::{
    Button, ButtonT, Canvas, CanvasRenderer, CanvasT, CheckBox, CheckBoxT, ComboBox, ComboBoxT,
    DatePicker, DatePickerT, FileDialog, FileDialogT, GroupBox, GroupBoxT, Label, LabelT, ListBox,
    ListBoxT, Menu, MenuItem, MenuItemT, MenuT, ProgressBar, ProgressBarT, RadioButton,
    RadioButtonT, Separator, SeparatorT, TextBox, TextBoxT, TextInput, TextInputT, Timer, TimerT,
    Window, WindowT,
};
pub use error::{Error, SystemError};
pub use events::{Event, EventArgs, EventCallback};
pub use low::other_helper::{error_message, fatal_message, message, simple_message};
pub use resources::{Font, FontT};
pub use ui::{dispatch_events, exit, Ui};
