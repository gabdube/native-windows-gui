/// The ~cool~ (unsafe, not recommended, but works better) way to set a cursor in winapi that never flickers
pub fn set_cursor(handle: &nwg::ControlHandle, cursor: &nwg::Cursor) {
    use winapi::um::winuser::{SetClassLongPtrW, GCLP_HCURSOR};
    let handle = handle.hwnd().unwrap();
    unsafe {
        SetClassLongPtrW(
            handle,
            GCLP_HCURSOR,
            cursor.handle as _
        );
    }
}
