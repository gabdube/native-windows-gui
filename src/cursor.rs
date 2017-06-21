/*!
    A global object that encapsulate the system cursor
*/
use std::hash::Hash;

use ui::Ui;
use error::Error;
use controls::AnyHandle;

pub struct Cursor;

impl Cursor {

    /**
        Return the cursor position in the screen.
    */
    pub fn get_position() -> (i32, i32) {
        use user32::GetCursorPos;
        use winapi::POINT;

        let mut p = POINT{x: 0, y: 0};
        unsafe{ GetCursorPos(&mut p); }

        (p.x as i32, p.y as i32)
    }

    /**
        Set the cursor position in the screen.

        Arguments:
        • `x`: The new x coordinates of the cursor
        • `y`: The new y coordinaets of the cursor
    */
    pub fn set_position(x: i32, y: i32) {
        use user32::SetCursorPos;
        use winapi::c_int;
        unsafe{ SetCursorPos(x as c_int, y as c_int); }
    }

    /**
        Set the cursor

        Arguments:
        • `ui`: The Ui holding the cursor resource
        • `cursor`: The id identifying the cursor resource
    */
    pub fn set<ID: Hash+Clone>(ui: &Ui<ID>, cursor: &ID) -> Result<(), Error> {
        use user32::SetCursor;

        match ui.handle_of(cursor) {
            Ok(AnyHandle::HCURSOR(h)) => unsafe{ SetCursor(h); Ok(()) },
            Ok(h) => Err(Error::BadResource(format!("Cursor resource required got {}", h.human_name()))),
            Err(e) => Err(e)
        }
    }

    /**
        Get the cursor identifier in a UI or `None` if it can't be matched

        Arguments:
        • `ui`: The Ui holding the cursor resource
    */
    pub fn get<ID: Hash+Clone>(ui: &Ui<ID>) -> Option<ID> {
        use user32::GetCursor;

        let cursor = unsafe{ GetCursor() };
        match ui.id_from_handle(&AnyHandle::HCURSOR(cursor)) {
            Ok(id) => Some(id),
            Err(_) => None
        }
    }

    /**
        Capture the mouse for a window-like control. Make sure to call `Cursor::release` to
        remove the capture. A control that has captured the mouse will receive mouse events
        even if the mouse is not hovering it.

        If the mouse is captured only to provide a "drag` gesture, yuse the `Cursor::drag` method instead.

        Arguments:
        • `ui`: The Ui holding the cursor resource
        • `control`: The control that will capture the mouse input
    */
    pub fn set_capture<ID: Hash+Clone>(ui: &Ui<ID>, control: &ID) -> Result<(), Error> {
        use user32::SetCapture;

        match ui.handle_of(control) {
            Ok(AnyHandle::HWND(h)) => unsafe{ SetCapture(h); Ok(()) },
            Ok(h) => Err(Error::BadResource(format!("Window-like control required got {}", h.human_name()))),
            Err(e) => Err(e)
        }
    }

    /**
        Get the current control that captured the cursor. Return `None` if no control captured the cursor.

        Arguments:
        • `ui`: The Ui holding the cursor resource
    */
    pub fn get_capture<ID: Hash+Clone>(ui: &Ui<ID>) -> Option<ID> {
        use user32::GetCapture;

        let cap = unsafe{ GetCapture() };
        match ui.id_from_handle(&AnyHandle::HWND(cap)) {
            Ok(id) => Some(id),
            Err(_) => None
        }
    }

    /**
        Captures the mouse and tracks its movement until the user releases the left button, presses the ESC key, or moves
        the mouse outside the drag rectangle around the specified point. 

        Return `Ok(true)` if the user did not execute the actions mentioned above or `Ok(false)` if it did.

        Arguments:
        • `ui`: The Ui holding the cursor resource
        • `control`: The control that will capture the mouse input
        • `point`: A point in screen coordinates where the dragging begins. If `None`, use the `Cursor::get_position` value.
    */
    pub fn dragging<ID: Hash+Clone>(ui: &Ui<ID>, control: &ID, point: Option<(i32, i32)>) -> Result<bool, Error> {
        use winapi::{POINT, LONG};
        use user32::DragDetect;

        let (x, y) = point.unwrap_or(Cursor::get_position());
        let c_point = POINT{x: x as LONG, y: y as LONG};

        match ui.handle_of(control) {
            Ok(AnyHandle::HWND(h)) => unsafe{ 
                Ok(DragDetect(h, c_point) == 1) 
            },
            Ok(h) => Err(Error::BadResource(format!("Window-like control required got {}", h.human_name()))),
            Err(e) => Err(e)
        }
    }

    /**
        Release the cursor if it was captured with `Cursor::capture`
    */
    pub fn release() {
        use user32::ReleaseCapture;
        unsafe{ ReleaseCapture(); }
    }

}