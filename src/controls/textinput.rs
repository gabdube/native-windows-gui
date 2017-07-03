/*!
    A simple text input control
*/

use std::hash::Hash;
use std::any::TypeId;
use std::mem;

use winapi::{HWND, HFONT, WPARAM};
use user32::SendMessageW;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use low::other_helper::to_utf16;
use error::Error;

/**
    A template that creates a single line textinput control

    Control specific events:  
    `textinput::ValueChanged, textinput::Focus` 

    Members:  
    • `text`: The text of the textinput  
    • `position`: The start position of the textinput  
    • `size`: The start size of the textinput  
    • `visible`: If the textinput should be visible to the user   
    • `disabled`: If the user can or can't click on the textinput  
    • `readonly`: If the user can copty the text but can't edit the textinput content  
    • `password`: If the textinput should hide its content  
    • `placeholder`: Some text that is displayed when the actual value is empty  
    • `limit`: The maximum number of characters that the control can hold  
    • `parent`: The textinput parent  
    • `font`: The textinput font. If None, use the system default  
*/
#[derive(Clone)]
pub struct TextInputT<S1: Clone+Into<String>, S2: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S1,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub password: bool,
    pub placeholder: Option<S2>,
    pub limit: u32,
    pub parent: ID,
    pub font: Option<ID>,
}

impl<S1: Clone+Into<String>, S2: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for TextInputT<S1, S2, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<TextInput>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, set_window_font_raw, handle_of_window, handle_of_font};
        use low::defs::{ES_AUTOHSCROLL, ES_READONLY, ES_PASSWORD, EM_LIMITTEXT};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, WS_BORDER};

        let flags: DWORD = WS_CHILD | WS_BORDER | ES_AUTOHSCROLL | 
        if self.readonly { ES_READONLY } else { 0 } |
        if self.password { ES_PASSWORD } else { 0 } |
        if self.visible  { WS_VISIBLE }  else { 0 } |
        if self.disabled { WS_DISABLED } else { 0 };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a textinput must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a button must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let params = WindowParams {
            title: self.text.clone().into(),
            class_name: "EDIT",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ 
                    set_window_font_raw(h, font_handle, true); 
                    SendMessageW(h, EM_LIMITTEXT, self.limit as WPARAM, 0);
                    
                    if let Some(placeholder) = self.placeholder.as_ref() {
                        set_placeholder(h, placeholder.clone());
                    }
                };

                Ok( Box::new(TextInput{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A single line textinput control
*/
pub struct TextInput {
    handle: HWND
}

impl TextInput {

    /// Set or unset the readonly status on the control
    pub fn set_readonly(&self, readonly: bool) {
        use winapi::{EM_SETREADONLY, UINT};
        unsafe{ SendMessageW(self.handle, EM_SETREADONLY as UINT, readonly as WPARAM, 0); }
    }
    
    /// Return `true` if the user cannot edit the content of the control or `false` if the user can
    pub fn get_readonly(&self) -> bool {
        use low::window_helper::get_window_long;
        use low::defs::ES_READONLY;
        use winapi::GWL_STYLE;

        let style = get_window_long(self.handle, GWL_STYLE) as u32;

        (style & ES_READONLY) == ES_READONLY
    }

    /// Add or remove a password mask on the control
    pub fn set_password(&self, password: bool) {
        use low::window_helper::{set_window_long, get_window_long};
        use low::defs::ES_PASSWORD;
        use winapi::GWL_STYLE;

        let old_style = get_window_long(self.handle, GWL_STYLE) as usize;
        if password {
            set_window_long(self.handle, GWL_STYLE, old_style|(ES_PASSWORD as usize));
        } else {
            set_window_long(self.handle, GWL_STYLE, old_style&(!ES_PASSWORD as usize) );
        }
    }

    /// Return `true` if the control has a password mask or `false` otherwise
    pub fn get_password(&self) -> bool {
        use low::window_helper::get_window_long;
        use low::defs::ES_PASSWORD;
        use winapi::GWL_STYLE;

        let style = get_window_long(self.handle, GWL_STYLE) as u32;

        (style & ES_PASSWORD) == ES_PASSWORD
    }

    /// Set the maximum number of characters that the control can hold
    pub fn set_limit(&self, limit: u32) {
        use low::defs::EM_LIMITTEXT;
        unsafe{ SendMessageW(self.handle, EM_LIMITTEXT, limit as WPARAM, 0); }
    }

    /// Return the maximum number of characters that the control can hold
    pub fn get_limit(&self) -> u32 {
        use low::defs::EM_GETLIMITTEXT;
        unsafe{ SendMessageW(self.handle, EM_GETLIMITTEXT, 0, 0) as u32 }
    }

    /// Set a new placeholder for the TextInput. To remove the current placeholder, send `""`  
    /// The maximum length of the placeholder is 255 characters
    pub fn set_placeholder<'a>(&self, placeholder: &'a str) {
        set_placeholder(self.handle, placeholder);
    }

    // Return the current placeholder for the TextInput. If there are no placeholder set, returns None.
    // EM_GETCUEBANNER IS NOT RELIABLE.
    /*pub fn get_placeholder(&self) -> Option<String> {
        use winapi::EM_GETCUEBANNER;

        let mut buffer: [u16; 256] = [0; 256];
        let mut buffer_size = 256;

        let placeholder_found = unsafe{ SendMessageW(self.handle, EM_GETCUEBANNER, mem::transmute(buffer.as_mut_ptr()), mem::transmute(&mut buffer_size)) };

        if placeholder_found == 1 {
            Some(from_utf16(&buffer))
        } else {
            None
        }
    }*/
    

    pub fn get_text(&self) -> String { unsafe{ ::low::window_helper::get_window_text(self.handle) } }
    pub fn set_text<'a>(&self, text: &'a str) { unsafe{ ::low::window_helper::set_window_text(self.handle, text); } }
    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
    pub fn get_font<ID: Hash+Clone>(&self, ui: &Ui<ID>) -> Option<ID> { unsafe{ ::low::window_helper::get_window_font(self.handle, ui) } }
    pub fn set_font<ID: Hash+Clone>(&self, ui: &Ui<ID>, f: Option<&ID>) -> Result<(), Error> { unsafe{ ::low::window_helper::set_window_font(self.handle, ui, f) } }
    pub fn update(&self) { unsafe{ ::low::window_helper::update(self.handle); } }
    pub fn focus(&self) { unsafe{ ::user32::SetFocus(self.handle); } }
}

impl Control for TextInput {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::TextInput 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}


fn set_placeholder<S: Into<String>>(handle: HWND, placeholder: S) {
    use winapi::EM_SETCUEBANNER;
    let text = to_utf16(placeholder.into().as_str());
    unsafe{ SendMessageW(handle, EM_SETCUEBANNER, 0, mem::transmute(text.as_ptr()) ); }
}