/*!
    A simple button
*/
use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text,
 get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
 set_window_parent};
use actions::{Action, ActionReturn};
use events::Event;

use winapi::{HWND, BS_NOTIFY};

/**
    Configuration properties to create simple button

    * text: The button text
    * size: The button size (width, height) in pixels
    * position: The button position (x, y) in the parent control
    * parent: The control parent
*/
pub struct Button<ID: Eq+Clone+Hash> {
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub parent: ID,
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for Button<ID> {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let base = WindowBase::<ID> {
            text: self.text.clone(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: true,
            resizable: false,
            extra_style: BS_NOTIFY,
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
                _ => ActionReturn::NotSupported
            }
        })
    }

}