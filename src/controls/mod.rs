/*!
    Holds various wrapper over Windows native controls, each in 
    their own module.
*/

use std::hash::Hash;
use winapi::HWND;

/**
    Trait that his shared by all control templates
*/
pub trait ControlTemplate<ID: Eq+Clone+Hash > {

    /**
        Create a new control from the template data.
    */
    fn create(&self,  ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()>;
}