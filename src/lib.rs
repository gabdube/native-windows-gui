/*!
    A very high level native gui library for Windows.
*/
#![allow(unused_variables)]

extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate comctl32;

pub mod controls;
pub mod events;
pub mod constants;
pub mod actions;

use std::ptr;
use std::mem;
use std::hash::Hash;
use std::collections::HashMap;

use controls::ControlTemplate;
use constants::Error;

use winapi::{MSG, HWND};
use user32::{GetMessageW, DispatchMessageW, TranslateMessage};

pub type ActionEvaluator<ID> = Box<Fn(&Ui<ID>, &ID, HWND, actions::Action<ID>) -> actions::ActionReturn<ID>>;
type ControlCollection<ID> = HashMap<ID, (HWND, ActionEvaluator<ID>) >;
type CallbackCollection<ID> = HashMap<events::Event, Vec<(ID, events::EventCallback<ID>)>>;

/**
    Structure stored in every window.
*/
struct WindowData<ID: Eq+Clone+Hash > {
    pub id: ID,
    pub _type: constants::ControlType,
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

        unsafe { ::controls::base::enable_visual_styles(); }

        Ui{controls: controls_raw}
    }

    /**
        Create a new control in the ui manager from the provided template
        and associate it with the ID provided by the user.

        If the control creation succeeded, return the id used by the user. 

        If the control creation somehow failed return `Err`
    */
    pub fn new_control<T:ControlTemplate<ID>>(&mut self, cont: ID, template: T) -> Result<ID, Error> {
        let controls: &mut ControlCollection<ID> = unsafe{ &mut *self.controls };
        
        if !controls.contains_key(&cont) {
            let handle: HWND;
            match template.create(self, cont.clone()) {
                Ok(h) => handle = h,
                Err(_) => { 
                    return Err(Error::TEMPLATE_CREATION);  // TODO propagate details about the error
                }
            }

            // Store the window data
            let mut callbacks = HashMap::new();
            for e in template.supported_events() {
                callbacks.insert(e, Vec::new());
            }

            let data = WindowData {
                id: cont.clone(),
                _type: template.control_type(),
                controls: self.controls,
                callbacks: callbacks,
            };
            controls::set_handle_data(handle, data);

            controls.insert(cont.clone(), (handle, template.evaluator()) );
            
            Ok(cont)
        } else {
            Err(Error::CONTROL_EXISTS) // Error: A widget with this id already exists
        }

    }

    /**
        Remove the control and ALL its children from the gui.
        If successful, return a vector of all deleted IDS.

        The control destruction is delayed.

        Return an error if the control was not found.
    */
    pub fn remove_control(&mut self, cont: ID) -> Result<Vec<ID>, Error> {
        let mut deleted_controls: Vec<ID>;
        let controls: &mut ControlCollection<ID> = unsafe{ &mut *self.controls };
        if let Some((handle, exec)) = controls.remove(&cont) {
            if let actions::ActionReturn::Children(c) = exec(self, &cont, handle, actions::Action::GetDescendants) {
                deleted_controls = *c;
            } else {
                deleted_controls = Vec::with_capacity(1); // Control can't have children
            }

            // Free the control and the control children's handle data.
            controls::free_handle::<ID>(handle);
        } else {
            return Err(Error::CONTROL_NOT_FOUND);
        }

        for id in deleted_controls.iter() {
            controls.remove(id).unwrap();
        }

        deleted_controls.push(cont);
        Ok(deleted_controls)
    }

    /**
        Add a callback to a control. Return Ok if the callback was added
        successfully, Err(reason) if `cont` was not found in ui.

        * cont : ID of the control to add the event to
        * name : Name of the event (used for unbinding)
        * cb   : The callback
    */
    pub fn bind(&self, cont: ID, name: ID, cb: events::EventCallback<ID>) -> Result<(), Error> {
        let controls: &mut ControlCollection<ID> = unsafe{ &mut *self.controls };
        if let Some(&(handle, _)) = controls.get(&cont) {
            let event = events::map_callback(&cb);
            let data: &mut WindowData<ID> = controls::get_handle_data(handle);
            if let Some(functions) = data.callbacks.get_mut(&event) {
                if functions.iter().any(|&(ref id, _)| hash(id) == hash(&name)) {
                    Err(Error::CALLBACK_ID_EXISTS) 
                } else {
                    functions.push((name, cb));
                    Ok(())
                }
            } else {
                Err(Error::CALLBACK_NOT_SUPPORTED) 
            }
        } else {
            Err(Error::CONTROL_NOT_FOUND) 
        }
    }

    /**
        Remove a callback from a control. Return Ok if the callback was added
        successfully, Err(reason) if `cont` or `name` were not found in ui.

        * cont  : ID of the control to remove the event from
        * name  : Name of the event to unbind
        * event : Event type to unbind
    */
    pub fn unbind(&self, cont: ID, name: ID, event: events::Event) -> Result<(), Error> {
        let controls: &mut ControlCollection<ID> = unsafe{ &mut *self.controls };
        if let Some(&(handle, _)) = controls.get(&cont) {
            let data: &mut WindowData<ID> = controls::get_handle_data(handle);
            if let Some(functions) = data.callbacks.get_mut(&event) {
                if let Some(pos) = functions.iter().position(|&(ref id, _)| hash(id) == hash(&name)) {
                    functions.remove(pos);
                    Ok(())
                } else {
                    Err(Error::CALLBACK_ID_NOT_FOUND)
                }
            } else {
                Err(Error::CALLBACK_NOT_SUPPORTED) 
            }
        } else {
            Err(Error::CONTROL_NOT_FOUND) 
        }
    }

    /**
        Execute an action on the specified control
    */
    pub fn exec(&self, cont: ID, action: actions::Action<ID>) -> Result<actions::ActionReturn<ID>, Error> {
        let controls: &mut ControlCollection<ID> = unsafe{ &mut *self.controls };
        if let Some(&(handle, ref exec)) = controls.get(&cont) {
            Ok(exec(self, &cont, handle, action))
        } else {
            Err(Error::CONTROL_NOT_FOUND)
        }
    }

}

impl<ID: Eq+Clone+Hash> Drop for Ui<ID> {
    fn drop(&mut self) {
        let controls: &ControlCollection<ID> = unsafe{ &mut *self.controls };
        for &(handle, _) in controls.values() {
            controls::free_handle_data::<WindowData<ID>>(handle);
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

/**
    Hash a single parameter. Used to compare events name, which do not use a hashmap.
*/
fn hash<T: Hash>(t: &T) -> u64 {
    use std::hash::{SipHasher, Hasher};
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}
