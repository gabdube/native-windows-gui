/*!
    Resources trait definition
*/

pub mod font;
pub mod image;
pub mod canvas;

use std::any::TypeId;
use std::hash::Hash;

use ui::Ui;
use controls::AnyHandle;
use error::Error;

pub use self::font::{FontT, Font};
pub use self::image::{ImageT, OemImageT, Image};
pub use self::canvas::{BrushT, Brush, PenT, Pen};

/**
    Structures implementing this trait can be used by a Ui to build a Resource
*/
pub trait ResourceT<ID: Clone+Hash> {

    /**
        Should return the TypeId of the generated resource. For example a `FontT` struct returns the TypeId of a `Font` struct.
    */
    fn type_id(&self) -> TypeId;

    /**
        Should instance the resource and return it as a Box<Resource>. If an error is raised, it will be returned by `ui.commit`.
    */
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Resource>, Error>;
}

/**
    Structures implementing this trait are resources that can be stored in a Ui
*/
pub trait Resource {

    /**
        Should return the underlying handle to the object
    */
    fn handle(&self) -> AnyHandle;

    /**
        If specified, should free any ressource allocated in the template `build` function.
    */
    fn free(&mut self) {}

}