/*!
    A blank custom control.
*/

use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text, show_message,
  get_window_pos, set_window_pos, get_window_size, set_window_size};
use actions::{Action, ActionReturn};

use winapi::{HWND};

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
                Action::GetText => get_window_text(handle),
                Action::GetPosition => get_window_pos(handle, false),
                Action::SetPosition(x, y) => set_window_pos(handle, x, y),
                Action::GetSize => get_window_size(handle),
                Action::SetSize(w, h) => set_window_size(handle, w, h),
                _ => ActionReturn::NotSupported
            }            
        })
    }

}