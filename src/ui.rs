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
use std::any::Any;
use std::cell::{RefCell, Ref};

use low::message_handler::MessageHandler;
use args::PackUserValueArgs;
use error::Error;

/**
    Inner window data shared within the thread
*/
pub struct UiInner<ID: Hash+Clone+'static> {
    pub messages: MessageHandler<ID>,
    pub user_values: HashMap<u64, RefCell<Box<Any>>>,
    pub ids_map: HashMap<u64, ID>,
}

impl<ID: Hash+Clone> UiInner<ID> {

    pub fn new() -> Result<UiInner<ID>, Error> {
        let messages: MessageHandler<ID> = match MessageHandler::new() {
            Ok(msg) => msg,
            Err(e) => { return Err(e); }
        };

        Ok(UiInner{
            messages: messages,
            user_values: HashMap::with_capacity(16),
            ids_map: HashMap::with_capacity(64) })
    }

    pub fn pack_user_value(&mut self, params: PackUserValueArgs<ID>) -> Option<Error> {
        let inner_id = UiInner::hash_id(&params.id);
        if self.ids_map.contains_key(&inner_id) {
            Some(Error::KeyExists)
        } else {
            self.ids_map.insert(inner_id, params.id);
            self.user_values.insert(inner_id, RefCell::new(params.value));
            None
        }
    }

    fn hash_id(id: &ID) -> u64 {
        use std::hash::Hasher;
        use std::collections::hash_map::{DefaultHasher};

        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        s.finish()
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
pub struct Ui<ID: Hash+Clone+'static> {
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
        Add an user value to the Ui. 
        Asynchonous, this only registers the command in the ui message queue. 
        Either call `ui.commit` to execute it now or wait for the command to be executed in the main event loop.

        The executiong will fail if the id already exists in the Ui
    */
    pub fn pack_value<T: Into<Box<T>>+'static >(&mut self, id: ID, value: T) {
        use low::defs::{NWG_PACK_USER_VALUE};
        
        let inner = unsafe{ &mut (&*self.inner) };
        let data = PackUserValueArgs{ id: id, value: value.into() as Box<Any> };
        inner.messages.post(self.inner, NWG_PACK_USER_VALUE, Box::new(data) as Box<Any> );
    }

    /**
        Return the element identified by `id` in the Ui.
    */
    pub fn get<T: 'static>(&self, id: &ID) -> Result<Ref<Box<T>>, Error> {
        let inner = unsafe{ &mut (&*self.inner) };
        let inner_id = UiInner::hash_id(id);
        
        if let Some(v) = inner.user_values.get(&inner_id) {
            if let Ok(v_ref) = v.try_borrow() {
                if let None = v_ref.as_ref().downcast_ref::<T>() {
                    return Err(Error::Unimplemented);
                }

                use std::mem;
                let x: &RefCell<Box<T>> = unsafe{mem::transmute(v)};
                return Ok( x.borrow() );
            } else {
                return Err(Error::Unimplemented);
            }
        }

        Err(Error::KeyNotFound)
    }

    /**
        Check if an id exists in the ui

        * id -> The ID to check
    */
    pub fn has_id(&self, id: &ID) -> bool {
        let inner = unsafe{ &mut (&*self.inner) };
        let inner_id = UiInner::hash_id(id);
        inner.ids_map.contains_key(&inner_id)
    }

}

impl<ID: Hash+Clone> Drop for Ui<ID> {
    fn drop(&mut self) {
        unsafe{ drop(Box::from_raw(self.inner)); }
        self.inner = ptr::null_mut();
    }
}