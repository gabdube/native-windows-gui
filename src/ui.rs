/*!
    Public interface over the GUI
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
use std::ptr;

use low::message_handler::MessageHandler;
use error::Error;

/**
    Inner window data shared within the thread
*/
pub struct UiInner<ID: Hash+Clone> {
    messages: MessageHandler,
    tmp: Option<ID>
}

impl<ID: Hash+Clone> UiInner<ID> {

    pub fn new() -> Result<UiInner<ID>, Error> {
        let messages = match MessageHandler::new() {
            Ok(msg) => msg,
            Err(e) => { return Err(e); }
        };

        Ok(UiInner{ messages: messages, tmp: None })
    }

}

impl<ID: Hash+Clone> Drop for UiInner<ID> {

    fn drop(&mut self) {
        self.messages.free();
    }

}


/**
    Object that manage the GUI elements
*/
pub struct Ui<ID: Hash+Clone> {
    inner: *mut UiInner<ID>
}

impl<ID:Hash+Clone> Ui<ID> {

    pub fn new() -> Result<Ui<ID>, Error> {
        let inner = match UiInner::new() {
            Ok(inner) => Box::into_raw(Box::new(inner)),
            Err(e) => { return Err(e); }
        };

        Ok( Ui{inner: inner} )
    }

}

impl<ID: Hash+Clone> Drop for Ui<ID> {
    fn drop(&mut self) {
        unsafe{ drop(Box::from_raw(self.inner)); }
        self.inner = ptr::null_mut();
    }
}