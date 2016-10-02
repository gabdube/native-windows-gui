/*!
    Callback definitions
*/

use std::hash::Hash;

pub type Ef0<ID> = Box<Fn(&mut ::Ui<ID>, &ID)>;
pub type Ef1<ID, A> = Box<Fn(&mut ::Ui<ID>, &ID, A)>;
pub type Ef4<ID, A, B, C, D> = Box<Fn(&mut ::Ui<ID>, &ID, A, B, C, D)>;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Event {
    MouseUp,
    MouseDown,
    Click,
    Focus,
    ValueChanged,
    MaxValue,
    Removed,
    Unknown,
    Last
}

pub enum EventCallback<ID: Eq+Hash+Clone> {
    MouseUp(Ef4<ID, i32, i32, u32, u32>),
    MouseDown(Ef4<ID, i32, i32, u32, u32>),
    Click(Ef0<ID>),
    Focus(Ef1<ID, bool>),
    Removed(Ef0<ID>),
    ValueChanged(Ef0<ID>),
    MaxValue(Ef0<ID>)
}

/**
    Map callbacks to an application event
*/
pub fn map_callback<ID: Eq+Hash+Clone>(cb: &EventCallback<ID>) -> Event {
    match cb {
        &EventCallback::MouseUp(_) => Event::MouseUp,
        &EventCallback::MouseDown(_) => Event::MouseDown,
        &EventCallback::Click(_) => Event::Click,
        &EventCallback::Focus(_) => Event::Focus,
        &EventCallback::ValueChanged(_) => Event::ValueChanged,
        &EventCallback::MaxValue(_) => Event::MaxValue,
        &EventCallback::Removed(_) => Event::Removed,
    }
}