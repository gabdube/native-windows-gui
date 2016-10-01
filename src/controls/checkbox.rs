/*!
    A simple button
*/
use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text,
 get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
 set_window_parent, get_window_enabled, set_window_enabled, 
 get_window_visibility, set_window_visibility, get_check_state, set_check_state};
use actions::{Action, ActionReturn};
use constants::{HTextAlign, VTextAlign};
use events::Event;

use winapi::{HWND, BS_AUTOCHECKBOX, BS_NOTIFY, BS_AUTO3STATE, BS_LEFT, BS_RIGHT,
  BS_TOP, BS_CENTER, BS_BOTTOM, BS_RIGHTBUTTON};

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
    pub text_align: (HTextAlign, VTextAlign),
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for CheckBox<ID> {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let mut extra;
        if self.tristate { extra = BS_AUTO3STATE | BS_NOTIFY; }
        else { extra = BS_AUTOCHECKBOX | BS_NOTIFY; }

        extra |= match self.text_align.0 {
            HTextAlign::Left => BS_LEFT,
            HTextAlign::Right => BS_RIGHT | BS_RIGHTBUTTON,
            HTextAlign::Center => BS_CENTER
        };

        extra |= match self.text_align.1 {
            VTextAlign::Top => BS_TOP,
            VTextAlign::Bottom => BS_BOTTOM,
            VTextAlign::Center => BS_CENTER
        };

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
        vec![Event::Click, Event::Focus, Event::Removed]
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

                Action::GetCheckState => get_check_state(handle),
                Action::SetCheckState(s) => set_check_state(handle, s),

                _ => ActionReturn::NotSupported
            }
        })
    }

}