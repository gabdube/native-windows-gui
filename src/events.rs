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

use winapi::{UINT, DWORD, LPARAM, WPARAM};

use ui::Ui;
use defs::MouseButton;
use low::defs::NWG_DESTROY;

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
    A procedure signature that takes raw message parameters and output a EventArgs structure.
    It basically parse a Event::System message
*/
pub type SystemEventUnpackProc = Fn(&Event, UINT, WPARAM, LPARAM) -> EventArgs;

/**
    A procedure signature that takes raw message parameters and output a EventArgs structure.
    It basically parse a Event::Command message
*/
pub type CommandEventUnpackProc = Fn(&Event) -> EventArgs;

/**
    A procedure signature that takes raw message parameters and output a EventArgs structure.
    It basically parse a Event::Notify message
*/
pub type NotifyEventUnpackProc = Fn(&Event) -> EventArgs;


/**
    An enum that list different way to handle message by the Windows system
*/
pub enum Event {
    /// A message wildcard
    /// Control that accepts this message will catch every sytem message
    Any,

    /// A simple system message (ex: WM_KEYPRESS)
    System(UINT, &'static SystemEventUnpackProc),

    /// A WM_COMMAND message 
    /// This is the method used by built-in control to emit their events
    Command(DWORD, &'static CommandEventUnpackProc),

    /// A WM_NOTIFY message
    /// This is the method used by built-in control to emit their events
    /// Used by newer control (such as the DatePicker control)
    Notify(DWORD, &'static NotifyEventUnpackProc),

    /// A custom message defined by a third party programmer
    /// The first argument is the TypeId of the associated control and the second parameter is a unique id defined by the programmer
    /// Internally, these message are located in the following range: WM_APP (0x8000) through 0xBFFF
    /// They are guaranteed be unique within an application
    Custom(TypeId, u16),

    // TODO: implement someday (prority: very low)
    // A custom message defined by a third party programmer
    // The argument should be a unique string
    // Internally, these message are located in the following range :0xC000 through 0xFFFF
    // They are guaranteed to be unique across the WHOLE system and can be used to communicate between applications
    // CustomGlobal(String)
}

/// A default function that can be used to define a system event that do not return any argument
pub fn system_event_unpack_no_args(evt: &Event, msg: UINT, w: WPARAM, l: LPARAM) -> EventArgs { EventArgs::None }

/// A default function that can be used to define a command event that do not return any argument
pub fn command_event_unpack_no_args(evt: &Event) -> EventArgs { EventArgs::None }

/// A default function that can be used to define a notify event that do not return any argument
pub fn notify_event_unpack_no_args(evt: &Event) -> EventArgs { EventArgs::None }


pub const Destroy: Event = Event::System(NWG_DESTROY, &system_event_unpack_no_args);

/*
    Events name definition
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
*/

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