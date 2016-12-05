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

use std::any::TypeId;

use winapi::HWND;

pub use controls::window::{WindowT, Window};
use events::Event;
use error::Error;

pub enum AnyHandle {
    HWND(HWND)
}

/**
    Structures implementing this trait can be used by a Ui to build a Control
*/
pub trait ControlT {

    fn type_id(&self) -> TypeId;

    fn build(&self) -> Result<Box<Control>, Error>;

    fn events(&self) -> Vec<Event> { Vec::new() }
}

/**
    Structures implementing this trait are visual control that can be stored in a Ui
*/
pub trait Control {

    fn handle(&self) -> AnyHandle;

    fn free(&mut self) {}

}