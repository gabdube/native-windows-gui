/*!
    The progress bar control definition
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
use std::mem;

use winapi::HWND;
use user32::SendMessageW;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::{Event, Destroyed, Moved, Resized};
use defs::ProgressBarState;

/**
    A template that creates a progress bar

    Events:  
    `Destroyed, Moved, Resized, Any`  

    Members:  
    • `position`: The start position of the progressbar  
    • `size`: The start size of the progressbar  
    • `visible`: If the progressbar should be visible to the user  
    • `disabled`: If the user can or can't click on the progressbar  
    • `range`: The minimum and the maximum value of the progress bar  
    • `position`: The starting position of the progress bar  
    • `step`: Amount of value to add when `step` is called  
    • `state`: The state of the progress bar.
    • `vertical`: If the progress bar should be vertical instead of horizontal
    • `parent`: The progressbar parent  
*/
#[derive(Clone)]
pub struct ProgressBarT<ID: Hash+Clone> {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub range: (u32, u32),
    pub value: u32,
    pub step: u32,
    pub state: ProgressBarState,
    pub vertical: bool,
    pub parent: ID,
}

impl<ID: Hash+Clone> ControlT<ID> for ProgressBarT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<ProgressBar>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, Moved, Resized, Event::Any]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, handle_of_window};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, PBS_VERTICAL};

        if self.range.1 <= self.range.0 {
            let msg = "The progress bar range maximum value must be greater than the minimum value";
            return Err(Error::UserError(msg.to_string()));
        }

        let flags: DWORD = WS_CHILD | 
        if self.visible  { WS_VISIBLE }   else { 0 } |
        if self.disabled { WS_DISABLED }  else { 0 } |
        if self.vertical { PBS_VERTICAL } else { 0 } ;

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a progress bar must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        let params = WindowParams {
            title: "",
            class_name: "msctls_progress32",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ 
                    set_range(h, self.range.0, self.range.1);
                    set_step(h, self.step); 
                    set_value(h, self.value);
                    set_state(h, &self.state);
                }
                Ok( Box::new(ProgressBar{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A standard progress bar
*/
pub struct ProgressBar {
    handle: HWND
}

impl ProgressBar {

    /// Return the current range of the progress bar
    pub fn get_range(&self) -> (u32, u32) {
        use winapi::{PBM_GETRANGE, PBRANGE};
        let mut range = PBRANGE{iLow: 0, iHigh: 0};
        unsafe{ SendMessageW(self.handle, PBM_GETRANGE, 0, mem::transmute(&mut range) ); }

        (range.iLow as u32, range.iHigh as u32)
    }

    /**
        Set the extent of the progress bar control. If `min` is bigger than `max`, an error
        will be returned.
    */
    pub fn set_range(&self, min: u32, max: u32) -> Result<(), Error> {
        if max <= min {
            let msg = "The progress bar range maximum value must be greater than the minimum value";
            return Err(Error::UserError(msg.to_string()));
        }
        unsafe{ set_range(self.handle, min, max); }
        Ok(())
    }

    /// Return the current step of the progress bar
    pub fn get_step(&self) -> u32 {
        use winapi::PBM_GETSTEP;
        unsafe{ SendMessageW(self.handle, PBM_GETSTEP, 0, 0) as u32 }
    }

    /**
        Set the step of the progress bar control. When `step()` is called, the progress bar value
        is increased by the value defined by step.
    */
    pub fn set_step(&self, step: u32) {
        unsafe{ set_step(self.handle, step); }
    }

    /// Return the current value of the progress bar
    pub fn get_value(&self) -> u32 {
        use winapi::PBM_GETPOS;
        unsafe{ SendMessageW(self.handle, PBM_GETPOS, 0, 0) as u32 }
    }

    /// Set the progress bar value
    pub fn set_value(&self, val: u32) {
        unsafe{ set_value(self.handle, val); }
    }

    /// Get the progress bar state
    pub fn get_state(&self) -> ProgressBarState {
        use winapi::{PBM_GETSTATE, PBST_ERROR, PBST_PAUSED};
        match unsafe{ SendMessageW(self.handle, PBM_GETSTATE, 0, 0) as i32 } {
            PBST_ERROR => ProgressBarState::Error,
            PBST_PAUSED => ProgressBarState::Paused,
            _ => ProgressBarState::Normal
        }
    }

    /// Set the progress bar state
    pub fn set_state(&self, state: ProgressBarState) {
        unsafe{ set_state(self.handle, &state); }
    }

    /* 
        Add the step value to the progress bar value.
        Once the value reach the maximum value of the progress bar, the value is reverted back to the minimum value.
    */
    pub fn step(&self) {
        use winapi::PBM_STEPIT;
        unsafe{ SendMessageW(self.handle, PBM_STEPIT, 0, 0); }
    }

    /**
        Add `amount` to the progress bar value
    */
    pub fn advance(&self, amount: u32) {
        use winapi::{PBM_DELTAPOS, WPARAM};
        unsafe{ SendMessageW(self.handle, PBM_DELTAPOS, amount as WPARAM, 0); }
    }

    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, false); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
}

impl Control for ProgressBar {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::ProgressBar 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}

// Private functions

#[inline(always)]
unsafe fn set_range(handle: HWND, min: u32, max: u32) {
    use winapi::{PBM_SETRANGE32, WPARAM, LPARAM};
    SendMessageW(handle, PBM_SETRANGE32, min as WPARAM, max as LPARAM);
}

#[inline(always)]
unsafe fn set_step(handle: HWND, step: u32) {
    use winapi::{PBM_SETSTEP, WPARAM};
    SendMessageW(handle, PBM_SETSTEP, step as WPARAM, 0);
}

#[inline(always)]
unsafe fn set_value(handle: HWND, val: u32) {
    use winapi::{PBM_SETPOS, WPARAM};
    SendMessageW(handle, PBM_SETPOS, val as WPARAM, 0);
}

#[inline(always)]
unsafe fn set_state(handle: HWND, state: &ProgressBarState) {
    use winapi::{PBM_SETSTATE, WPARAM, PBST_NORMAL, PBST_ERROR, PBST_PAUSED};
    let state = match state {
        &ProgressBarState::Normal => PBST_NORMAL,
        &ProgressBarState::Paused => PBST_PAUSED,
        &ProgressBarState::Error => PBST_ERROR,
    };

    SendMessageW(handle, PBM_SETSTATE, state as WPARAM, 0);
}