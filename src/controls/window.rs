/*!
    A blank custom control.
*/

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base};
use actions::{Action, ActionReturn, ActMessageParams};

use winapi::{UINT, HWND};
use user32::{MessageBoxW, SetWindowTextW, GetWindowTextW, GetWindowTextLengthW};

/**
    Configuration properties to create a window

    * caption: Window title (in the upper bar)
    * size: Window size (width, height) in pixels
    * position: Starting position (x, y) of the window 
    * visible: If the window should be visible from the start
    * resizable: If the window should be resizable by the user
*/
pub struct Window {
    pub caption: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub visible: bool,
    pub resizable: bool
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for Window {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {
        let base = WindowBase::<ID> {
            text: self.caption.clone(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: self.visible,
            resizable: self.resizable,
            class: None,
            parent: None
        };

        unsafe { create_base::<ID>(ui, base) }
    }

    fn evaluator(&self) -> ::ActionEvaluator<ID> {
        Box::new( |ui, id, handle, action| {
            match action {
                Action::Message(p) => show_message(handle, *p),
                Action::SetText(t) => set_window_text(handle, *t),
                Action::GetText => ActionReturn::None
                //_ => ActionReturn::None
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

fn show_message(handle: HWND, params: ActMessageParams) -> ActionReturn { unsafe {
    let text = to_utf16(params.content);
    let title = to_utf16(params.title);
    MessageBoxW(handle, text.as_ptr(), title.as_ptr(), params.type_ as UINT);
    ActionReturn::None
}}

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