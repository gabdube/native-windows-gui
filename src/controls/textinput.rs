/*!
    A control where the user can enter text
*/

use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text,
 get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
 set_window_parent, get_window_enabled, set_window_enabled, get_window_visibility,
 set_window_visibility};
use actions::{Action, ActionReturn};
use events::Event;
use constants::{HTextAlign};

use winapi::{HWND, ES_LEFT, ES_RIGHT, ES_CENTER, WS_BORDER, ES_AUTOHSCROLL};

/**
    Configuration properties to create a simple TextInput

    * text: The button text
    * size: The button size (width, height) in pixels
    * position: The button position (x, y) in the parent control
    * parent: The control parent
*/
pub struct TextInput<ID: Eq+Clone+Hash> {
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub parent: ID,
    pub text_align: HTextAlign
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for TextInput<ID> {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let h_align = match self.text_align {
            HTextAlign::Left => ES_LEFT,
            HTextAlign::Right => ES_RIGHT,
            HTextAlign::Center => ES_CENTER
        };

        let base = WindowBase::<ID> {
            text: self.text.clone(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: true,
            resizable: false,
            extra_style: h_align | WS_BORDER | ES_AUTOHSCROLL,
            class: Some("EDIT".to_string()),
            parent: Some(self.parent.clone())
        };

        unsafe { create_base::<ID>(ui, base) }
    }

    fn supported_events(&self) -> Vec<Event> {
        vec![Event::Focus, Event::ValueChanged, Event::MaxValue]
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

                Action::GetTextLimit => get_text_limit(handle),
                Action::SetTextLimit(l) => set_text_limit(handle, l),
                Action::GetSelectedText => get_select_text(handle),
                Action::SetSelectedText(t) => set_select_text(handle, *t),
                Action::Undo => undo_text(handle),

                _ => ActionReturn::NotSupported
            }
        })
    }

}

use winapi::{EM_LIMITTEXT, EM_GETLIMITTEXT, UINT, WPARAM, WM_UNDO, EM_GETSEL, DWORD};
use controls::base::{send_message};
use constants::Error;
use std::mem;

fn get_text_limit<ID: Eq+Clone+Hash>(handle: HWND) -> ActionReturn<ID> {
    let limit = send_message(handle, EM_GETLIMITTEXT as UINT, 0, 0) as u32;
    ActionReturn::TextLimit(limit)
}

fn set_text_limit<ID: Eq+Clone+Hash>(handle: HWND, limit: u32) -> ActionReturn<ID> {
    send_message(handle, EM_LIMITTEXT as UINT, limit as WPARAM, 0);
    ActionReturn::None
}

fn undo_text<ID: Eq+Clone+Hash>(handle: HWND) -> ActionReturn<ID> {
    send_message(handle, WM_UNDO as UINT, 0, 0);
    ActionReturn::None
}

fn get_select_text<ID: Eq+Clone+Hash>(handle: HWND) -> ActionReturn<ID> {
    let mut min: DWORD = 0;
    let mut max: DWORD = 0;
    
    unsafe{ send_message(handle, EM_GETSEL as u32, mem::transmute(&mut min), mem::transmute(&mut max)) };

    if let ActionReturn::Text(t) = get_window_text::<ID>(handle) {
        let min = if min == !0 { 0usize } else { min as usize };
        let max = if max == !0 { t.len() } else { max as usize };
        ActionReturn::None
    } else {
        ActionReturn::Error(Error::UNKNOWN)
    }
}

fn set_select_text<ID: Eq+Clone+Hash>(handle: HWND, text: String) -> ActionReturn<ID> {
    ActionReturn::None
}