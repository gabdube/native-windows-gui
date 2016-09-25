/*!
    Callback definitions
*/

use std::hash::Hash;

pub type Ef0<ID> = Box<Fn(&mut ::Ui<ID>, &ID)>;
pub type Ef1<ID, A> = Box<Fn(&mut ::Ui<ID>, &ID, A)>;
pub type Ef4<ID, A, B, C, D> = Box<Fn(&mut ::Ui<ID>, &ID, A, B, C, D)>;

#[derive(Hash, PartialEq, Eq)]
pub enum Event {
    MouseUp,
    ButtonClick,
    Focus,
    Unknown,
    Last
}

pub enum EventCallback<ID: Eq+Hash+Clone> {
    MouseUp(Ef4<ID, i32, i32, u32, u32>),
    ButtonClick(Ef0<ID>),
    Focus(Ef1<ID, bool>)
}

/**
    Map callbacks to an application event
*/
pub fn map_callback<ID: Eq+Hash+Clone>(cb: &EventCallback<ID>) -> Event {
    match cb {
        &EventCallback::MouseUp(_) => Event::MouseUp,
        &EventCallback::ButtonClick(_) => Event::ButtonClick,
        &EventCallback::Focus(_) => Event::Focus,
    }
}