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
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};

use low::message_handler::MessageHandler;
use args::{PackUserValueArgs, PackControlArgs, AnyHandle};
use controls::{ControlT, Control};
use error::Error;

/**
    Inner window data shared within the thread
*/
pub struct UiInner<ID: Hash+Clone+'static> {
    pub messages: MessageHandler<ID>,
    pub controls: HashMap<u64, RefCell<Box<Control>>>,
    pub user_values: HashMap<u64, RefCell<Box<Any>>>,
    pub ids_map: HashMap<u64, ID>,
}

impl<ID: Hash+Clone> UiInner<ID> {

    pub fn new() -> Result<UiInner<ID>, Error> {
        use low::other_helper::enable_visual_styles;
        let messages: MessageHandler<ID> = match MessageHandler::new() {
            Ok(msg) => msg,
            Err(e) => { return Err(e); }
        };

        unsafe{ enable_visual_styles(); }

        Ok(UiInner{
            messages: messages,
            user_values: HashMap::with_capacity(16),
            controls: HashMap::with_capacity(32),
            ids_map: HashMap::with_capacity(64) })
    }

    pub fn pack_user_value(&mut self, params: PackUserValueArgs<ID>) -> Option<Error> {
        let (inner_id, inner_type_id) = UiInner::hash_id(&params.id, &params.tid);
        if self.ids_map.contains_key(&inner_id) {
            Some(Error::KeyExists)
        } else {
            self.ids_map.insert(inner_id, params.id);
            self.user_values.insert(inner_type_id, RefCell::new(params.value));
            None
        }
    }

    pub fn pack_control(&mut self, params: PackControlArgs<ID>) -> Option<Error> {
        use low::events::hook_window_events;
            
        let (inner_id, inner_type_id) = UiInner::hash_id(&params.id, &params.value.type_id());
        if self.ids_map.contains_key(&inner_id) {
            Some(Error::KeyExists)
        } else {
            match params.value.build() {
                Ok(control) => {

                    // Hook the window events if the handle is a HWND
                    match control.handle() {
                        AnyHandle::HWND(h) => hook_window_events(self, inner_id, inner_type_id, h)
                    }

                    self.ids_map.insert(inner_id, params.id);
                    self.controls.insert(inner_type_id, RefCell::new(control) );
                    None
                },
                Err(e) => Some(e)
            }
        }
    }

    pub fn unpack_control(&mut self, id: u64, tid: u64) -> Option<Error> {
        // TODO call destroyed callback
        // TODO destroy children

        // Test if everything is valid
        if let Some(e) = {
            if !self.ids_map.contains_key(&id) {
                Some(Error::KeyNotFound)
            } else {
                if let Some(c) = self.controls.get(&tid) {
                    if let Err(e) = c.try_borrow_mut() { Some(Error::BorrowError) } 
                    else { None }
                } else {
                    Some(Error::BadType)
                }
            }
        } { return Some(e); }

        self.ids_map.remove(&id);
        let control = self.controls.remove(&tid).unwrap();
        let mut control = control.into_inner();
        control.free();

        None
    }

    fn hash_id(id: &ID, tid: &TypeId) -> (u64, u64) {
        use std::hash::Hasher;
        use std::collections::hash_map::{DefaultHasher};

        let mut s1 = DefaultHasher::new();
        let mut s2 = DefaultHasher::new();

        id.hash(&mut s2);
        tid.hash(&mut s2);

        id.hash(&mut s1);

        (s1.finish(), s2.finish())
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
        let data = PackUserValueArgs{ id: id, tid: TypeId::of::<T>(), value: value.into() as Box<Any>};
        inner.messages.post(self.inner, NWG_PACK_USER_VALUE, Box::new(data) as Box<Any> );
    }

    /**
        Add a control to the Ui.
        Asynchonous, this only registers the command in the ui message queue. 
        Either call `ui.commit` to execute it now or wait for the command to be executed in the main event loop.

        The executiong will fail if the id already exists in the Ui or if the template creation fails.
    */
    pub fn pack_control<T: ControlT+'static>(&mut self, id: ID, value: T) {
        use low::defs::{NWG_PACK_CONTROL};

        let inner = unsafe{ &mut (&*self.inner) };
        let data = PackControlArgs{ id: id, value: Box::new(value)};
        inner.messages.post(self.inner, NWG_PACK_CONTROL, Box::new(data) as Box<Any> );
    }

    /**
        Return an immutable reference to the element identified by `id` in the Ui.
        It is required to give a type `T` to this function as it is needed to cast the underlying value.
        Ex: `ui.get::<u32>(100)`

        Params:  
        * id: The id that identify the element in the ui

        * Error::KeyNotFound will be returned if the key was not found in the Ui
        * Error::BadType will be returned if the key exists, but the type do not match
        * Error::BorrowError will be returned if the element was already borrowed mutably

    */
    pub fn get<T: 'static>(&self, id: &ID) -> Result<Ref<Box<T>>, Error> {
        use std::mem;
         
        let inner = unsafe{ &mut (&*self.inner) };
        let (inner_id, inner_type_id) = UiInner::hash_id(id, &TypeId::of::<T>());

        if !inner.ids_map.contains_key(&inner_id) { return Err(Error::KeyNotFound); }
        
        if let Some(v) = inner.user_values.get(&inner_type_id) {
            let v_casted: &RefCell<Box<T>> = unsafe{mem::transmute(v)};
            if let Ok(v_ref) = v_casted.try_borrow() {
                return Ok( v_ref );
            } else {
                return Err(Error::BorrowError);
            }
        }

        if let Some(v) = inner.controls.get(&inner_type_id) {
            let v_casted: &RefCell<Box<T>> = unsafe{mem::transmute(v)};
            if let Ok(v_ref) = v_casted.try_borrow() {
                return Ok( v_ref );
            } else {
                return Err(Error::BorrowError);
            }
        }

        return Err(Error::BadType);
    }

    /**
        Return an mutable referemce to element identified by `id` in the Ui.
        It is required to give a type `T` to this function as it is needed to cast the underlying value.
        Ex: `ui.get::<u32>(100)`

        Params:  
        * id: The id that identify the element in the ui

        * Error::KeyNotFound will be returned if the key was not found in the Ui
        * Error::BadType will be returned if the key exists, but the type do not match
        * Error::BorrowError will be returned if the element was already borrowed mutably
    */
    pub fn get_mut<T: 'static>(&self, id: &ID) -> Result<RefMut<Box<T>>, Error> {
        use std::mem;
         
        let inner = unsafe{ &mut (&*self.inner) };
        let (inner_id, inner_type_id) = UiInner::hash_id(id, &TypeId::of::<T>());

        if !inner.ids_map.contains_key(&inner_id) { return Err(Error::KeyNotFound); }
        
        if let Some(v) = inner.user_values.get(&inner_type_id) {
            let v_casted: &RefCell<Box<T>> = unsafe{mem::transmute(v)};
            if let Ok(v_ref) = v_casted.try_borrow_mut() {
                return Ok( v_ref );
            } else {
                return Err(Error::BorrowError);
            }
        }

        if let Some(v) = inner.controls.get(&inner_type_id) {
            let v_casted: &RefCell<Box<T>> = unsafe{mem::transmute(v)};
            if let Ok(v_ref) = v_casted.try_borrow_mut() {
                return Ok( v_ref );
            } else {
                return Err(Error::BorrowError);
            }
        }

        return Err(Error::BadType);
    }

    /**
        Check if an id exists in the ui

        * id -> The ID to check
    */
    pub fn has_id(&self, id: &ID) -> bool {
        let inner = unsafe{ &mut (&*self.inner) };
        let (inner_id, _) = UiInner::hash_id(id, &TypeId::of::<()>());
        inner.ids_map.contains_key(&inner_id)
    }

}

impl<ID: Hash+Clone> Drop for Ui<ID> {
    fn drop(&mut self) {
        unsafe{ drop(Box::from_raw(self.inner)); }
        self.inner = ptr::null_mut();
    }
}


/**
    Dispatch the messages waiting the the system message queue to the associated Uis. This includes NWG custom messages.

    Return once a quit event was received.
*/
pub fn dispatch_events() {
    // Actual code is located under the low module because that's where most of the unsafe code should be
    unsafe{ ::low::events::dispatch_events(); }
}