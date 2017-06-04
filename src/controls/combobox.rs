/*!
    Combobox control definition
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
use std::any::TypeId;
use std::fmt::Display;
use std::mem;

use user32::SendMessageW;
use winapi::{HWND, HFONT, WPARAM};

use ui::Ui;
use error::Error;
use controls::{Control, ControlT, ControlType, AnyHandle};
use events::{Event, Destroyed, Moved, Resized};
use events::combobox::{Focus, SelectionChanged};
use low::other_helper::{to_utf16, from_utf16};

/**
    Template that creates a combobox control

    Available events:  
    `Destroyed, Moved, Resized, SelectionChanged, Focus, Any`  

    Members:  
    • `collection`: Item collection of the combobox. The item type must implement `Display`  
    • `position`: The start position of the combobox  
    • `size`: The start size of the combobox  
    • `visible`: If the combobox should be visible to the user  
    • `disabled`: If the user can or can't use the combobox   
    • `parent`: The combobox parent  
    • `font`: The combobox font. If None, use the system default  
*/
#[derive(Clone)]
pub struct ComboBoxT<D: Clone+Display+'static, ID: Hash+Clone, S: Clone+Into<String>> {
    pub collection: Vec<D>,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub placeholder: Option<S>,
    pub parent: ID,
    pub font: Option<ID>,
}

impl<D: Clone+Display+'static, ID: Hash+Clone, S: Clone+Into<String>> ControlT<ID> for ComboBoxT<D, ID, S> {
    fn type_id(&self) -> TypeId { TypeId::of::<ComboBox<D>>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, Moved, Resized, SelectionChanged, Focus, Event::Any]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font, handle_of_window, handle_of_font};
        use low::defs::{CBS_DROPDOWNLIST, CBS_HASSTRINGS, CB_ADDSTRING};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD};
        use user32::SendMessageW;

        let flags: DWORD = WS_CHILD | CBS_HASSTRINGS | CBS_DROPDOWNLIST |
        if self.visible      { WS_VISIBLE }      else { 0 } |
        if self.disabled     { WS_DISABLED }     else { 0 };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a combobox must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a combobox must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: "",
            class_name: "COMBOBOX",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ 
                    // Set font
                    set_window_font(h, font_handle, true); 

                    // Set placeholder
                    match self.placeholder.as_ref() {
                        Some(p) => { set_placeholder(h, p.clone()); },
                        _ => {}
                    }

                    // Init collection
                    let collection: Vec<D> = self.collection.iter().map(
                        |s|{  
                            let text = to_utf16(format!("{}", s).as_str());
                            SendMessageW(h, CB_ADDSTRING, 0, mem::transmute(text.as_ptr()));
                            s.clone() 
                        } 
                    ).collect();


                    Ok( Box::new(ComboBox{handle: h, collection: collection}) )
                }
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A combobox control
*/
pub struct ComboBox<D: Clone+Display> {
    handle: HWND,
    collection: Vec<D>
}

impl<D: Clone+Display> ComboBox<D> {

    /// Return the number of items in the inner collection
    pub fn len(&self) -> usize { self.collection.len() }

    /// Return the inner collection of the combobox
    pub fn collection(&self) -> &Vec<D> { &self.collection }

    /// Return the inner collection of the combobox, mutable.
    /// If the inner collection is changed, `combobox.sync` must be called to show the changes in the combobox
    pub fn collection_mut(&mut self) -> &mut Vec<D> { &mut self.collection }

    /// Reload the content of the combobox
    pub fn sync(&self) {
        use low::defs::{CB_RESETCONTENT, CB_ADDSTRING};

        unsafe{ SendMessageW(self.handle, CB_RESETCONTENT, 0, 0); }

        for i in self.collection.iter() {
            let text = to_utf16(format!("{}", i).as_str());
            unsafe{ SendMessageW(self.handle, CB_ADDSTRING, 0, mem::transmute(text.as_ptr())); }
        }
    }

    /// Add an item at the end of the combobox. Updates both the inner collection and the ui.
    pub fn push(&mut self, item: D) {
        use low::defs::CB_ADDSTRING;

        let text = to_utf16(format!("{}", item).as_str());
        unsafe{ SendMessageW(self.handle, CB_ADDSTRING, 0, mem::transmute(text.as_ptr())); }

        self.collection.push(item);
    }

    /// Remove an item from the inner collection and the combobox. Return the removed item.  
    /// `Panics` if index is out of bounds.
    pub fn remove(&mut self, index: usize) -> D {
        use low::defs::CB_DELETESTRING;
        unsafe{ SendMessageW(self.handle, CB_DELETESTRING, index as WPARAM, 0); }
        self.collection.remove(index)
    }

    /// Insert an item at the selected position in the lisbox and the inner collection.  
    /// If index is -1, the item is added at the end of the list.
    pub fn insert(&mut self, index: usize, item: D) {
        use low::defs::CB_INSERTSTRING;

        let text = to_utf16(format!("{}", item).as_str());
        unsafe{ SendMessageW(self.handle, CB_INSERTSTRING, index as WPARAM, mem::transmute(text.as_ptr())); }

        self.collection.insert(index, item);
    }

    /// Try to find an item with the text `text` in the collection. If one is found, return its index else, returns None.  
    /// If `full_match` is true, the text must match exactly otherwise it only needs to match the beginning.
    /// The search is NOT case sensitive.
    pub fn find_string<'a>(&self, text: &'a str, full_match: bool) -> Option<usize> {
        use low::defs::{CB_FINDSTRING, CB_FINDSTRINGEXACT};

        let text = to_utf16(text);
        let msg = if full_match { CB_FINDSTRINGEXACT } else { CB_FINDSTRING };
        let index = unsafe{ SendMessageW(self.handle, msg, -1isize as WPARAM, mem::transmute(text.as_ptr()) ) };

        if index == -1 {
            None
        } else {
            Some(index as usize)
        }
    }

    /// Return the index of currently selected item.  
    /// Return None if there is no selected item
    pub fn get_selected_index(&self) -> Option<usize> {
        use low::defs::CB_GETCURSEL;

        let index = unsafe{ SendMessageW(self.handle, CB_GETCURSEL, 0, 0) };
        if index == -1 { None } 
        else { Some(index as usize) }
    }

    /// Return the currently selected element text. Returns `""` if no item is selected.
    pub fn get_selected_text(&self) -> String {
        unsafe{ ::low::window_helper::get_window_text(self.handle) }
    }

    /// Set the selected index in a combobox.  
    /// If `index` is `usize::max_value`, remove the selected index from the combobox
    pub fn set_selected_index(&self, index: usize) {
        use low::defs::CB_SETCURSEL;
        unsafe{ SendMessageW(self.handle, CB_SETCURSEL, index as WPARAM, 0); }
    }

    /// Return the item text at the provided index. Returns None if the index is not valid.
    pub fn get_string(&self, index: usize) -> Option<String> {
        use low::defs::{CB_GETLBTEXT, CB_GETLBTEXTLEN};

        let length = unsafe{ SendMessageW(self.handle, CB_GETLBTEXTLEN, index as WPARAM, 0) };
        if length == -1 { return None; }

        let length = (length+1) as usize;
        let mut buffer: Vec<u16> = Vec::with_capacity(length);
        unsafe {
            buffer.set_len(length);
            let err = SendMessageW(self.handle, CB_GETLBTEXT, index as WPARAM, mem::transmute( buffer.as_mut_ptr() ));
            if err == -1 { return None; }
        }

       Some( from_utf16(&buffer[..]) )
    }

    /// Set a new placeholder for the combobox. To remove the current placeholder, send `""`  
    /// The maximum length of the placeholder is 255 characters
    pub fn set_placeholder<'a>(&self, placeholder: &'a str) {
        set_placeholder(self.handle, placeholder);
    }

    /* Return the current placeholder for the combobox. If there are no placeholder set, returns None.

    CB_GETCUEBANNER IS NOT RELIABLE (blame Windows for this one).

    pub fn get_placeholder(&self) -> Option<String> {
        use winapi::CB_GETCUEBANNER;

        let mut buffer: [u16; 256] = [0; 256];
        let mut buffer_size = 0;

        let placeholder_found = unsafe{ SendMessageW(self.handle, CB_GETCUEBANNER, mem::transmute(buffer.as_mut_ptr()), mem::transmute(&mut buffer_size)) };

        if placeholder_found == 1 {
            Some(from_utf16(&buffer))
        } else {
            None
        }
    }
    */

    /// Return true if the combobox dropdown is visible
    pub fn get_dropped_state(&self) -> bool {
        use low::defs::CB_GETDROPPEDSTATE;
        unsafe{ SendMessageW(self.handle, CB_GETDROPPEDSTATE, 0, 0) != 0 }
    }

    /// Show or hide the control dropdown
    pub fn set_dropped_state(&self, dropped: bool) {
        use low::defs::CB_SHOWDROPDOWN;
        unsafe{ SendMessageW(self.handle, CB_SHOWDROPDOWN, dropped as WPARAM, 0); }
    }

    /// Remove every item in the inner collection and in the combobox
    pub fn clear(&mut self) {
        use low::defs::CB_RESETCONTENT;
        unsafe{ SendMessageW(self.handle, CB_RESETCONTENT, 0, 0) };
        self.collection.clear();
    }

    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
}
    

impl<D: Clone+Display> Control for ComboBox<D> {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::ComboBox 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }
}

// Private combobox methods

/// Set the placeholder of a combobox.
fn set_placeholder<S: Into<String>>(handle: HWND, placeholder: S) {
    use low::defs::CB_SETCUEBANNER;
    let text = to_utf16(placeholder.into().as_str());
    unsafe{ SendMessageW(handle, CB_SETCUEBANNER, 0, mem::transmute(text.as_ptr()) ); }
}