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
use std::rc::Rc;

use low::message_handler::MessageHandler;
use low::defs::{PackUserValueArgs, PackControlArgs, PackResourceArgs, UnpackArgs, BindArgs, UnbindArgs};
use controls::{ControlT, Control, AnyHandle};
use resources::{ResourceT, Resource};
use events::{Event, EventCallback, EventArgs};
use error::Error;

pub type BoxedCallback<ID> = Box<EventCallback<ID>>;
pub type CallbackCollection<ID> = Rc<Vec<(u64, BoxedCallback<ID>)>>;
pub type EventCollection<ID> = HashMap<Event, CallbackCollection<ID>>;

/**
    Inner window data shared within the thread
*/
pub struct UiInner<ID: Hash+Clone+'static> {
    pub messages: MessageHandler<ID>,
    pub controls: HashMap<u64, RefCell<Box<Control>>>,
    pub control_events: HashMap<u64, EventCollection<ID>>,
    pub resources: HashMap<u64, RefCell<Box<Resource>>>,
    pub user_values: HashMap<u64, RefCell<Box<Any>>>,
    pub ids_map: HashMap<u64, (ID, u64)>,
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
            control_events: HashMap::with_capacity(32),
            resources: HashMap::with_capacity(16),
            ids_map: HashMap::with_capacity(64) })
    }

    pub fn pack_user_value(&mut self, params: PackUserValueArgs<ID>) -> Option<Error> {
        let (inner_id, inner_type_id) = UiInner::hash_id(&params.id, &params.tid);
        if self.ids_map.contains_key(&inner_id) {
            Some(Error::KeyExists)
        } else {
            self.ids_map.insert(inner_id, (params.id, inner_type_id) );
            self.user_values.insert(inner_type_id, RefCell::new(params.value));
            None
        }
    }

    pub fn pack_control(&mut self, params: PackControlArgs<ID>) -> Option<Error> {
        use low::events::hook_window_events;
        use low::menu_helper::{init_menu_data, init_menu_item_data};
            
        let (inner_id, inner_type_id) = UiInner::hash_id(&params.id, &params.value.type_id());
        if self.ids_map.contains_key(&inner_id) {
            Some(Error::KeyExists)
        } else {
            let tmp_ui: Ui<ID> = Ui{inner: self as *mut UiInner<ID>};
            match params.value.build(&tmp_ui) {
                Ok(control) => {
                    
                    match control.handle() {
                        AnyHandle::HWND(h) => hook_window_events(self, inner_id, h), // Hook the window events if the handle is a HWND
                        AnyHandle::HMENU(h) => init_menu_data(h, inner_id),          // Save the id in the menu
                        AnyHandle::HMENU_ITEM(parent_h, uid) => init_menu_item_data(parent_h, uid, inner_id),
                        AnyHandle::HFONT(_) => {/* Nothing to initialize for resources */}
                    }

                    // Init events
                    let events = params.value.events();
                    let mut event_collection: EventCollection<ID> = HashMap::with_capacity(events.len());
                    for e in events {
                        event_collection.insert(e, Rc::new(Vec::new()));
                    }

                    self.ids_map.insert(inner_id, (params.id, inner_type_id));
                    self.controls.insert(inner_type_id, RefCell::new(control) );
                    self.control_events.insert(inner_type_id, event_collection);

                    ::std::mem::forget(tmp_ui);

                    None
                },
                Err(e) => { ::std::mem::forget(tmp_ui); Some(e)}
            }
        }
    }

    pub fn pack_resource(&mut self, params: PackResourceArgs<ID>) -> Option<Error> {
        let (inner_id, inner_type_id) = UiInner::hash_id(&params.id, &params.value.type_id());
        if self.ids_map.contains_key(&inner_id) {
            Some(Error::KeyExists)
        } else {
            let tmp_ui: Ui<ID> = Ui{inner: self as *mut UiInner<ID>};
            match params.value.build(&tmp_ui) {
                Ok(resource) => {
                    self.ids_map.insert(inner_id, (params.id, inner_type_id));
                    self.resources.insert(inner_type_id, RefCell::new(resource) );
                    ::std::mem::forget(tmp_ui);
                    None
                },
                Err(e) => { ::std::mem::forget(tmp_ui); Some(e)}
            }
        }
    }

    fn unpack_control(&mut self, id: u64, tid: u64) -> Option<Error> {
        use low::events::unhook_window_events;
        use low::menu_helper::{list_menu_children, free_menu_data, free_menu_item_data};
        use low::window_helper::list_window_children;
       

        // Check if the control is currently borrowed by the user
        if let Err(_) = self.controls.get(&tid).unwrap().try_borrow_mut() { 
            return Some(Error::ControlInUse);
        }

        // Check if one of the control events is currently being executed
        {
            let events_collection = self.control_events.get_mut(&tid).unwrap();
            for mut ec in events_collection.values_mut() {
                if Rc::get_mut(&mut ec).is_none() {
                    return Some(Error::ControlInUse);
                }
            }
        }

        // Unpack the children
        let handle = self.handle_of(id);
        if handle.is_err() { return Some(handle.err().unwrap()); }

        let children_ids: Vec<u64> = match handle.unwrap() {
            AnyHandle::HMENU(h) => unsafe {
                let mut children = vec![id];
                children.append( &mut list_menu_children(h) );
                children
            },
            AnyHandle::HWND(h) => unsafe { 
                let mut children = vec![id];
                children.append( &mut list_window_children(h, self as *mut UiInner<ID>) );
                children
            },
            AnyHandle::HMENU_ITEM(_, _) | AnyHandle::HFONT(_) => vec![id], // menu items / resources can't have children
        };
       
        for id in children_ids.iter().rev() {

            // Call the destroy callbacks
            self.trigger(*id, Event::Destroyed, EventArgs::None);

            // Removes stuffs
            let (_, tid) = self.ids_map.remove(&id).unwrap();
            self.control_events.remove(&tid).unwrap();
            let control = self.controls.remove(&tid).unwrap();
            let mut control = control.into_inner();

            // Unhook the events dispatcher if its a window
            match control.handle() {
                AnyHandle::HWND(h) => unhook_window_events::<ID>(h),
                AnyHandle::HMENU(h) => free_menu_data(h),
                AnyHandle::HMENU_ITEM(parent_h, uid) => { free_menu_item_data(parent_h, uid) },
                AnyHandle::HFONT(_) => {/* Nothing to free in resources */}
            };
            
            // Free the control custom resources
            control.free();
        }

        // Control gets dropped here
        None
    }

    fn unpack_resource(&mut self, id: u64, tid: u64) -> Option<Error> {
        // Check if the resource is currently borrowed by the user
        if let Err(_) = self.resources.get(&tid).unwrap().try_borrow_mut() { 
            return Some(Error::ResourceInUse);
        }

         // Removes stuffs
        self.ids_map.remove(&id).unwrap();
        let resource = self.resources.remove(&tid).unwrap();
        let mut resource = resource.into_inner();
        
        // Free the control custom resources
        resource.free();

        None
    }

    fn unpack_user_value(&mut self, id: u64, tid: u64) -> Option<Error> {
        if let Err(_) = self.user_values.get(&tid).unwrap().try_borrow_mut() { 
            return Some(Error::ControlInUse);
        }

        self.ids_map.remove(&id);
        let value = self.user_values.remove(&tid).unwrap();
        value.into_inner();
    
        None
    }

    pub fn unpack(&mut self, params: UnpackArgs) -> Option<Error> {
        let id = params.id;

        // Test if everything is valid
        if !self.ids_map.contains_key(&id) {
            Some(Error::KeyNotFound)
        } else {
            let tid = self.ids_map.get(&id).unwrap().1;
            if self.controls.contains_key(&tid) {
                self.unpack_control(id, tid)
            } else if self.user_values.contains_key(&tid) {
                self.unpack_user_value(id, tid)
            } else if self.resources.contains_key(&tid) {
                self.unpack_resource(id, tid)
            } else {
                Some(Error::BadType)
            }
        }
    }

    pub fn bind(&mut self, params: BindArgs<ID>) -> Option<Error> {
        let (id, cb_id, event, cb) = (params.id, params.cb_id, params.event, params.cb);

        let tid = match self.ids_map.get(&id) {
            Some(&(_, tid)) => tid,
            None => { return Some(Error::KeyNotFound); }
        };

        // Get the event collection of the control
        let events_collection = self.control_events.get_mut(&tid);
        if events_collection.is_none() { return Some(Error::ControlRequired); }

        // Get the callback list for the requested event
        let callbacks = events_collection.unwrap().get_mut(&event);
        if callbacks.is_none() { return Some(Error::EventNotSupported(event)); }

        // Get a mutable reference to the callback list
        let mut callbacks = callbacks.unwrap();
        let callbacks = Rc::get_mut(&mut callbacks);
        if callbacks.is_none() { return Some(Error::ControlInUse); }

        // Check if the cb id already exists for the event and if not, push the callback
        let callbacks = callbacks.unwrap();
        if let Some(_) = callbacks.iter().find(|&&(cb_id2, _)| cb_id2 == cb_id) {
            Some(Error::KeyExists)
        } else {
            callbacks.push((cb_id, cb)); 
            None
        }
    }

    pub fn unbind(&mut self, params: UnbindArgs) -> Option<Error> {
        let (id, cb_id, event) = (params.id, params.cb_id, params.event);

        let tid = match self.ids_map.get(&id) {
            Some(&(_, tid)) => tid,
            None => { return Some(Error::KeyNotFound); }
        };

        // Get the event collection of the control
        let events_collection = self.control_events.get_mut(&tid);
        if events_collection.is_none() { return Some(Error::ControlRequired); }

        // Get the callback list for the requested event
        let callbacks = events_collection.unwrap().get_mut(&event);
        if callbacks.is_none() { return Some(Error::EventNotSupported(event)); }

        // Get a mutable reference to the callback list
        let mut callbacks = callbacks.unwrap();
        let callbacks = Rc::get_mut(&mut callbacks);
        if callbacks.is_none() { return Some(Error::ControlInUse); }

        // Check if the cb id exists for the event and if it is, remove the callback
        let callbacks = callbacks.unwrap();
        if let Some(index) = callbacks.iter().position(|&(cb_id2, _)| cb_id2 == cb_id) {
            callbacks.remove(index);
            None
        } else {
            Some(Error::KeyNotFound)
        }
    }

    pub fn trigger(&mut self, id: u64, event: Event, args: EventArgs) -> Option<Error> {
        let (pub_id, tid) = match self.ids_map.get_mut(&id) {
            Some(&mut (ref pub_id, tid)) => (pub_id.clone(), tid),
            None => { return Some(Error::KeyNotFound); }
        };

        let callback_list = {
            // Get the event collection of the control
            let events_collection = self.control_events.get_mut(&tid);
            if events_collection.is_none() { return Some(Error::ControlRequired); }

            // The the callback list for the requested event
            let callbacks = events_collection.unwrap().get_mut(&event);
            if callbacks.is_none() { return Some(Error::EventNotSupported(event)); }

            // Return a reference to the callback list. While the reference exists, it will be impossible
            // to push new callback into the event.
            callbacks.unwrap().clone()
        };

        let tmp_ui: Ui<ID> = Ui{inner: self as *mut UiInner<ID>};
        for &( _, ref callback) in callback_list.iter() {
            (callback)(&tmp_ui, &pub_id, &event, &args);
        }

        ::std::mem::forget(tmp_ui);
        None
    }

    pub fn handle_of(&self, id: u64) -> Result<AnyHandle, Error> {
        let tid = if let Some(v) = self.ids_map.get(&id) {
            v.1
        } else {
            return Err(Error::KeyNotFound);
        };
        
        if let Some(v) = self.controls.get(&tid) {
            if let Ok(v_ref) = v.try_borrow() {
                return Ok( v_ref.handle() );
            } else {
                return Err(Error::BorrowError);
            }
        }

        Err(Error::ControlOrResourceRequired)
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
        let controls_ids: Vec<u64> = self.ids_map.keys().map(|k| *k).collect();
        for id in controls_ids {
            self.unpack(UnpackArgs{id: id});
        }

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

        Returns `Ok(ui)` if the initialization was successful  
        Returns `Err(Error::System)` if the system could not initialize the ui
    */
    pub fn new() -> Result<Ui<ID>, Error> {
        let inner = match UiInner::new() {
            Ok(inner) => Box::into_raw(Box::new(inner)),
            Err(e) => { return Err(e); }
        };

        Ok( Ui{inner: inner} )
    }

    /**
        Execute the NWG commands waiting in the Ui command queue in the order they where added.

        Returns `Ok(())` if everything was executed without Errors  
        Returns `Err(Error)` if an error was encountered while executing the commands.  
        As soon as an error is found, the function will return. If there are still commands  
        waiting in the queue, they wont be touched.
    */
    pub fn commit(&self) -> Result<(), Error> {
        unsafe{ (&mut *self.inner).messages.commit() }
    }

    /**
        Add an user value to the Ui.  
        Asynchronous, this only registers the command in the ui message queue.  
        Either call `ui.commit` to execute it now or wait for the command to be executed in the main event loop.  

        Commit returns  
        • `Error::KeyExist` if the key already exists in the ui
    */
    pub fn pack_value<T: Into<Box<T>>+'static >(&self, id: &ID, value: T) {
        use low::defs::{NWG_PACK_USER_VALUE};
        
        let inner = unsafe{ &mut *self.inner };
        let data = PackUserValueArgs{ id: id.clone(), tid: TypeId::of::<T>(), value: value.into() as Box<Any>};
        inner.messages.post(self.inner, NWG_PACK_USER_VALUE, Box::new(data) as Box<Any> );
    }

    /**
        Add a control to the Ui.  
        Asynchronous, this only registers the command in the ui message queue.  
        Either call `ui.commit` to execute it now or wait for the command to be executed in the main event loop.

        Commit returns
          • `Error::KeyExist` if the key already exists in the ui  
          • `Error::{Any}` if the template creation fails
    */
    pub fn pack_control<T: ControlT<ID>+'static>(&self, id: &ID, value: T) {
        use low::defs::{NWG_PACK_CONTROL};

        let inner = unsafe{ &mut *self.inner };
        let data = PackControlArgs{ id: id.clone(), value: Box::new(value)};
        inner.messages.post(self.inner, NWG_PACK_CONTROL, Box::new(data) as Box<Any> );
    }

    /**
        Add a resource to the Ui.  
        Asynchronous, this only registers the command in the ui message queue.  
        Either call `ui.commit` to execute it now or wait for the command to be executed in the main event loop.

        Commit returns
          • `Error::KeyExist` if the key already exists in the ui  
          • `Error::{Any}` if the template creation fails
    */
    pub fn pack_resource<T: ResourceT<ID>+'static>(&self, id: &ID, value: T) {
        use low::defs::{NWG_PACK_RESOURCE};

        let inner = unsafe{ &mut *self.inner };
        let data = PackResourceArgs{ id: id.clone(), value: Box::new(value)};
        inner.messages.post(self.inner, NWG_PACK_RESOURCE, Box::new(data) as Box<Any> );
    }

     /**
        Remove a element from the ui using its ID. The ID can identify a control, a resource or a user value.  
        Asynchronous, this only registers the command in the ui message queue.   
        Either call `ui.commit` to execute it now or wait for the command to be executed in the main event loop.

        Commit returns:  
          • `Error::ControlInUse` if the control callbacks are being executed  
          • `Error::ControlInUse` if the object is currently borrowed (using ui.get or ui.get_mut)  
          • `Error::KeyNotFound` if the id do not exists in the Ui
    */
    pub fn unpack(&self, id: &ID) {
        use low::defs::{NWG_UNPACK};
        
        let inner = unsafe{ &mut *self.inner };
        let (inner_id, _) = UiInner::hash_id(id, &TypeId::of::<()>());
        let data = UnpackArgs{ id: inner_id };
        inner.messages.post(self.inner, NWG_UNPACK, Box::new(data) as Box<Any> );
    }

    /**
        Return an immutable reference to the element identified by `id` in the Ui.  
        It is required to give a type `T` to this function as it is needed to cast the underlying value.
        Ex: `ui.get::<u32>(100)`

        Params:  
          • id: The id that identify the element in the ui

        Commit returns:  
          • `Error::KeyNotFound` will be returned if the key was not found in the Ui  
          • `Error::BadType` will be returned if the key exists, but the type do not match  
          • `Error::BorrowError` will be returned if the element was already borrowed mutably
    */
    pub fn get<T: 'static>(&self, id: &ID) -> Result<Ref<Box<T>>, Error> {
        use std::mem;
         
        let inner = unsafe{ &mut *self.inner };
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

        if let Some(v) = inner.resources.get(&inner_type_id) {
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
        Return an mutable reference to element identified by `id` in the Ui.
        It is required to give a type `T` to this function as it is needed to cast the underlying value.
        Ex: `ui.get_mut::<u32>(100)`

        Params:  
          • id: The id that identify the element in the ui

        Commit returns:  
          • `Error::KeyNotFound` will be returned if the key was not found in the Ui  
          • `Error::BadType` will be returned if the key exists, but the type do not match  
          • `Error::BorrowError` will be returned if the element was already borrowed mutably
    */
    pub fn get_mut<T: 'static>(&self, id: &ID) -> Result<RefMut<Box<T>>, Error> {
        use std::mem;
         
        let inner = unsafe{ &mut *self.inner };
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

        if let Some(v) = inner.resources.get(&inner_type_id) {
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
        Bind/Add a callback to a control event. 
        Asynchronous, this only registers the command in the ui message queue. 
        Either call `ui.commit` to execute it now or wait for the command to be executed in the main event loop.

        Params:
          • id: The id that identify the element in the ui  
          • cb_id: An id the identify the callback (to use with unbind)  
          • event: Type of event to target  
          • cb: The callback  

        Commit returns:  
          • `Error::EventNotSupported` if the event is not supported on the callback  
          • `Error::ControlRequired` if the id do not indentify a control  
          • `Error::KeyNotFound` if the id is not in the Ui.  
          • `Error::KeyExists` if the cb_id is not unique for the event type.  
          • `Error::ControlInUse` if NWG is currently executing the callback of the event
        
    */
    pub fn bind<T>(&self, id: &ID, cb_id: &ID, event: Event, cb: T) where
      T: Fn(&Ui<ID>, &ID, &Event, &EventArgs) -> ()+'static {
        use low::defs::{NWG_BIND};
        
        let inner = unsafe{ &mut *self.inner };
        let (inner_id, _) = UiInner::hash_id(id, &TypeId::of::<()>());
        let (cb_inner_id, _) = UiInner::hash_id(cb_id, &TypeId::of::<()>());
        let data = BindArgs{ id: inner_id, cb_id: cb_inner_id, event: event, cb: Box::new(cb)};
        inner.messages.post(self.inner, NWG_BIND, Box::new(data) as Box<Any> );
    }

    /**
        Unbind/Remove a callback to a control event.
        Asynchronous, this only registers the command in the ui message queue. 
        Either call `ui.commit` to execute it now or wait for the command to be executed in the main event loop.

        Commit returns:  
          • `Error::EventNotSupported` if the event is not supported on the callback  
          • `Error::ControlRequired` if the id do not indentify a control  
          • `Error::KeyNotFound` if the id is not in the Ui.  
          • `Error::KeyNotFound` if the cb_id do not exist for the event  
          • `Error::ControlInUse` if NWG is currently executing the callback of the event
    */
    pub fn unbind(&self, id: &ID, cb_id: &ID, event: Event) {
        use low::defs::{NWG_UNBIND};
        
        let inner = unsafe{ &mut *self.inner };
        let (inner_id, _) = UiInner::hash_id(id, &TypeId::of::<()>());
        let (cb_inner_id, _) = UiInner::hash_id(cb_id, &TypeId::of::<()>());
        let data = UnbindArgs{ id: inner_id, cb_id: cb_inner_id, event: event};
        inner.messages.post(self.inner, NWG_UNBIND, Box::new(data) as Box<Any> );
    }

    /**
        Return the underlying handle of a control or a resource.
        While this method is safe, anything done with the returned handle definitely won't be.

        Returns:  
          • `Ok(AnyHandle)` if the control or the resource is found
          • `Error::KeyNotFound` if the id is not in the Ui.  
          • `Error::ControlOrResourceRequired` if the id indentify a user value  
          • `Error::BorrowError` if the element was already borrowed mutably  
    */
    pub fn handle_of(&self, id: &ID) -> Result<AnyHandle, Error> {
        let inner = unsafe{ &mut *self.inner };
        let (inner_id, _) = UiInner::hash_id(id, &TypeId::of::<()>());
        inner.handle_of(inner_id)
    }

    /**
        Check if an id exists in the ui

        Params:  
          • id -> The ID to check  
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

/**
    Send a WM_QUIT to the system queue. Breaks the dispatch_events loop.
*/
pub fn exit() {
    // Actual code is located under the low module because that's where most of the unsafe code should be
    unsafe{ ::low::events::exit(); }
}