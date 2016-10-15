/*!
    Callback definitions
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

pub type Ef0<ID> = Box<Fn(&mut ::Ui<ID>, &ID)>;
pub type Ef1<ID, A> = Box<Fn(&mut ::Ui<ID>, &ID, A)>;
pub type Ef2<ID, A, B> = Box<Fn(&mut ::Ui<ID>, &ID, A, B)>;
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
    MenuOpen,
    MenuClose,
    SelectionChanged,
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
    MenuOpen(Ef0<ID>),
    MenuClose(Ef0<ID>),
    MaxValue(Ef0<ID>),
    SelectionChanged(Ef2<ID, u32, String>),
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
        &EventCallback::MenuOpen(_) => Event::MenuOpen,
        &EventCallback::MenuClose(_) => Event::MenuClose,
        &EventCallback::SelectionChanged(_) => Event::SelectionChanged,
    }
}