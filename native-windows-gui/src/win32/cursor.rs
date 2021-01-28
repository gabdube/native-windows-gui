use crate::Cursor;
use crate::controls::ControlHandle;
use std::mem;


/**
    Tracking information returned by [track_mouse_query](GlobalCursor::track_mouse_query).
    Note that this returns the raw handle of the tracked control.
*/
pub struct TrackCursorInfo {
    /// Raw handle of the tracked control
    pub handle: ControlHandle,

    /// If hover is being tracked
    pub hover: bool,

    /// If leaving is being tracked
    pub leaving: bool,

    /// Time in ms for the tracking
    pub hover_time: u32
}


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
        if control.blank() { panic!(MSG); }
        let handle = control.hwnd().expect(MSG);

        let (x, y) = point.unwrap_or(GlobalCursor::position());
        let mut p = POINT{x: x as LONG, y: y as LONG};

        unsafe { ScreenToClient(handle, &mut p); }

        (p.x as i32, p.y as i32)
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

        if control.blank() { panic!(MSG); }
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
        use nwg::{Event::*, MousePressEvent::*};

        // Sample code that capture the cursor if the user tries to drag a control
        fn on_mouse_press(frame: &nwg::Frame, event: nwg::Event) {
            match event {
                OnMousePress(MousePressLeftDown) => {
                    if nwg::GlobalCursor::dragging(&frame.handle, None) {
                        nwg::GlobalCursor::set_capture(&frame.handle);
                    }
                },
                OnMousePress(MousePressLeftUp) => {
                    nwg::GlobalCursor::release();
                },
                _ => unreachable!()
            }
        }
    ```
    */
    pub fn dragging(control: &ControlHandle, point: Option<(i32, i32)>) -> bool {
        use winapi::shared::ntdef::LONG;
        use winapi::shared::windef::POINT;
        use winapi::um::winuser::DragDetect;

        const MSG: &'static str = "Dragging can only be set for window control";

        if control.blank() { panic!(MSG); }
        let handle = control.hwnd().expect(MSG);

        let (x, y) = point.unwrap_or(GlobalCursor::position());
        let c_point = POINT{x: x as LONG, y: y as LONG};

        unsafe { DragDetect(handle, c_point) == 1 }
    }

    /**
        Tells winapi to track when the cursor will leave the control. 

        Panics if control is not a window-like control
    */
    pub fn track_mouse_leaving<C: Into<ControlHandle>>(control: C) {
        use winapi::um::winuser::{TrackMouseEvent, TRACKMOUSEEVENT, TME_LEAVE};

        let hwnd = control.into().hwnd().expect("Control to be a window like");

        unsafe {
            let mut p = TRACKMOUSEEVENT {
                cbSize: mem::size_of::<TRACKMOUSEEVENT>() as _,
                dwFlags: TME_LEAVE,
                hwndTrack: hwnd,
                dwHoverTime: 0
            };

            TrackMouseEvent(&mut p);
        }
    }

    /**
        Tells winapi to notice the control when the cursor hovers it for a period of time.
        If `hover_time_ms` is None, use the system default waiting time (should be 400 milliseconds).

        Panics if control is not a window-like control
    */
    pub fn track_mouse_hover<C: Into<ControlHandle>>(control: C, hover_time_ms: Option<u32>) {
        use winapi::um::winuser::{TrackMouseEvent, TRACKMOUSEEVENT, TME_HOVER, HOVER_DEFAULT};

        let hwnd = control.into().hwnd().expect("Control to be a window like");
        let hover = hover_time_ms.unwrap_or(HOVER_DEFAULT);

        unsafe {
            let mut p = TRACKMOUSEEVENT {
                cbSize: mem::size_of::<TRACKMOUSEEVENT>() as _,
                dwFlags: TME_HOVER,
                hwndTrack: hwnd,
                dwHoverTime: hover
            };

            TrackMouseEvent(&mut p);
        }
    }

    /**
        Cancel the tracking of the cursor previously set by `track_mouse`. 
        Use `leaving` and `hover` parameters to specify which tracking to cancel
        
        Panics if control is not a window-like control
    */
    pub fn track_mouse_cancel<C: Into<ControlHandle>>(control: C, leaving: bool, hover: bool) {
        use winapi::um::winuser::{TrackMouseEvent, TRACKMOUSEEVENT, TME_CANCEL, TME_LEAVE, TME_HOVER};

        let hwnd = control.into().hwnd().expect("Control to be a window like");
        let mut cancel = 0;

        if !leaving && !hover {
            return;
        }

        if leaving { cancel |= TME_LEAVE; }
        if hover { cancel |= TME_HOVER; }
        
        unsafe {
            let mut p = TRACKMOUSEEVENT {
                cbSize: mem::size_of::<TRACKMOUSEEVENT>() as _,
                dwFlags: TME_CANCEL | cancel,
                hwndTrack: hwnd,
                dwHoverTime: 0
            };

            TrackMouseEvent(&mut p);
        }
    }

    /**
        Returns the information about which control is currently being tracked by the `track_mouse_*` functions.
    */
    pub fn track_mouse_query(&self) -> TrackCursorInfo {
        use winapi::um::winuser::{TrackMouseEvent, TRACKMOUSEEVENT, TME_QUERY, TME_LEAVE, TME_HOVER};
        use std::ptr;

        let mut p = TRACKMOUSEEVENT {
            cbSize: mem::size_of::<TRACKMOUSEEVENT>() as _,
            dwFlags: TME_QUERY,
            hwndTrack: ptr::null_mut(),
            dwHoverTime: 0
        };

        unsafe {
            TrackMouseEvent(&mut p);
        }

        TrackCursorInfo {
            handle: ControlHandle::Hwnd(p.hwndTrack),
            leaving: p.dwFlags & TME_LEAVE == TME_LEAVE,
            hover: p.dwFlags & TME_HOVER == TME_HOVER,
            hover_time: p.dwHoverTime
        }
    }

}
