/*!
    Designates a control consisting of a list box and a selection field similar to an edit control. 
*/
use std::hash::Hash;
use std::mem;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;


use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text,
 get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
 set_window_parent, get_window_enabled, set_window_enabled, get_window_visibility,
 set_window_visibility, send_message, to_utf16_ref, get_control_type};
use actions::{Action, ActionReturn};
use constants::{CBS_AUTOHSCROLL, CBS_DROPDOWNLIST, CBS_HASSTRINGS, Error, CB_ERR, CBS_SORT, ControlType};
use events::Event;

use winapi::{HWND, BS_NOTIFY, CB_RESETCONTENT, CB_ADDSTRING, CB_DELETESTRING,
 CB_FINDSTRINGEXACT, CB_GETCOUNT, CB_GETCURSEL, CB_SETCURSEL, LPARAM, WPARAM,
 CB_SHOWDROPDOWN, CB_GETDROPPEDSTATE, CB_GETLBTEXT, CB_GETLBTEXTLEN};

/**
    Configuration properties to create simple button

    * size: The button size (width, height) in pixels
    * position: The button position (x, y) in the parent control
    * parent: The control parent
    * collection: List of combobox choice
*/
pub struct ComboBox<ID: Eq+Clone+Hash> {
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub parent: ID,
    pub collection: Vec<String>,
    pub sorted: bool
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for ComboBox<ID> {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let sorted = if self.sorted { CBS_SORT } else { 0 };

        let base = WindowBase::<ID> {
            text: "".to_string(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: true,
            resizable: false,
            extra_style: BS_NOTIFY | CBS_AUTOHSCROLL | CBS_DROPDOWNLIST | CBS_HASSTRINGS | sorted,
            class: Some("COMBOBOX".to_string()),
            parent: Some(self.parent.clone())
        };

        let handle = unsafe { create_base::<ID>(ui, base) };

        match handle {
            Ok(h) => {
                 set_collection::<ID>(h, &self.collection);
                 Ok(h)
            }
            e => e
        }
    }

    fn supported_events(&self) -> Vec<Event> {
        vec![Event::Focus, Event::MouseUp, Event::MouseDown, Event::Removed,
             Event::MenuClose, Event::MenuOpen, Event::SelectionChanged,
             Event::ValueChanged]
    }

    fn evaluator(&self) -> ::ActionEvaluator<ID> {
        Box::new( |ui, id, handle, action| {
            match action {
                Action::SetText(t) => set_window_text(handle, *t),
                Action::GetText => get_window_text(handle),
                Action::GetPosition => get_window_pos(handle, true),
                Action::SetPosition(x, y) => set_window_pos(handle, x, y),
                Action::GetSize => get_window_size(handle),
                Action::SetSize(w, h) => set_window_size(handle, w, h),
                Action::GetParent => get_window_parent(handle),
                Action::SetParent(p) => set_window_parent(ui, handle, p, true),
                Action::GetEnabled => get_window_enabled(handle),
                Action::SetEnabled(e) => set_window_enabled(handle, e),
                Action::GetVisibility => get_window_visibility(handle),
                Action::SetVisibility(v) => set_window_visibility(handle, v),
                Action::GetSelectedIndex => get_selected_index(handle),
                Action::SetSelectedIndex(i) => set_selected_index(handle, i), 
                Action::Reset => reset_combobox(handle),
                Action::GetControlType => get_control_type(handle),

                Action::GetDropdownVisibility => get_combobox_dropdown_visibility(handle),
                Action::SetDropdownVisibility(s) => show_combobox_dropdown(handle, s),
                
                Action::AddString(s) => add_string_item(handle, s.as_ref()),
                Action::FindString(s) => find_string_item(handle, s.as_ref()),
                Action::RemoveString(s) => remove_string_item(handle, s.as_ref()),
                Action::GetStringCollection => get_collection(handle),
                Action::SetStringCollection(c) => set_collection(handle, c.as_ref()),

                Action::GetIndexedItem(i) => get_item(handle, i),
                Action::RemoveIndexedItem(i) => remove_item(handle, i),
                Action::CountItems => count_item(handle),

                _ => ActionReturn::NotSupported
            }
        })
    }

    fn control_type(&self) -> ControlType {
        ControlType::ComboBox
    }

}

#[inline(always)]
fn reset_combobox<ID: Eq+Clone+Hash>(handle: HWND) -> ActionReturn<ID> {
    send_message(handle, CB_RESETCONTENT, 0, 0);
    ActionReturn::None
}

/**
    Add a string to a combobox
*/
#[inline(always)]
fn add_string_item<ID: Eq+Clone+Hash >(handle: HWND, item: &String) -> ActionReturn<ID> {
    let item_vec = to_utf16_ref(item);
    let item_vec_ptr: LPARAM = unsafe { mem::transmute(item_vec.as_ptr()) };
    send_message(handle, CB_ADDSTRING, 0, item_vec_ptr);
    ActionReturn::None
}

/**
    Return the string at index `i` in the combobox dropdown
*/
#[inline(always)]
fn get_item<ID: Eq+Clone+Hash>(handle: HWND, i: u32) -> ActionReturn<ID> {
    let buffer_len: usize;
    let mut buffer: Vec<u16>;
    let buffer_addr: LPARAM;

    buffer_len = send_message(handle, CB_GETLBTEXTLEN, i as WPARAM, 0) as usize;
    if buffer_len == (CB_ERR as usize) {
        return ActionReturn::Error(Error::INDEX_OUT_OF_BOUNDS)
    }
    
    buffer = Vec::with_capacity(buffer_len);
    unsafe{ 
        buffer.set_len(buffer_len); 
        buffer_addr = mem::transmute(buffer.as_mut_ptr());
    }

    if send_message(handle, CB_GETLBTEXT, i as WPARAM, buffer_addr) != CB_ERR {
        let text = OsString::from_wide(&(buffer.as_slice()));
        let text = text.into_string().unwrap_or("ERROR!".to_string());

        ActionReturn::Text(Box::new(text))
    } else {
        ActionReturn::Error(Error::UNKNOWN)
    }
}

/**
    Remove an item from a list by using its index as reference
*/
#[inline(always)]
fn remove_item<ID: Eq+Clone+Hash >(handle: HWND, index: u32) -> ActionReturn<ID> {
    if send_message(handle, CB_DELETESTRING, index as WPARAM, 0) != CB_ERR {
        ActionReturn::None
    } else {
        ActionReturn::Error(Error::INDEX_OUT_OF_BOUNDS)
    }
}


/**
    Find the index of a string item in a combobox
*/
#[inline(always)]
fn find_string_item<ID: Eq+Clone+Hash >(handle: HWND, s: &String) -> ActionReturn<ID> {
    let item_vec = to_utf16_ref(s);
    let item_vec_ptr: LPARAM = unsafe { mem::transmute(item_vec.as_ptr()) };
    let index = send_message(handle, CB_FINDSTRINGEXACT, 0, item_vec_ptr); 
    if index != CB_ERR {
        ActionReturn::ItemIndex(index as u32)
    } else {
        ActionReturn::Error(Error::ITEM_NOT_FOUND)
    }
}

/**
    Remove a string from a combobox
*/
#[inline(always)]
fn remove_string_item<ID: Eq+Clone+Hash >(handle: HWND, s: &String) -> ActionReturn<ID> {
    let item_vec = to_utf16_ref(s);
    let item_vec_ptr: LPARAM = unsafe { mem::transmute(item_vec.as_ptr()) };
    let index = send_message(handle, CB_FINDSTRINGEXACT, 0, item_vec_ptr); 
    if index != CB_ERR {
        send_message(handle, CB_DELETESTRING, index as WPARAM, 0);
        ActionReturn::None
    } else {
        ActionReturn::Error(Error::ITEM_NOT_FOUND)
    }
}

/**
    Count the number of item in a combobox
*/
#[inline(always)]
fn count_item<ID: Eq+Clone+Hash >(handle: HWND) -> ActionReturn<ID> {
    let count = send_message(handle, CB_GETCOUNT, 0, 0);
    if count != CB_ERR {
        ActionReturn::ItemCount(count as u32)
    } else {
        ActionReturn::Error(Error::UNKNOWN)
    }
}

/**
    Return the index of the selected item in a combobox
*/
#[inline(always)]
fn get_selected_index<ID: Eq+Clone+Hash >(handle: HWND) -> ActionReturn<ID> {
    let selected = send_message(handle, CB_GETCURSEL, 0, 0);
    if selected != CB_ERR {
        ActionReturn::ItemIndex(selected as u32)
    } else {
        ActionReturn::None
    }
}

/**
    Set the selected index in a combobox
    TODO maybe add some validation to check if index is out of bounds as it currently clear the box
    as if None was passed
*/
#[inline(always)]
fn set_selected_index<ID: Eq+Clone+Hash>(handle: HWND, index: Option<u32>) -> ActionReturn<ID> {
    if let Some(i) = index {
        let result = send_message(handle, CB_SETCURSEL, i as WPARAM, 0);
        if result != CB_ERR {
            ActionReturn::None
        } else {
            ActionReturn::Error(Error::INDEX_OUT_OF_BOUNDS)
        }
    } else {
        send_message(handle, CB_SETCURSEL, !0, 0);
        ActionReturn::None
    }
}

/**
    Set the visibility of the combobox dropdown
*/
#[inline(always)]
fn show_combobox_dropdown<ID: Eq+Clone+Hash>(handle: HWND, show: bool) -> ActionReturn<ID> {
    send_message(handle, CB_SHOWDROPDOWN, show as WPARAM, 0);
    ActionReturn::None
}

/**
    Get the visibility of a combobox dropdown
*/
#[inline(always)]
fn get_combobox_dropdown_visibility<ID: Eq+Clone+Hash>(handle: HWND) -> ActionReturn<ID> {
    ActionReturn::Visibility(send_message(handle, CB_GETDROPPEDSTATE, 0, 0) == 1)
}


/**
    Return every item contained in the combobox list
*/
#[inline(always)]
fn get_collection<ID: Eq+Clone+Hash>(handle: HWND) -> ActionReturn<ID> {
    let item_count: usize;
    let mut buffer_length: usize = 0;
    let mut buffer: Vec<u16>;
    let buffer_addr: LPARAM;
    let mut collection: Vec<String>;

    if let ActionReturn::ItemCount(c) = count_item::<ID>(handle) {
        item_count = c as usize;
    } else {
        return ActionReturn::Error(Error::UNKNOWN);
    }

    // If there are no items, return an empty vector
    if item_count == 0 { return ActionReturn::StringCollection(Box::new(Vec::new())); }

    // Get the length of the biggest string in the combobox
    for i in 0..(item_count as WPARAM) {
        let size = send_message(handle, CB_GETLBTEXTLEN, i, 0) as usize;
        if size > buffer_length {
            buffer_length = size;
        }
    }

    // Create the buffer
    buffer = Vec::with_capacity(buffer_length);
    unsafe {
        buffer.set_len(buffer_length);
        buffer_addr = mem::transmute(buffer.as_mut_ptr());
    }

    // Unpack the items in the collection
    collection = Vec::with_capacity(item_count);
    for i in 0..(item_count as WPARAM) {
        if send_message(handle, CB_GETLBTEXT, i, buffer_addr) != CB_ERR {
            let end_index = buffer.iter().enumerate().find(|&(index, i)| *i == 0).unwrap_or((buffer_length, &0)).0;
            let text = OsString::from_wide(&(buffer.as_slice()[0..end_index]));
            let text = text.into_string().unwrap_or("ERROR!".to_string());

            collection.push(text);
        }
    }

    ActionReturn::StringCollection(Box::new(collection))
}

/**
    Set the collection in the combobox list
*/
fn set_collection<ID: Eq+Clone+Hash>(handle: HWND, collection: &Vec<String>) -> ActionReturn<ID> {
    reset_combobox::<ID>(handle);
    for i in collection.iter() {
        add_string_item::<ID>(handle, i);
    }

    ActionReturn::None
}