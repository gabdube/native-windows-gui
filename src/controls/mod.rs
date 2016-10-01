/*!
    Holds various wrapper over Windows native controls, each in 
    their own module.
*/

pub mod base;
pub mod window;
pub mod button;
pub mod checkbox;
pub mod groupbox;
pub mod radiobutton;
pub mod textinput;

pub use controls::window::Window;
pub use controls::button::Button;
pub use controls::checkbox::CheckBox;
pub use controls::groupbox::GroupBox;
pub use controls::radiobutton::RadioButton;
pub use controls::textinput::TextInput;

use std::hash::Hash;
use winapi::HWND;

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
}


pub fn cleanup() {
    unsafe { base::cleanup(); }
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

pub fn free_handle<T>(handle: HWND) {
    unsafe { base::free_handle::<T>(handle); }
}