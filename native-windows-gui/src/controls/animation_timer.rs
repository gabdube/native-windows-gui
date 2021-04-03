use crate::controls::ControlHandle;
use crate::NwgError;
use crate::win32::window_helper as wh;
use std::{thread, time::{Duration, Instant}, sync::{Mutex, Arc}};

use winapi::um::winuser::SendNotifyMessageW;
use winapi::shared::minwindef::{WPARAM, LPARAM};
use winapi::shared::windef::HWND;

const NOT_BOUND: &'static str = "AnimationTimer is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: AnimationTimer handle is not Timer!";

lazy_static! {
    
    static ref THREAD_STATE: Arc<Mutex<AnimationThread>> = {
        let state = AnimationThread {
            timers: Vec::new(),
        };

        let state = Arc::new(Mutex::new(state));
        let shared_state = state.clone();
        
        thread::spawn(move || {
            let sleep_time = Duration::from_millis(1);

            loop {
                let mut state = shared_state.lock().unwrap();

                for (id, timer) in state.timers.iter_mut().enumerate() {
                    let timer = match timer.as_mut() {
                        Some(t) => match t.active {
                            true => t,
                            false => { continue; }
                        },
                        None => { continue; }
                    };

                    if timer.last_tick.elapsed() > timer.interval {
                        AnimationThread::timer_tick(id as u32, timer.hwnd);
                        timer.last_tick = Instant::now();
                        timer.current_tick += 1;

                        if Some(timer.current_tick) == timer.max_tick {
                            AnimationThread::timer_stop(id as u32, timer.hwnd);
                            timer.active = false;

                            // Must not trigger timer_stop twice due to birthtime
                            continue;
                        }
                    }

                    if let Some(lf) = timer.lifetime {
                        if timer.birthtime.elapsed() > lf {
                            AnimationThread::timer_stop(id as u32, timer.hwnd);
                            timer.active = false;
                        }
                    }
                }

                drop(state);
                thread::sleep(sleep_time);
            }
        });

        state
    };
}

#[derive(Copy, Clone)]
struct InnerTimer {
    interval: Duration,
    last_tick: Instant,
    lifetime: Option<Duration>,
    birthtime: Instant,
    max_tick: Option<u64>,
    current_tick: u64,
    active: bool,
    hwnd: usize,
}

struct AnimationThread {
    timers: Vec<Option<InnerTimer>>,
}

impl AnimationThread {

    fn add_timer(inner: InnerTimer) -> u32 {
        let mut state = THREAD_STATE.lock().unwrap();
        
        let empty = state.timers
            .iter_mut()
            .enumerate()
            .find(|(_i, t)| t.is_none());

        match empty {
            Some((i, t)) => {
                *t = Some(inner);
                i as u32
            },
            None => {
                state.timers.push(Some(inner));
                (state.timers.len() - 1) as u32
            }
        }
    }

    fn reset_timer(id: u32) {
        let mut state = THREAD_STATE.lock().unwrap();
        if let Some(Some(t)) = state.timers.get_mut(id as usize) {
            t.active = true;
            t.birthtime = Instant::now();
            t.current_tick = 0;
        }
    }

    fn update_timer(id: u32, interval: Option<Duration>, lifetime: Option<Option<Duration>>, max_tick: Option<Option<u64>>) {
        let mut state = THREAD_STATE.lock().unwrap();
        if let Some(Some(t)) = state.timers.get_mut(id as usize) {
            if let Some(v) = interval {
                t.interval = v;
            }

            if let Some(v) = lifetime {
                t.lifetime = v;
            }

            if let Some(v) = max_tick {
                t.max_tick = v;
            }
        }
    }

    fn stop_timer(id: u32) {
        let mut state = THREAD_STATE.lock().unwrap();
        if let Some(Some(t)) = state.timers.get_mut(id as usize) {
            t.active = false;
        }
    }

    fn remove_timer(id: u32) {
        let mut state = THREAD_STATE.lock().unwrap();
        if let Some(t) = state.timers.get_mut(id as usize) {
            *t = None;
        }
    }

    pub fn timer_tick(id: u32, hwnd: usize) {
        unsafe {
            SendNotifyMessageW(hwnd as HWND, wh::NWG_TIMER_TICK, id as WPARAM, hwnd as LPARAM);
        }
    }

    pub fn timer_stop(id: u32, hwnd: usize) {
        unsafe {
            SendNotifyMessageW(hwnd as HWND, wh::NWG_TIMER_STOP, id as WPARAM, hwnd as LPARAM);
        }
    }

}


/**
A timer is an invisible UI component that trigger the `OnTimerTick` event at the specified interval.
Timers are mosty used to handle animations OR to create a timeout. To sync multithreaded action see the `Notice` object.

AnimationTimer is controlled from a singletion running in another thread. All instance of AnimationTimer will live on that thread.

A timer still requires a top level window parent. If the top level window parent is destroyed, the timer becomes invalid.

AnimationTimer replaces the default winapi timer. Please, for the love of god, do not use the default timer.

**Builder parameters:**
    * `parent`:     **Required.** The timer parent container that will receive the timer event. Should be a top level window
    * `interval`:   The timer tick interval as a rust Duration. Minimum is 1 ms
    * `lifetime`:   The timer should automatically stop after the selected Duration. Defaults to `None`.
    * `max_tick`:   The timer should automatically stop after sending X amount of OnTImerTick events. Defaults to `None`.
    * `active`:     If the timer should start right away. Default to `false`

**Control events:**
    * `OnTimerTick`: When the timer ticks
    * `OnTimerStop`: When the timer stops itself (due to max_tick_count or lifetime being reached, not user actions)

```
use native_windows_gui as nwg;
use std::time::Duration;

/// Builds a timer that will animation something at 60fps for 3 sec
fn build_timer(parent: &nwg::Window)  {
    let mut timer = Default::default();
    nwg::AnimationTimer::builder()
        .parent(parent)
        .interval(Duration::from_millis(1000/60))
        .lifetime(Some(Duration::from_millis(3000)))
        .build(&mut timer);
}
```
*/
#[derive(Default, PartialEq, Eq)]
pub struct AnimationTimer {
    pub handle: ControlHandle,
}

impl AnimationTimer {

    pub fn builder() -> AnimationTimerBuilder {
        AnimationTimerBuilder {
            parent: None,
            interval: Duration::from_millis(1000/60),
            max_tick: None,
            lifetime: None,
            active: false,
        }
    }

    /// Checks if the timer is still usable. A timer becomes unusable when the parent window is destroyed.
    /// This will also return false if the timer is not initialized.
    pub fn valid(&self) -> bool {
        if self.handle.blank() { return false; }
        let (hwnd, _) = self.handle.timer().expect(BAD_HANDLE);
        wh::window_valid(hwnd)
    }

    /**
        Start the selected timer. If the timer is already running this resets it.
        This resets the life time and tick count if relevant.
    */
    pub fn start(&self) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let (_, id) = self.handle.timer().expect(BAD_HANDLE);
        AnimationThread::reset_timer(id);
    }

    /**
        Stop the selected timer. If the timer is already stopped, this does nothing.
    */
    pub fn stop(&self) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let (_, id) = self.handle.timer().expect(BAD_HANDLE);
        AnimationThread::stop_timer(id);
    }

    /// Sets the interval on the this timer
    pub fn set_interval(&self, i: Duration) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let (_, id) = self.handle.timer().expect(BAD_HANDLE);
        AnimationThread::update_timer(id, Some(i), None, None);
    }

    /// Sets the life time on the this timer
    pub fn set_lifetime(&self, life: Option<Duration>) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let (_, id) = self.handle.timer().expect(BAD_HANDLE);
        AnimationThread::update_timer(id, None, Some(life), None);
    }

    /// Sets the max tick count on the this timer
    pub fn set_max_tick(&self, max_tick: Option<u64>) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let (_, id) = self.handle.timer().expect(BAD_HANDLE);
        AnimationThread::update_timer(id, None, None, Some(max_tick));
    }

}

impl Drop for AnimationTimer {

    fn drop(&mut self) {
        match &self.handle {
            ControlHandle::Timer(_, id) => {
                AnimationThread::remove_timer(*id);
            },
            _ => {}
        }
    }

}

pub struct AnimationTimerBuilder {
    parent: Option<ControlHandle>,
    interval: Duration,
    max_tick: Option<u64>,
    lifetime: Option<Duration>,
    active: bool
}

impl AnimationTimerBuilder {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> AnimationTimerBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn interval(mut self, interval: Duration) -> AnimationTimerBuilder {
        self.interval = interval;
        self
    }

    pub fn max_tick(mut self, max_tick: Option<u64>) -> AnimationTimerBuilder {
        self.max_tick = max_tick;
        self
    }

    pub fn lifetime(mut self, lifetime: Option<Duration>) -> AnimationTimerBuilder {
        self.lifetime = lifetime;
        self
    }

    pub fn active(mut self, active: bool) -> AnimationTimerBuilder {
        self.active = active;
        self
    }

    pub fn build(self, out: &mut AnimationTimer) -> Result<(), NwgError> {
        let parent = match self.parent {
            Some(p) => match p.hwnd() {
                Some(handle) => Ok(handle),
                None => Err(NwgError::control_create("Wrong parent type"))
            },
            None => Err(NwgError::no_parent("Timer"))
        }?;

        if self.interval < Duration::from_millis(1) {
            return Err(NwgError::control_create("Timer interval cannot be smaller than 1 ms"));
        }

        let inner = InnerTimer {
            interval: self.interval,
            last_tick: Instant::now(),
            lifetime: self.lifetime,
            birthtime: Instant::now(),
            max_tick: self.max_tick,
            current_tick: 0,
            active: self.active,
            hwnd: parent as usize,
        };

        let id = AnimationThread::add_timer(inner);

        *out = AnimationTimer {
            handle: ControlHandle::Timer(parent, id)
        };

        Ok(())
    }

}
