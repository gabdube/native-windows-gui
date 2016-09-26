/*!
    A simple button
*/
use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text,
 get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
 set_window_parent, send_message};
use actions::{Action, ActionReturn};
use constants::CheckState;
use events::Event;

use winapi::{HWND, BS_AUTOCHECKBOX, BS_NOTIFY, BS_AUTO3STATE};

use constants::{BM_GETSTATE, BST_CHECKED, BST_INDETERMINATE};

/**
    Configuration properties to create simple checkbox

    * text: The checkbox text
    * size: The checkbox size (width, height) in pixels
    * position: The checkbox position (x, y) in the parent control
    * parent: The control parent
    * tristate: If the checkbox should have 3 check instead of 2
*/
pub struct CheckBox<ID: Eq+Clone+Hash> {
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub parent: ID,
    pub tristate: bool,
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for CheckBox<ID> {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let extra;
        if self.tristate { extra = BS_AUTO3STATE | BS_NOTIFY; }
        else { extra = BS_AUTOCHECKBOX | BS_NOTIFY; }

        let base = WindowBase::<ID> {
            text: self.text.clone(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: true,
            resizable: false,
            extra_style: extra,
            class: Some("BUTTON".to_string()),
            parent: Some(self.parent.clone())
        };

        unsafe { create_base::<ID>(ui, base) }
    }

    fn supported_events(&self) -> Vec<Event> {
        vec![Event::ButtonClick, Event::Focus]
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
                Action::GetCheckState => get_check_state::<ID>(handle),
                _ => ActionReturn::NotSupported
            }
        })
    }

}

fn get_check_state<ID: Eq+Clone+Hash >(handle: HWND) -> ActionReturn<ID> {
    let state = send_message(handle, BM_GETSTATE, 0, 0) as u32;
    let state = if state & BST_CHECKED != 0 {
        CheckState::Checked
    } else if state & BST_INDETERMINATE != 0 {
        CheckState::Indeterminate
    } else {
        CheckState::Unchecked
    };

    ActionReturn::CheckState(state)
}