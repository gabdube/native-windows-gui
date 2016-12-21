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

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::Event;

/**
    A template that creates a timer.

    Members:  
    • `interval`: The timer interval
    • `running`: If the timer should starts right away
    • `once`: If the timer should stop itself after ticking
*/
#[derive(Clone)]
pub struct TimerT {
    pub interval: Duration,
    pub running: bool,
    pub once: bool
}

impl<ID: Hash+Clone> ControlT<ID> for TimerT {
    fn type_id(&self) -> TypeId { TypeId::of::<Timer>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed, Event::Click]
    }

    #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        //if self.running & self.once
        Ok(Box::new(Timer))
    }
}

/**
    A timer control
*/
pub struct Timer;

impl Control for Timer {

    fn handle(&self) -> AnyHandle {
        AnyHandle::None
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Timer 
    }

    fn free(&mut self) {
        /* TODO */
    }

}