/*!
    Control trait definition. The base control definitions are located in the submodules.
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

pub mod window;
pub mod menu;
pub mod button;
pub mod textinput;
pub mod textbox;
pub mod checkbox;
pub mod radiobutton;
pub mod label;
pub mod listbox;
pub mod combobox;
pub mod file_dialog;
pub mod timer;
pub mod canvas;

use std::any::TypeId;
use std::hash::Hash;

use winapi::{HWND, HMENU, UINT, HFONT};

pub use controls::window::{WindowT, Window};
pub use controls::menu::{MenuT, Menu, MenuItemT, MenuItem, SeparatorT, Separator};
pub use controls::button::{ButtonT, Button};
pub use controls::textinput::{TextInputT, TextInput};
pub use controls::textbox::{TextBoxT, TextBox};
pub use controls::checkbox::{CheckBoxT, CheckBox};
pub use controls::radiobutton::{RadioButtonT, RadioButton};
pub use controls::label::{LabelT, Label};
pub use controls::listbox::{ListBoxT, ListBox};
pub use controls::combobox::{ComboBoxT, ComboBox};
pub use controls::file_dialog::{FileDialogT, FileDialog};
pub use controls::timer::{TimerT, Timer};
pub use controls::canvas::{CanvasT, Canvas, CanvasRenderer};
use ui::Ui;
use events::Event;
use error::Error;

/**
    A type that expose the different underlying handle into one type
*/
#[derive(Clone, Hash, Debug)]
#[allow(non_camel_case_types)]
pub enum AnyHandle {
    HWND(HWND),
    HMENU(HMENU),
    HMENU_ITEM(HMENU, UINT),
    HFONT(HFONT),
    Custom(TypeId, usize)
}

/**
    An enum that list type names for the common controls.

    This is used internally to differentiate the common control notification codes.
*/
#[derive(Clone, Debug)]
pub enum ControlType {
    Window,
    Button,
    TextInput,
    TextBox,
    CheckBox,
    RadioButton,
    ListBox,
    Label,
    Menu,
    MenuItem,
    Timer,
    ComboBox,
    NumericInput,
    FileDialog,
    Canvas,
    Undefined  // Control is not a common control
}

/**
    Structures implementing this trait can be used by a Ui to build a Control
*/
pub trait ControlT<ID: Clone+Hash> {

    /**
        Should return the TypeId of the generated control. For example a `WindowT` struct returns the TypeId of a `Window` struct.
    */
    fn type_id(&self) -> TypeId;

    /**
        Should instance the control and return it as a Box<Control>. If an error is raised, it will be returned by `ui.commit`.
    */
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error>;

    /**
        Should return the events supported by the control.
    */
    fn events(&self) -> Vec<Event> { Vec::new() }
}

/**
    Structures implementing this trait are controls that can be stored in a Ui
*/
pub trait Control {

    /**
        Should return the underlying handle to the object
    */
    fn handle(&self) -> AnyHandle;

    /**
        Should return the type of the control. For custom controls, the return value should be `Undefined` (the default).
    */
    fn control_type(&self) -> ControlType { ControlType::Undefined }

    /**
        If specified, should free any ressource allocated in the template `build` function. This includes functions like `DestroyWindow`.
    */
    fn free(&mut self) {}

}