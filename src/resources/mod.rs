/*!
    Resources trait definition
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

pub mod font;
pub mod brush;

use std::any::TypeId;
use std::hash::Hash;

use ui::Ui;
use controls::AnyHandle;
use error::Error;

pub use resources::font::{FontT, Font};
pub use resources::brush::{SolidBrushT, SolidBrush, LinearGradientBrushT, LinearGradientBrush, RadialGradientBrushT, RadialGradientBrush};

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