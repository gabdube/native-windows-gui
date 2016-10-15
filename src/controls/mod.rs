/*!
    Holds various wrapper over Windows native controls, each in 
    their own module.
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

pub mod base;
pub mod window;
pub mod button;
pub mod checkbox;
pub mod groupbox;
pub mod radiobutton;
pub mod textinput;
pub mod combobox;
pub mod label;
pub mod frame;
pub mod textbox;

pub use controls::window::Window;
pub use controls::button::Button;
pub use controls::checkbox::CheckBox;
pub use controls::groupbox::GroupBox;
pub use controls::radiobutton::RadioButton;
pub use controls::textinput::TextInput;
pub use controls::combobox::ComboBox;
pub use controls::label::Label;
pub use controls::frame::Frame;
pub use controls::textbox::TextBox;

use std::hash::Hash;
use winapi::HWND;

use constants::ControlType;

/**
    Trait that is shared by all control templates
*/
pub trait ControlTemplate<ID: Eq+Clone+Hash > {

    /**
        Create a new control from the template data.
    */
    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()>;

    /**
        Return the function that will be used to evaluates the control actions
    */
    fn evaluator(&self) -> ::ActionEvaluator<ID>;

    /**
        Return the list of callback supported by this control
    */
    fn supported_events(&self) -> Vec<::events::Event>;

    /**
        Return the type of the control. This is used for reflecting
        and because the window proc need to know the control to dispatch
        the events correctly
    */
    fn control_type(&self) -> ControlType;
}

pub fn set_handle_data<T>(handle: HWND, data: T) {
    unsafe { base::set_handle_data(handle, data); }
}

pub fn get_handle_data<'a, T>(handle: HWND) -> &'a mut T {
    unsafe { base::get_handle_data(handle).unwrap() }
}

pub fn free_handle_data<T>(handle: HWND) {
    unsafe { base::free_handle_data::<T>(handle); }
}

pub fn free_handle<ID: Eq+Clone+Hash>(handle: HWND) {
    unsafe { base::free_handle::<ID>(handle); }
}