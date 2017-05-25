/*!
    High level events definitions
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

use std::time::Duration;
use std::any::TypeId;

use ui::Ui;
use defs::MouseButton;

/**
The function signature for the event callback

Arguments:  
  • 1: A reference to the Ui  
  • 2: A reference to the ID of the control  
  • 3: A reference to the event type that was called  
  • 4: A reference to the arguments passed with the controls  
*/
pub type EventCallback<ID> = Fn(&Ui<ID>, &ID, &Event, &EventArgs) -> ();

/**
*/
pub type EventType = (TypeId, usize);

/**
    Events name definition
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Event {

    // NWG special events
    Destroyed,

    // System events
    KeyDown,
    KeyUp,
    Char,
    Closed,
    MouseUp,
    MouseDown,
    Moved,
    Resized,
    Paint,
    Raw,

    // Default control specific events
    Click,
    DoubleClick,
    MenuOpen,
    MenuClose,
    SelectionChanged,
    ValueChanged,
    LimitReached,
    Focus,
    Tick,
    Triggered,
    DateChanged
}

/**
    Events arguments definition. If an event do not have arguments, EventArgs::None is passed.
*/
pub enum EventArgs {
    Key(u32),
    Char(char),
    MouseClick{btn: MouseButton, pos: (i32, i32)},
    Focus(bool),
    Tick(Duration),
    Position(i32, i32),
    Size(u32, u32),
    Raw(u32, usize, usize), // MSG, WPARAM, LPARAM
    None
}