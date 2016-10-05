/*!
    A blank custom control.
*/

use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text, show_message,
  get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
  set_window_parent, get_window_enabled, set_window_enabled, get_window_visibility,
  set_window_visibility, get_window_display, set_window_display, get_window_children,
  get_window_descendant};
use actions::{Action, ActionReturn};
use events::Event;

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
            extra_style: 0,
            class: None,
            parent: None
        };

        unsafe { create_base::<ID>(ui, base) }
    }

    fn supported_events(&self) -> Vec<Event> {
        vec![Event::MouseUp, Event::MouseDown, Event::Focus, Event::Removed]
    }

    fn evaluator(&self) -> ::ActionEvaluator<ID> {
        Box::new( |ui, id, handle, action| {
            match action {
                Action::Message(p) => show_message(handle, *p),
                Action::GetText => get_window_text(handle),
                Action::SetText(t) => set_window_text(handle, *t),
                Action::GetPosition => get_window_pos(handle, false),
                Action::SetPosition(x, y) => set_window_pos(handle, x, y),
                Action::GetSize => get_window_size(handle),
                Action::SetSize(w, h) => set_window_size(handle, w, h),
                Action::GetParent => get_window_parent(handle),
                Action::SetParent(p) => set_window_parent(ui, handle, p, false),
                Action::GetChildren => get_window_children(handle),
                Action::GetDescendants => get_window_descendant(handle),
                Action::GetEnabled => get_window_enabled(handle),
                Action::SetEnabled(e) => set_window_enabled(handle, e),
                Action::GetVisibility => get_window_visibility(handle),
                Action::SetVisibility(v) => set_window_visibility(handle, v),

                Action::GetWindowDisplay => get_window_display(handle),
                Action::SetWindowDisplay(d) => set_window_display(handle, d),
                
                _ => ActionReturn::NotSupported
            }            
        })
    }

}