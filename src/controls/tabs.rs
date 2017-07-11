/*!
    A tabs control
*/

use std::hash::Hash;
use std::any::TypeId;
use std::mem;

use winapi::{HWND, HFONT};
use user32::SendMessageW;

use ui::Ui;
use error::{Error, SystemError};
use controls::{Control, ControlT, ControlType, AnyHandle};

/**
    A template that creates a treeview

    Treeview specific events:  
    `treeview::SelectionChanged, treeview::Click, treeview::DoubleClick, treeview::Focus, treeview::DeleteItem,
    treeview::ItemChanged, treeview::ItemChanging, treeview::ItemExpanded, treeview::ItemExpanding`

    Members:  
        • position: The initial position of the control  
        • size: The inital size of the control  
        • visible: If the control should be visible  
        • disable: If the control should be disabled  
        • parent: The control parent          
*/
#[derive(Clone)]
pub struct TabViewT<ID: Hash+Clone> {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID,
    pub font: Option<ID>
}

impl<ID: Hash+Clone> ControlT<ID> for TabViewT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<TabView>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{handle_of_window, build_window, handle_of_font, set_window_font_raw, WindowParams};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD};

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a tabsview must be a window-like control.") {
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

        let flags: DWORD = WS_CHILD | 
        if self.visible    { WS_VISIBLE }   else { 0 } |
        if self.disabled   { WS_DISABLED }  else { 0 };

        let params = WindowParams {
            title: "",
            class_name: "SysTabControl32",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ set_window_font_raw(h, font_handle, true); }
                Ok( Box::new(TabView{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}


pub struct TabView {
    handle: HWND
}

/**
    A TabView control
*/
impl TabView {

    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
    pub fn update(&self) { unsafe{ ::low::window_helper::update(self.handle); } }
    pub fn focus(&self) { unsafe{ ::user32::SetFocus(self.handle); } }

}


impl Control for TabView {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::TabsView 
    }

    /*fn children(&self) -> Vec<AnyHandle> {
    }*/

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }
}


/**
    A templates that creates a tab in a tabview


*/
pub struct TabT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub parent: ID
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for TabT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<Tab>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        // Check if the parent handle is valid
        let view_handle = ui.handle_of(&self.parent);
        let view_handle = match view_handle {
            Ok(AnyHandle::HWND(h)) => {
                match ui.type_of_control(&self.parent) {
                    Ok(ControlType::TabsView) => { h },
                    Ok(t) => { return Err(Error::BadParent(format!("TabView parent required got \"{:?}\" control", t))); }
                    Err(e) => { return Err(e); }
                }
            },
            Ok(ref h) => { return Err(Error::BadParent(format!("TabViewparent required got \"{}\" control", h.human_name()))); },
            Err(ref e) => { return Err(e.clone()); }
        };


        let text = self.text.clone().into();
        match unsafe{ insert_tab(view_handle, text) } {
            Ok(_) => { 
                let tab = Tab{view: view_handle, handle: ::std::ptr::null_mut()};
                Ok( Box::new(tab) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A Tab in a TabView
*/
pub struct Tab {
    view: HWND,
    handle: HWND
}


impl Control for Tab {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(1 as HWND)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Tab 
    }

    fn children(&self) -> Vec<AnyHandle> {
        Vec::new()
    }

    fn free(&mut self) {
        /*use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };*/
    }
}

// Private functions

unsafe fn insert_tab(view: HWND, text: String) -> Result<(), SystemError> {
    use winapi::{TCM_INSERTITEMW, TCITEMW, LPARAM, TCIF_TEXT};
    use low::other_helper::to_utf16;

    let mut text = to_utf16(&text);

    let mut info = TCITEMW {
        mask: TCIF_TEXT,
        dwState: 0,
        dwStateMask: 0,
        pszText: text.as_mut_ptr(),
        cchTextMax: 0,
        iImage: -1,
        lParam: 0
    };

    let info_ptr: LPARAM = mem::transmute(&mut info);

    if SendMessageW(view, TCM_INSERTITEMW, 0, info_ptr) != -1 {
        Ok(())
    } else {
        Err(SystemError::SystemMessageFailed("Could insert tab in tabview".to_owned()))
    }
}
