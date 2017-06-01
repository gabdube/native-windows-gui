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

use ui::Ui;
use defs::MouseButton;

use winapi::{WPARAM, LPARAM};

// System events that can be applied to any HWND based control
pub use low::events::{Event, Destroyed, Paint, Closed, Moved, KeyDown, KeyUp, Resized, Char, MouseUp, MouseDown};

// Control specfic events
pub mod button { pub use low::events::{BtnClick as Click, BtnDoubleClick as DoubleClick, BtnFocus as Focus}; }
pub use self::button as checkbox; // Checkboxes use the same events of the buttons
pub use self::button as radiobutton; // Radiobuttons use the same events of the buttons
pub mod combobox { pub use low::events::{CbnFocus as Focus, CbnSelectionChanged as SelectionChanged}; }
pub mod label { pub use low::events::{StnClick as Click, StnDoubleClick as DoubleClick}; }
pub mod datepicker { pub use low::events::DateChanged; }
pub mod listbox { pub use low::events::{LbnSelectionChanged as SelectionChanged, LbnDoubleClick as DoubleClick, LbnFocus as Focus}; }
pub mod textbox { pub use low::events::{EnFocus as Focus, EnLimit as Limit, EnValueChanged as ValueChanged}; }
pub use self::textbox as textinput; // Textinput use the same events of the textbox
pub mod menu { pub use low::events::MenuTrigger as Triggered; }
pub mod timer { pub use low::events::TimerTick as Tick; }

pub use self::Event::Any as Any;

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
    Raw(u32, WPARAM, LPARAM), // MSG, WPARAM, LPARAM
    None
}