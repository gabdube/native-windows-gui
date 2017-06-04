/*!
    A routine executed at a constant interval
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use std::hash::Hash;
use std::any::TypeId;
use std::time::Duration;

use winapi::{HWND, UINT_PTR, ULONG_PTR, UINT, DWORD};

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::{Event, Destroyed};
use events::timer::Tick;

static mut TIMERS_ID: UINT_PTR = 0; 

/**
    A template that creates a timer. Note that because the timer callbacks must be added AFTER
    its creation, it cannot start automatically.

    Events:  
    `Destroyed, Tick`   

    Members:  
    • `interval`: The timer interval in milliseconds
*/
#[derive(Clone)]
pub struct TimerT {
    pub interval: u32,
}

impl<ID: Hash+Clone> ControlT<ID> for TimerT {
    fn type_id(&self) -> TypeId { TypeId::of::<Timer>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, Tick]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        Ok(Box::new(Timer{
            watcher: unsafe{ ui.message_handle() },
            interval: self.interval,
            id_event: unsafe{ TIMERS_ID+=1; TIMERS_ID },
            handle: None,
            time: 0,
        }))
    }
}

/**
    A timer control
*/
pub struct Timer {
    time: u32,
    interval: u32,

    watcher: HWND,             // The watcher of a built-in timer is always its Ui. This way, the watcher cannot be freed before the timer.
    id_event: ULONG_PTR,       // A unique timer id to identify the time
    handle: Option<UINT_PTR>,  // If the timer is not running, handle is None.
}

impl Timer {

    /// Start the timer. If the timer was already running, it is restarted.
    pub fn start(&mut self) {
        use user32::SetTimer;
        use kernel32::GetTickCount;

        self.time = unsafe{GetTickCount()};
        self.handle = unsafe{ Some( SetTimer(self.watcher, self.id_event, self.interval, Some(timer_callback)) ) };
    }

    /// Stop the timer. If the timer was not started, this do nothing
    pub fn stop(&mut self) {
        use user32::KillTimer;
        if let Some(id) = self.handle.take() {
            unsafe{ KillTimer(self.watcher, id); }
            self.time = 0;
        }
    }

    /// Check if the timer is running. Return `true` if it is or `false` otherwise
    pub fn running(&self) -> bool {
        self.handle.is_some()
    }

    /// Return the time elapsed since the timer started. If the timer was never started or was stopped, the returned value is 0.
    pub fn elapsed(&self) -> Duration {
        use kernel32::GetTickCount;
        let elapsed = unsafe{ (GetTickCount() - self.time) as u64 };
        Duration::from_millis( elapsed )
    }

    /// Set the interval of the timer. If the timer is running, it will be applied when the timer is restarted
    pub fn set_interval(&mut self, interval: u32) { self.interval = interval; }

    /// Return the interval of the timer
    pub fn get_interval(&self) -> u32 { self.interval }

}

impl Control for Timer {

    fn handle(&self) -> AnyHandle {
        AnyHandle::Custom(TypeId::of::<Timer>(), self.id_event as usize)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Timer 
    }

    fn free(&mut self) {
        use user32::KillTimer;
        if let Some(id) = self.handle.take() {
            unsafe{ KillTimer(self.watcher, id); }
        }
    }

}

#[allow(unused_variables, non_snake_case)]
unsafe extern "system" fn timer_callback(hwnd: HWND, uMsg: UINT, idEvent: UINT_PTR, dwTime: DWORD) {
    use user32::SendMessageW;
    use winapi::{WM_TIMER, WPARAM};
    
    SendMessageW(hwnd, WM_TIMER, idEvent as WPARAM, 0);
}