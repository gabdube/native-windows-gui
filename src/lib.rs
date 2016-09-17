/*!
    A very high level native gui library for Windows.
*/

extern crate winapi;

use std::hash::Hash;
use std::collections::HashMap;
use winapi::HWND;

type ControlCollection<ID> = HashMap<ID, HWND>;

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

}