/*!
A list-view control is a window that displays a collection of items.
List-view controls provide several ways to arrange and display items and are much more flexible than simple ListBox.
*/

use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use super::{ControlBase, ControlHandle};

const NOT_BOUND: &'static str = "ListView is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ListView handle is not HWND!";


bitflags! {
    pub struct ListViewFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}


/**
A list-view control is a window that displays a collection of items.
List-view controls provide several ways to arrange and display items and are much more flexible than simple ListBox.
*/
#[derive(Default, Debug)]
pub struct ListView {
    pub handle: ControlHandle
}


pub struct ListViewBuilder {

}
