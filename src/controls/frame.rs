/*!
    A simple frame
*/

use std::hash::Hash;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text,
 get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
 set_window_parent, get_window_enabled, set_window_enabled, get_window_visibility,
 set_window_visibility, get_control_type};
use actions::{Action, ActionReturn};
use constants::{ControlType, FrameBorderStyle, FrameBorderColor, SS_NOTIFY,
 SS_SUNKEN, SS_BLACKFRAME, SS_GRAYFRAME, SS_WHITEFRAME};
use events::Event;

use winapi::{HWND};


/**
    Configuration properties to create simple label

    * size: The frame size (width, height) in pixels
    * position: The frame position (x, y) in the parent control
    * parent: The control parent
*/
pub struct Frame<ID: Eq+Clone+Hash> {
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub parent: ID,
    pub border_style: FrameBorderStyle,
    pub border_color: FrameBorderColor
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for Frame<ID> {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> {

        let style = match self.border_style {
            FrameBorderStyle::Sunken => SS_SUNKEN,
            FrameBorderStyle::Simple => 0
        };

        let color = match self.border_color {
            FrameBorderColor::Parent => 0,
            FrameBorderColor::Black => SS_BLACKFRAME,
            FrameBorderColor::Gray => SS_GRAYFRAME,
            FrameBorderColor::White => SS_WHITEFRAME
        };

        let base = WindowBase::<ID> {
            text: "".to_string(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: true,
            resizable: false,
            extra_style: SS_NOTIFY | style | color,
            class: Some("STATIC".to_string()),
            parent: Some(self.parent.clone())
        };

        unsafe { create_base::<ID>(ui, base) }
    }

    fn supported_events(&self) -> Vec<Event> {
        vec![Event::MouseUp, Event::MouseDown, Event::Removed, Event::Click]
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
                Action::SetParent(p) => set_window_parent(ui, handle, p, true),
                Action::GetEnabled => get_window_enabled(handle),
                Action::SetEnabled(e) => set_window_enabled(handle, e),
                Action::GetVisibility => get_window_visibility(handle),
                Action::SetVisibility(v) => set_window_visibility(handle, v),
                Action::GetControlType => get_control_type(handle),
                _ => ActionReturn::NotSupported
            }
        })
    }

    fn control_type(&self) -> ControlType {
        ControlType::Frame
    }

}