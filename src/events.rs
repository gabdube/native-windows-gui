/*!
    Callback definitions
*/

use std::hash::Hash;
use winapi::{UINT};

pub type Eventfunc0<ID> = Box<Fn(::Ui<ID>, &ID)>;

pub enum Event {
    MouseClick,
    ButtonClick,
    Unknown,
    Last
}

pub enum EventCallback<ID: Eq+Hash+Clone> {
    MouseClick(Eventfunc0<ID>),
    ButtonClick(Eventfunc0<ID>)
}

/**
    Map callbacks to an application event
*/
pub fn map_callback<ID: Eq+Hash+Clone>(cb: &EventCallback<ID>) -> Event {
    match cb {
        &EventCallback::MouseClick(_) => Event::MouseClick,
        &EventCallback::ButtonClick(_) => Event::ButtonClick
    }
}

/**
    Map system events to application events
*/
pub fn map_system_event(evt: UINT) -> Event {
    match evt {
        _ => Event::Unknown
    }
}