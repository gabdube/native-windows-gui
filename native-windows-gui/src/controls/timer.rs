#![allow(deprecated)]

use crate::controls::ControlHandle;
use crate::win32::{window_helper as wh, window::build_timer};
use crate::NwgError;
use std::cell::RefCell;

const NOT_BOUND: &'static str = "Timer is not yet bound to a winapi object";
const UNUSABLE_TIMER: &'static str = "Timer parent window was freed";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Timer handle is not Timer!";


/**
WARNING: Use AnimationTimer instead. The winapi timer does not have a constant tick and will call your single threaded from another thread.

A timer is an invisible UI component that trigger the `OnTimerTick` event at the specified interval.
Timers are mosty used to handle animations OR to create a timeout. To sync multithreaded action see the `Notice` object.

A timer still requires a top level window parent. If the top level window parent is destroyed, the timer becomes invalid.

Note that timer SHOULD NOT be used when a consistent interval is needed. The timer event might be triggered much faster
than the `interval` value. For example, when a user resize a window, Timer OnTimerTick gets triggered each time the window size changes.
This is a Windows "feature", there's probably nothing I can do to fix that.


Requires the `timer` feature. 

**Builder parameters:**
  * `parent`:   **Required.** The timer parent container. Should be a top level window
  * `interval`:  The timer tick interval in millisecond
  * `stopped`:   If the timer should start right away. By default timers starts "stopped(true)". Be sure to include `stopped(false)` in your builder if you want the timer to start instantly.

**Control events:**
  * `OnTimerTick`: When the timer ticks

```
use native_windows_gui as nwg;

fn build_timer(parent: &nwg::Window)  {
    let mut timer = Default::default();
    nwg::Timer::builder()
        .parent(parent)
        .interval(100)
        .stopped(false)
        .build(&mut timer);
}
```
*/
#[deprecated(
    since = "1.0.11",
    note = "Use AnimationTimer instead. The winapi timer does not have a constant tick and will call your single threaded from another thread."
)]
#[derive(Default)]
pub struct Timer {
    pub handle: ControlHandle,
    interval: RefCell<u32>,
}

impl Timer {

    pub fn builder() -> TimerBuilder {
        TimerBuilder {
            parent: None,
            interval: 100,
            stopped: true
        }
    }

    /// Checks if the timer is still usable. A timer becomes unusable when the parent window is destroyed.
    /// This will also return false if the timer is not initialized.
    pub fn valid(&self) -> bool {
        if self.handle.blank() { return false; }
        let (hwnd, _) = self.handle.timer().expect(BAD_HANDLE);
        wh::window_valid(hwnd)
    }

    /// Returns the interval of the timer, in milliseconds.
    pub fn interval(&self) -> u32 {
        *self.interval.borrow()
    }

    /// Sets the interval of the timer, in milliseconds.
    pub fn set_interval(&self, i: u32) {
        *self.interval.borrow_mut() = i;
    }

    /// Stops the timer.
    pub fn stop(&self) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        if !self.valid() { panic!("{}", UNUSABLE_TIMER); }
        let (hwnd, id) = self.handle.timer().expect(BAD_HANDLE);

        wh::kill_timer(hwnd, id);
    }

    /// Starts the timer. If the timer is already running, this restarts it.
    pub fn start(&self) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        if !self.valid() { panic!("{}", UNUSABLE_TIMER); }
        let (hwnd, id) = self.handle.timer().expect(BAD_HANDLE);

        wh::start_timer(hwnd, id, self.interval());
    }

}

impl Drop for Timer {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

pub struct TimerBuilder {
    parent: Option<ControlHandle>,
    interval: u32,
    stopped: bool
}

impl TimerBuilder {
    
    pub fn interval(mut self, interval: u32) -> TimerBuilder {
        self.interval = interval;
        self
    }

    pub fn stopped(mut self, stop: bool) -> TimerBuilder {
        self.stopped = stop;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TimerBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut Timer) -> Result<(), NwgError> {
        let parent = match self.parent {
            Some(p) => match p.hwnd() {
                Some(handle) => Ok(handle),
                None => Err(NwgError::control_create("Wrong parent type"))
            },
            None => Err(NwgError::no_parent("Timer"))
        }?;

        *out = Default::default();

        out.handle = unsafe { build_timer(parent, self.interval, self.stopped) };
        out.set_interval(self.interval);
        
        Ok(())
    }

}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
