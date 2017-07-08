/*!
    High level events definitions
*/

use std::time::Duration;

use ui::Ui;
use defs::MouseButton;

use winapi::{WPARAM, LPARAM};

// System events that can be applied to any HWND based control
pub use low::events::{Event, Destroyed, Paint, Closed, Moved, KeyDown, KeyUp, Resized, Char, MouseUp, MouseDown, MouseMove};

// Control specfic events
pub mod button { pub use low::events::{BtnClick as Click, BtnDoubleClick as DoubleClick, BtnFocus as Focus}; }
pub use self::button as checkbox; // Checkboxes use the same events of the buttons
pub use self::button as radiobutton; // Radiobuttons use the same events of the buttons
pub mod combobox { pub use low::events::{CbnFocus as Focus, CbnSelectionChanged as SelectionChanged}; }
pub mod label { pub use low::events::{StnClick as Click, StnDoubleClick as DoubleClick}; }
pub use self::label as image_frame;
pub mod datepicker { pub use low::events::DateChanged; }
pub mod listbox { pub use low::events::{LbnSelectionChanged as SelectionChanged, LbnDoubleClick as DoubleClick, LbnFocus as Focus}; }
pub mod textbox { pub use low::events::{EnFocus as Focus, EnLimit as Limit, EnValueChanged as ValueChanged}; }
pub use self::textbox as textinput; // Textinput use the same events of the textbox
pub mod menu { pub use low::events::MenuTrigger as Triggered; }
pub mod timer { pub use low::events::TimerTick as Tick; }
pub mod treeview { pub use low::events::{TreeViewSelectionChanged as SelectionChanged, TreeViewClick as Click, TreeViewDoubleClick as DoubleClick,
 TreeViewFocus as Focus, TreeViewDeleteItem as DeleteItem, TreeViewItemChanged as ItemChnaged, TreeViewItemChanging as ItemChanging,
 TreeViewItemExpanded as ItemExpanded, TreeViewItemExpanding as ItemExpanding}; }

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