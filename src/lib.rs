/*!
    A very high level native gui library for Windows.
*/

extern crate winapi;
extern crate user32;
extern crate kernel32;

pub mod controls;
pub mod events;
pub mod constants;

use std::ptr;
use std::mem;
use std::hash::Hash;
use std::collections::HashMap;
use std::iter::FromIterator;
use controls::ControlTemplate;
use winapi::{MSG, HWND};
use user32::{GetMessageW, DispatchMessageW, TranslateMessage};

type ControlCollection<ID> = HashMap<ID, HWND>;
type CallbackCollection<ID> = Vec<Vec<events::EventCallback<ID>>>;

/**
    Structure stored in every window.
*/
struct WindowData<ID: Eq+Clone+Hash > {
    pub id: ID,
    pub controls: *mut ControlCollection<ID>,
    pub callbacks: CallbackCollection<ID>
}

/**
    A single threaded window manager.
*/
pub struct Ui<ID: Eq+Clone+Hash > {
    controls: *mut ControlCollection<ID>
}

impl<ID: Eq+Clone+Hash> Ui<ID> {

    /**
        Creates a new `Ui` that will manage the interface on the 
        current thread.
    */
    pub fn new() -> Ui<ID> {
        let controls = ControlCollection::<ID>::new();
        let controls_raw = Box::into_raw(Box::new(controls));
        Ui{controls: controls_raw}
    }

    /**
        Create a new control in the ui manager from the provided template
        and associate it with the ID provided by the user.

        If the control creation succeeded, return the id used by the user. 

        If the control creation somehow failed return `Err`
    */
    pub fn new_control<T:ControlTemplate<ID>>(&mut self, cont: ID, template: T) -> Result<ID, ()> {
        let controls: &mut ControlCollection<ID> = unsafe{ &mut *self.controls };
        
        if !controls.contains_key(&cont) {
            let handle: HWND;
            match template.create(self, cont.clone()) {
                Ok(h) => handle = h,
                Err(_) => { 
                    return Err(());  // Error: Template creation failed: *template error* 
                }
            }

            // Store the window data
            let event_range = 0..events::Event::Last as usize;
            let callbacks = Vec::from_iter(event_range.map(|_|
                Vec::new()
            ));

            let data = WindowData{
                id: cont.clone(),
                controls: self.controls,
                callbacks: callbacks
            };
            controls::set_handle_data(handle, data);

            controls.insert(cont.clone(), handle);
            Ok(cont)
        } else {
            Err(()) // Error: A widget with this id already exists
        }

    }

    /**
        Add a callback to a control. Return `true` if the callback was added
        successfully, `false` if `cont` was not found in ui.
    */
    pub fn bind(&self, cont: ID, cb: events::EventCallback<ID>) -> bool {
        let controls: &mut ControlCollection<ID> = unsafe{ &mut *self.controls };
        if let Some(handle) = controls.get(&cont) {
            let data: &mut WindowData<ID> = controls::get_handle_data(*handle);
            let index = events::map_callback(&cb) as usize;
            data.callbacks[index].push(cb);
            true
        } else {
            false
        }
    }

}

impl<ID: Eq+Clone+Hash> Drop for Ui<ID> {
    fn drop(&mut self) {
        let controls: &ControlCollection<ID> = unsafe{ &mut *self.controls };
        for handle in controls.values() {
            controls::free_handle_data::<WindowData<ID>>(*handle);
        }

        unsafe { Box::from_raw(self.controls); }
        controls::cleanup();
    }
}

/**
    Wait for system events and dispatch them
*/
pub fn dispatch_events() { unsafe{ 
    let mut msg: MSG = mem::uninitialized();
    while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
        TranslateMessage(&msg); 
        DispatchMessageW(&msg); 
    }
}}

