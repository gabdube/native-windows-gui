use winapi::shared::windef::HWND;
use winapi::um::winuser::{GetSystemMetrics, MonitorFromWindow, GetMonitorInfoW, MONITORINFO,
    SM_CXSCREEN, SM_CYSCREEN, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, MONITOR_DEFAULTTONEAREST};
use crate::ControlHandle;
use std::mem;

/**
    Expose basic properties of the monitor(s) on the system and the virtual screen.

    This object cannot be instanced. The methods should be used this way:

    ```rust
    // Creating and centering a window in the main monitor

    use native_windows_gui as nwg;

    fn create_window(width: i32, height: i32) -> nwg::Window {
        let [total_width, total_height] = [nwg::Monitor::width(), nwg::Monitor::height()];
        let mut window = nwg::Window::default();

        let x = (total_width-width)/2;
        let y = (total_height-height)/2;

        nwg::Window::builder()
            .size((width, height))
            .position((x, y))
            .build(&mut window)
            .unwrap();

        window
    }
    
    ```
*/
pub struct Monitor;

impl Monitor {

    fn monitor_info_from_window(handle: HWND) -> MONITORINFO {
        unsafe {
            let m = MonitorFromWindow(handle, MONITOR_DEFAULTTONEAREST);

            let mut info: MONITORINFO = mem::zeroed();
            info.cbSize = mem::size_of::<MONITORINFO>() as _;
            GetMonitorInfoW(m, &mut info);

            info
        }
    }

    /// Returns the width in pixel of the monitor that has the largest area of intersection with the bounding rectangle of a specified window
    /// If the window does not intersect any display monitor, returns the nearest monitor width
    /// Panics if `window` is not a window like control.
    pub fn width_from_window<H: Into<ControlHandle>>(window: H) -> i32 {
        let handle = window.into().hwnd().expect("Window to be a window-like control");
        let info = Self::monitor_info_from_window(handle);
        (info.rcMonitor.right - info.rcMonitor.left) as _
    }

    /// Returns the height in pixel of the monitor that has the largest area of intersection with the bounding rectangle of a specified window
    /// If the window does not intersect any display monitor, returns the nearest monitor height
    /// Panics if `window` is not a window like control.
    pub fn height_from_window<H: Into<ControlHandle>>(window: H) -> i32 {
        let handle = window.into().hwnd().expect("Window to be a window-like control");
        let info = Self::monitor_info_from_window(handle);
        (info.rcMonitor.bottom - info.rcMonitor.top) as _
    }

    /// Returns a [left, top, right, bottom] rectangle that specifies the display monitor rectangle, expressed in virtual-screen coordinates. 
    /// Note that if the monitor is not the primary display monitor, some of the rectangle's coordinates may be negative values.
    /// Panics if `window` is not a window like control.
    pub fn monitor_rect_from_window<H: Into<ControlHandle>>(window: H) -> [i32; 4] {
        let handle = window.into().hwnd().expect("Window to be a window-like control");
        let info = Self::monitor_info_from_window(handle);
        let m = info.rcMonitor;

        [
            m.left,
            m.top,
            m.right,
            m.bottom
        ]
    }

    /// Returns the primary monitor width in pixel
    /// Use `Monitor::virtual_width` to get the dimensions of the virtual screen
    pub fn width() -> i32 {
        unsafe {
            GetSystemMetrics(SM_CXSCREEN) as _
        }
    }

    /// Returns the primary monitor height in pixel
    /// Use `Monitor::virtual_height` to get the dimensions of the virtual screen
    pub fn height() -> i32 {
        unsafe {
            GetSystemMetrics(SM_CYSCREEN) as _
        }
    }

    /// Returns the primary monitor width in pixel
    /// Use `Monitor::width` to get the dimensions of the virtual screen
    pub fn virtual_width() -> i32 {
        unsafe {
            GetSystemMetrics(SM_CXVIRTUALSCREEN) as _
        }
    }

    /// Returns the primary monitor height in pixel
    /// Use `Monitor::height` to get the dimensions of the virtual screen
    pub fn virtual_height() -> i32 {
        unsafe {
            GetSystemMetrics(SM_CYVIRTUALSCREEN) as _
        }
    }

}
