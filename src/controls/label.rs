/*!
    Label control definition
*/

use std::hash::Hash;
use std::any::TypeId;

use winapi::{HWND, HFONT};

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use defs::HTextAlign;

/**
    A template that creates a standard label

    Control specific events:  
    `label::Click, label::DoubleClick`

    Members:  
    • `text`: The text of the label  
    • `position`: The start position of the label  
    • `size`: The start size of the label  
    • `visible`: If the label should be visible to the user  
    • `disabled`: If the user can or can't click on the label  
    • `align`: The text align of the label
    • `parent`: The label parent  
    • `font`: The label font. If None, use the system default  
*/
#[derive(Clone)]
pub struct LabelT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub align: HTextAlign,
    pub parent: ID,
    pub font: Option<ID>,
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for LabelT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<Label>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font_raw, handle_of_window, handle_of_font};
        use low::defs::{SS_NOTIFY, SS_NOPREFIX, SS_LEFT, SS_RIGHT, SS_CENTER};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD};

        let flags: DWORD = WS_CHILD | SS_NOTIFY | SS_NOPREFIX | 
        if self.visible    { WS_VISIBLE }   else { 0 } |
        if self.disabled   { WS_DISABLED }  else { 0 } |
        match self.align   { HTextAlign::Center=>SS_CENTER, HTextAlign::Left=>SS_LEFT, HTextAlign::Right=>SS_RIGHT };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a label must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a label must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: self.text.clone().into(),
            class_name: "STATIC",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ set_window_font_raw(h, font_handle, true); }
                Ok( Box::new(Label{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A standard label
*/
pub struct Label {
    handle: HWND
}

impl Label {
    pub fn get_text(&self) -> String { unsafe{ ::low::window_helper::get_window_text(self.handle) } }
    pub fn set_text<'a>(&self, text: &'a str) { unsafe{ ::low::window_helper::set_window_text(self.handle, text); } }
    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, false); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
    pub fn get_font<ID: Hash+Clone>(&self, ui: &Ui<ID>) -> Option<ID> { unsafe{ ::low::window_helper::get_window_font(self.handle, ui) } }
    pub fn set_font<ID: Hash+Clone>(&self, ui: &Ui<ID>, f: Option<&ID>) -> Result<(), Error> { unsafe{ ::low::window_helper::set_window_font(self.handle, ui, f) } }
    pub fn update(&self) { unsafe{ ::low::window_helper::update(self.handle); } }
    pub fn focus(&self) { unsafe{ ::user32::SetFocus(self.handle); } }
}

impl Control for Label {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Label 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}