/*!
    A simple button
*/
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base};
use actions::{Action, ActionReturn};

use winapi::HWND;
use user32::{SetWindowTextW, GetWindowTextW, GetWindowTextLengthW};

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
            class: Some("BUTTON".to_string()),
            parent: Some(self.parent.clone())
        };

        unsafe { create_base::<ID>(ui, base) }
    }

    fn evaluator(&self) -> ::ActionEvaluator<ID> {
        Box::new( |ui, id, handle, action| {
            match action {
                Action::SetText(t) => set_window_text(handle, *t),
                Action::GetText => get_window_text(handle),
                _ => ActionReturn::None
            }
        })
    }

}

/**
    String to utf16. Add a trailing null char.
*/
#[inline(always)]
fn to_utf16(n: String) -> Vec<u16> {
    OsStr::new(n.as_str())
      .encode_wide()
      .chain(Some(0u16).into_iter())
      .collect()
}

fn set_window_text(handle: HWND, _text: String) -> ActionReturn { unsafe {
    let text = to_utf16(_text);
    SetWindowTextW(handle, text.as_ptr());
    ActionReturn::None
}}

fn get_window_text(handle: HWND) -> ActionReturn { unsafe {
    let text_length = (GetWindowTextLengthW(handle) as usize)+1;
    let mut buffer: Vec<u16> = Vec::with_capacity(text_length);
    buffer.set_len(text_length);

    GetWindowTextW(handle, buffer.as_mut_ptr(), text_length as i32);

    let text = OsString::from_wide(&(buffer.as_slice()[0..text_length-1]));
    let text = text.into_string().unwrap_or("ERROR!".to_string());
    ActionReturn::Text(Box::new(text))
}}