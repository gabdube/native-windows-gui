/*!
    Designates a control consisting of a list box and a selection field similar to an edit control. 
*/
use std::hash::Hash;
use std::mem;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text,
 get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
 set_window_parent, get_window_enabled, set_window_enabled, get_window_visibility,
 set_window_visibility, send_message, to_utf16_ref};
use actions::{Action, ActionReturn};
use constants::{CBS_AUTOHSCROLL, CBS_DROPDOWNLIST, CBS_HASSTRINGS, Error, CB_ERR};
use events::Event;

use winapi::{HWND, BS_NOTIFY, CB_RESETCONTENT, CB_ADDSTRING, CB_DELETESTRING,
 CB_FINDSTRINGEXACT, CB_GETCOUNT, CB_GETCURSEL, CB_SETCURSEL, LPARAM, WPARAM};

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
    pub collection: Vec<String>
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for ComboBox<ID> {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let base = WindowBase::<ID> {
            text: "".to_string(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: true,
            resizable: false,
            extra_style: BS_NOTIFY | CBS_AUTOHSCROLL | CBS_DROPDOWNLIST | CBS_HASSTRINGS,
            class: Some("COMBOBOX".to_string()),
            parent: Some(self.parent.clone())
        };

        let handle = unsafe { create_base::<ID>(ui, base) };

        match handle {
            Ok(h) => {
                 for i in self.collection.iter() {
                    add_string_item::<ID>(h, i);
                 }
                 Ok(h)
            }
            e => e
        }
    }

    fn supported_events(&self) -> Vec<Event> {
        vec![Event::Focus, Event::MouseUp, Event::MouseDown, Event::Removed]
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
                Action::SetParent(p) => set_window_parent(ui, handle, *p, true),
                Action::GetEnabled => get_window_enabled(handle),
                Action::SetEnabled(e) => set_window_enabled(handle, e),
                Action::GetVisibility => get_window_visibility(handle),
                Action::SetVisibility(v) => set_window_visibility(handle, v),
                Action::GetSelectedIndex => get_selected_index(handle),
                Action::SetSelectedIndex(i) => set_selected_index(handle, i), 
                Action::Reset => reset_combobox(handle),
                
                Action::AddString(s) => add_string_item(handle, s.as_ref()),
                Action::FindString(s) => find_string_item(handle, s.as_ref()),
                Action::RemoveString(s) => remove_string_item(handle, s.as_ref()),

                Action::RemoveItem(i) => remove_item(handle, i),
                Action::CountItems => count_item(handle),

                _ => ActionReturn::NotSupported
            }
        })
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
pub fn add_string_item<ID: Eq+Clone+Hash >(handle: HWND, item: &String) -> ActionReturn<ID> {
    let item_vec = to_utf16_ref(item);
    let item_vec_ptr: LPARAM = unsafe { mem::transmute(item_vec.as_ptr()) };
    send_message(handle, CB_ADDSTRING, 0, item_vec_ptr);
    ActionReturn::None
}

/**
    Remove an item from a list by using its index as reference
*/
pub fn remove_item<ID: Eq+Clone+Hash >(handle: HWND, index: u32) -> ActionReturn<ID> {
    if send_message(handle, CB_DELETESTRING, index as WPARAM, 0) != CB_ERR {
        ActionReturn::None
    } else {
        ActionReturn::Error(Error::INDEX_OUT_OF_BOUNDS)
    }
}


/**
    Find the index of a string item in a combobox
*/
pub fn find_string_item<ID: Eq+Clone+Hash >(handle: HWND, s: &String) -> ActionReturn<ID> {
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
pub fn remove_string_item<ID: Eq+Clone+Hash >(handle: HWND, s: &String) -> ActionReturn<ID> {
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
pub fn count_item<ID: Eq+Clone+Hash >(handle: HWND) -> ActionReturn<ID> {
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
pub fn get_selected_index<ID: Eq+Clone+Hash >(handle: HWND) -> ActionReturn<ID> {
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
pub fn set_selected_index<ID: Eq+Clone+Hash>(handle: HWND, index: Option<u32>) -> ActionReturn<ID> {
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