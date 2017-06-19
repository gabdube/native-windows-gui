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
        • x: The new x coordinates of the cursor
        • y: The new y coordinaets of the cursor
    */
    pub fn set_position(x: i32, y: i32) {
        use user32::SetCursorPos;
        use winapi::c_int;
        unsafe{ SetCursorPos(x as c_int, y as c_int); }
    }

    /**
        Set the cursor

        Arguments:
        • ui: The Ui holding the cursor resource
        • cursor: The id identifying the cursor resource
    */
    pub fn set<ID: Hash+Clone>(ui: &Ui<ID>, cursor: &ID) -> Result<(), Error> {
        use user32::SetCursor;

        match ui.handle_of(cursor) {
            Ok(AnyHandle::HCURSOR(h)) => unsafe{ SetCursor(h); Ok(()) },
            Ok(h) => Err(Error::BadResource(format!("Cursor resource required got {}", h.human_name()))),
            Err(e) => Err(e)
        }
    }

}