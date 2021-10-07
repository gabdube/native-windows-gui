use crate::Cursor;
use crate::controls::ControlHandle;
use crate::win32::high_dpi;

/**
    A global object that wraps the system cursor.
    Requires the `cursor` feature.


    This object cannot be instanced. The methods should be used this way:

    ```rust
    use native_windows_gui as nwg;
    let (x,y) = nwg::GlobalCursor::position();
    ```
*/
pub struct GlobalCursor;

impl GlobalCursor {

    /**
        Return the cursor position in the screen.
    */
    pub fn position() -> (i32, i32) {
        use winapi::um::winuser::GetCursorPos;
        use winapi::shared::windef::POINT;

        let mut p = POINT{x: 0, y: 0};
        unsafe { GetCursorPos(&mut p); }

        (p.x as i32, p.y as i32)
    }

    /**
        Return or map the cursor position relatively to a control.
        If point is `None`, `Cursor::position` is used.
    */
    pub fn local_position<C: Into<ControlHandle>>(control: C, point: Option<(i32, i32)>) -> (i32, i32) {
        use winapi::shared::ntdef::LONG;
        use winapi::shared::windef::POINT;
        use winapi::um::winuser::ScreenToClient;

        const MSG: &'static str = "local_position can only be used for window control";

        let control = control.into();
        if control.blank() { panic!("{}", MSG); }
        let handle = control.hwnd().expect(MSG);

        let (x, y) = point.unwrap_or(GlobalCursor::position());
        let mut p = POINT{x: x as LONG, y: y as LONG};

        unsafe { ScreenToClient(handle, &mut p); }

        (p.x as i32, p.y as i32)
    }

    /**
        Return or map the cursor position relatively to a control and convert to logical.
        If point is `None`, `Cursor::position` is used.
    */
    pub fn local_logical_position<C: Into<ControlHandle>>(control: C, point: Option<(i32, i32)>) -> (i32, i32) {
        use winapi::shared::ntdef::LONG;
        use winapi::shared::windef::POINT;
        use winapi::um::winuser::ScreenToClient;

        const MSG: &'static str = "local_position can only be used for window control";

        let control = control.into();
        if control.blank() { panic!("{}", MSG); }
        let handle = control.hwnd().expect(MSG);

        let (x, y) = point.unwrap_or(GlobalCursor::position());
        let mut p = POINT{x: x as LONG, y: y as LONG};

        unsafe {
            ScreenToClient(handle, &mut p);

            high_dpi::physical_to_logical(p.x as i32, p.y as i32)
        }
    }

    /**
        Set the cursor position in the screen.

        Arguments:
        • `x`: The new x coordinates of the cursor
        • `y`: The new y coordinaets of the cursor
    */
    pub fn set_position(x: i32, y: i32) {
        use winapi::um::winuser::SetCursorPos;
        use winapi::ctypes::c_int;

        unsafe { SetCursorPos(x as c_int, y as c_int); }
    }

    /**
        Set the cursor image.
        If your application must set the cursor while it is in a window, make sure the class cursor
        for the specified window's class is set to NULL. If the class cursor is not NULL,
        the system restores the class cursor each time the mouse is moved.

        Arguments:
        • `cursor`: The id identifying the cursor resource
    */
    pub fn set(cursor: &Cursor) {
        use winapi::shared::windef::HCURSOR;
        use winapi::um::winuser::SetCursor;

        unsafe { SetCursor(cursor.handle as HCURSOR); }
    }

    /**
        Get the cursor image

        Returns `None` if there is no cursor.
    */
    pub fn get() -> Option<Cursor> {
        use winapi::um::winuser::GetCursor;
        use winapi::um::winnt::HANDLE;

        let cursor = unsafe { GetCursor() };

        match cursor.is_null() {
            true => None,
            false => Some( Cursor { handle: cursor as HANDLE, owned: false } )
        }
    }

    /**
        Capture the mouse for a window-like control. Make sure to call `Cursor::release` to
        remove the capture. A control that has captured the mouse will receive mouse events
        even if the mouse is not hovering it.

        Will panic if the control handle passed to the method is not a window or if the control is not yet initialized.

        Arguments:
        • `control`: The control that will capture the mouse input
    */
    pub fn set_capture(control: &ControlHandle) {
        use winapi::um::winuser::SetCapture;
        const MSG: &'static str = "Mouse capture can only be set for window control";

        if control.blank() { panic!("{}", MSG); }
        let handle = control.hwnd().expect(MSG);

        unsafe { SetCapture(handle); }
    }

    /**
        Release the cursor if it was captured with `Cursor::set_capture`
    */
    pub fn release() {
        use winapi::um::winuser::ReleaseCapture;
        unsafe{ ReleaseCapture(); }
    }

    /**
        Return the handle of the control that has captured the mouse. Return `None` if no control captured the cursor.

        Arguments:
        • `ui`: The Ui holding the cursor resource
    */
    pub fn capture() -> Option<ControlHandle> {
        use winapi::um::winuser::GetCapture;

        let cap = unsafe{ GetCapture() };
        match cap.is_null() {
            true => None,
            false => Some(ControlHandle::Hwnd(cap))
        }
    }

    /**
    Captures the mouse and tracks its movement until the user releases the left button, presses the ESC key, or moves
    the mouse outside the drag rectangle around the specified point.

    Return `Ok(true)` if the user did not execute the actions mentioned above or `Ok(false)` if it did.

    Will panic if the control handle passed to the method is not a window or if the control is not yet initialized.

    Arguments:
    • `control`: The control that will capture the mouse input
    • `point`: A point in screen coordinates where the dragging begins. If `None`, use the `Cursor::position` value.

    ```rust
        use native_windows_gui as nwg;
        fn dragging(c: &nwg::Window) {
            if nwg::GlobalCursor::dragging(&c.handle, None) {
                println!("DRAGGING!")
            }
        }
    ```
    */
    pub fn dragging(control: &ControlHandle, point: Option<(i32, i32)>) -> bool {
        use winapi::shared::ntdef::LONG;
        use winapi::shared::windef::POINT;
        use winapi::um::winuser::DragDetect;

        const MSG: &'static str = "Dragging can only be set for window control";

        if control.blank() { panic!("{}", MSG); }
        let handle = control.hwnd().expect(MSG);

        let (x, y) = point.unwrap_or(GlobalCursor::position());
        let c_point = POINT{x: x as LONG, y: y as LONG};

        unsafe { DragDetect(handle, c_point) == 1 }
    }

}
