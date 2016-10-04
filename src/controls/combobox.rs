/*!
    Designates a control consisting of a list box and a selection field similar to an edit control. 
*/
use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text,
 get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
 set_window_parent, get_window_enabled, set_window_enabled, get_window_visibility,
 set_window_visibility, add_string_item, remove_item, find_string_item, 
 remove_string_item, count_item, get_selected_index};
use actions::{Action, ActionReturn};
use constants::{CBS_AUTOHSCROLL, CBS_DROPDOWNLIST, CBS_HASSTRINGS};
use events::Event;

use winapi::{HWND, BS_NOTIFY};

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