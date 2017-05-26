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
use std::mem;

use winapi::{HWND, UINT, WORD, LPARAM, WPARAM};

use ui::Ui;
use defs::MouseButton;
pub use low::events::{Destroyed, Paint, Close, Moved, KeyDown, KeyUp, Resized,
  Char, MouseUp, MouseDown};

pub mod buttons {
    pub use low::events::{BtnClick as Click, BtnDoubleClick as DoubleClick, BtnFocus as Focus};
}

pub mod combobox {
    pub use low::events::{CbnFocus as Focus, CbnSelectionChanged as SelectionChanged};
}

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
pub type SystemEventUnpackProc = Fn(HWND, UINT, WPARAM, LPARAM) -> Option<EventArgs>;

/**
    A procedure signature that takes raw message parameters and output a EventArgs structure.
    It basically parse a Event::Command message
*/
pub type CommandEventUnpackProc = Fn(HWND, WORD) -> Option<EventArgs>;

/**
    A procedure signature that takes raw message parameters and output a EventArgs structure.
    It basically parse a Event::Notify message
*/
pub type NotifyEventUnpackProc = Fn() -> EventArgs;


/**
    An enum that list different way to handle message by the Windows system
*/
pub enum Event {
    /// A message wildcard
    /// Control that accepts this message will catch every sytem message
    Any,

    /// A simple system message (ex: WM_KEYPRESS)
    System(UINT, &'static SystemEventUnpackProc),

    /// Same as System, but accepts a list of message identifier
    /// Some NWG events can be triggered by different system events
    SystemGroup(&'static [UINT], &'static SystemEventUnpackProc),

    /// A WM_COMMAND message 
    /// This is the method used by built-in control to emit their events
    Command(WORD, &'static CommandEventUnpackProc),

    /// Same as Command, but accepts a list of message identifier
    CommandGroup(&'static [WORD], &'static CommandEventUnpackProc),

    /// A WM_NOTIFY message
    /// This is the method used by built-in control to emit their events
    /// Used by newer control (such as the DatePicker control)
    Notify(WORD, &'static NotifyEventUnpackProc),

    /// A custom message defined by a third party programmer
    /// The first argument is the TypeId of the associated control and the second parameter is a unique id defined by the programmer
    /// Internally, these message are located in the following range: WM_APP (0x8000) through 0xBFFF
    /// They are guaranteed be unique within an application
    Custom(TypeId, u16),

    // TODO: implement someday (priority: very low)
    // A custom message defined by a third party programmer
    // The argument should be a unique string
    // Internally, these message are located in the following range :0xC000 through 0xFFFF
    // They are guaranteed to be unique across the WHOLE system and can be used to communicate between applications
    // CustomGlobal(String)
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