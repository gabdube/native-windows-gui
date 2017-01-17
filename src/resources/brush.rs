/*!
    A brush resource that can be imported in a canvas
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
use std::any::TypeId;
use std::hash::Hash;

use ui::Ui;
use controls::AnyHandle;
use resources::{ResourceT, Resource};
use error::{Error, SystemError};

#[derive(Clone)]
pub struct SolidBrushT {

}

#[derive(Clone)]
pub struct SolidBrush {

}

#[derive(Clone)]
pub struct LinearGradientBrushT {

}

#[derive(Clone)]
pub struct LinearGradientBrush {

}

#[derive(Clone)]
pub struct RadialGradientBrushT {

}

#[derive(Clone)]
pub struct RadialGradientBrush {

}