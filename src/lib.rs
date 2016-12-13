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

extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate comctl32;
extern crate gdi32;

mod low;
mod defs;
mod error;
mod events;
mod controls;
mod resources;
mod ui;

pub use error::{Error, SystemError};
pub use events::{EventCallback, Event, EventArgs};
pub use controls::{ControlT, Control, WindowT, Window, MenuT, Menu, MenuItemT, MenuItem, ButtonT, Button, AnyHandle};
pub use resources::{ResourceT, Resource, FontT, Font};
pub use ui::{Ui, dispatch_events, exit};

pub mod constants {
    /*!
        Re-exported controls constants
    */
    pub use defs::*;

    pub use resources::font::{FONT_DECO_NORMAL, FONT_DECO_ITALIC, FONT_DECO_UNDERLINE, FONT_DECO_STRIKEOUT, FONT_WEIGHT_DONTCARE, FONT_WEIGHT_THIN, FONT_WEIGHT_EXTRALIGHT,
    FONT_WEIGHT_LIGHT, FONT_WEIGHT_NORMAL, FONT_WEIGHT_MEDIUM, FONT_WEIGHT_SEMIBOLD, FONT_WEIGHT_BOLD, FONT_WEIGHT_EXTRABOLD, FONT_WEIGHT_BLACK};
}