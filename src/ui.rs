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
use std::collections::HashMap;

use low::message_handler::MessageHandler;
use error::Error;

/**
    Inner window data shared within the thread
*/
pub struct UiInner<ID: Hash+Clone> {
    pub messages: MessageHandler<ID>,
    pub ids_map: HashMap<u64, ID>,
}

impl<ID: Hash+Clone> UiInner<ID> {

    pub fn new() -> Result<UiInner<ID>, Error> {
        let messages: MessageHandler<ID> = match MessageHandler::new() {
            Ok(msg) => msg,
            Err(e) => { return Err(e); }
        };

        Ok(UiInner{ messages: messages, ids_map: HashMap::with_capacity(64) })
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

    /**
        Create a new Ui.

        * Returns `Ok(ui)` if the initialization was successful
        * Returns `Err(Error::System)` if the system could not initialize the ui
    */
    pub fn new() -> Result<Ui<ID>, Error> {
        let inner = match UiInner::new() {
            Ok(inner) => Box::into_raw(Box::new(inner)),
            Err(e) => { return Err(e); }
        };

        Ok( Ui{inner: inner} )
    }

    /**
        Execute the NWG commands waiting in the Ui command queue in the order they
        where added.

        * Returns `Ok(())` if everything was executed without Errors
        * Returns `Err(Error)` if an error was encountered while executing the commands.
          As soon as an error is found, the function will return. If there are still commands
          waiting in the queue, they wont be touched.
    */
    pub fn commit(&mut self) -> Result<(), Error> {
        unsafe{ (&mut *self.inner).messages.commit() }
    }

    /**
        Add an element to the Ui. 
        Asynchonous, this only registers the command in the ui message queue. Call `ui.commit` to execute it.
    */
    pub fn pack(&mut self) {
        use low::defs::{NWG_PACK_USER_VALUE};
        
        let inner = unsafe{ &mut (&*self.inner) };
        inner.messages.post(self.inner, NWG_PACK_USER_VALUE)
    }

}

impl<ID: Hash+Clone> Drop for Ui<ID> {
    fn drop(&mut self) {
        unsafe{ drop(Box::from_raw(self.inner)); }
        self.inner = ptr::null_mut();
    }
}