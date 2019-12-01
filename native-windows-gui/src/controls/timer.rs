use crate::controls::ControlHandle;
use crate::win32::window_helper as wh;

const NOT_BOUND: &'static str = "Timer is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Timer handle is not Timer!";

/// A timer is a invisible component that trigger a `OnTimerTick` event at the requested interval
/// Timers are mosty used to handle animations OR to create a timeout. To sync multithreaded action
/// see the `Notice` object.
#[derive(Default)]
pub struct Timer {
    pub handle: ControlHandle
}

impl Timer {

    /// Stop the timer. Does nothing if the timer is not running.
    pub fn stop(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let (hwnd, id) = self.handle.timer().expect(BAD_HANDLE);

        wh::kill_timer(hwnd, id);
    }

    /// Start the timer at `interval`. Restart the timer if it is already running.
    pub fn start(&self, interval: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let (hwnd, id) = self.handle.timer().expect(BAD_HANDLE);

        wh::start_timer(hwnd, id, interval);
    }

}
