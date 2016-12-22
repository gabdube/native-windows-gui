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

use winapi::{HWND, UINT_PTR, UINT, DWORD};

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::Event;

/**
    A template that creates a timer. Note that because the timer callbacks must be added AFTER
    its creation, it cannot start automatically.

    Members:  
    • `interval`: The timer interval in miliseconds
    • `repeat`: If the timer should automatically restart after the interval
*/
#[derive(Clone)]
pub struct TimerT {
    pub interval: u32,
    pub repeat: bool
}

impl<ID: Hash+Clone> ControlT<ID> for TimerT {
    fn type_id(&self) -> TypeId { TypeId::of::<Timer>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed, Event::Tick]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        Ok(Box::new(Timer{
            watcher: unsafe{ ui.message_handle() },
            interval: self.interval,
            id: None,
            //repeat: self.repeat
        }))
    }
}

/**
    A timer control
*/
pub struct Timer {
    pub watcher: HWND,         // The watcher of a built-in timer is always its Ui. This way, the watcher cannot be freed before the timer.
    pub id: Option<UINT_PTR>,  // If the timer is not running, id is None.
    pub interval: u32,
    //pub repeat: bool
}

impl Timer {

    /**
        Start the timer
    */
    pub fn start(&mut self) {
        use user32::SetTimer;
        self.id = unsafe{ Some( SetTimer(self.watcher, 0, self.interval, Some(timer_callback)) ) };
    }

    /**
        Stop the timer
    */
    pub fn stop(&mut self) {
        use user32::KillTimer;
        if let Some(id) = self.id.take() {
            unsafe{ KillTimer(self.watcher, id); }
        }
    }

    /**
        Check if the timer is running. Return `true` if it is or `false` otherwise
    */
    pub fn running(&self) -> bool {
        self.id.is_some()
    }

}

impl Control for Timer {

    fn handle(&self) -> AnyHandle {
        AnyHandle::None
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Timer 
    }

    fn free(&mut self) {
        use user32::KillTimer;
        if let Some(id) = self.id.take() {
            unsafe{ KillTimer(self.watcher, id); }
        }
    }

}

#[allow(unused_variables, non_snake_case)]
unsafe extern "system" fn timer_callback(hwnd: HWND, uMsg: UINT, idEvent: UINT_PTR, dwTime: DWORD) {

}