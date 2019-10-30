use crate::controls::ControlHandle;
use crate::win32::menu as mh;
use std::ptr;

const NOT_BOUND: &'static str = "Menu/MenuItem is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Menu/MenuItem handle is not HMENU!";


/// A windows menu. Can represent a menu in a window menubar or a submenu in another menu
#[derive(Default, Debug)]
pub struct Menu {
    pub handle: ControlHandle
}

impl Menu {

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let (parent_handle, handle) = self.handle.hmenu().expect(BAD_HANDLE);
        unsafe { mh::is_menu_enabled(parent_handle, handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let (parent_handle, handle) = self.handle.hmenu().expect(BAD_HANDLE);
        unsafe { mh::enable_menu(parent_handle, handle, v); }
    }

    /// Show a popup menu as the selected position. Do nothing for menubar menu.
    pub fn popup(&self, x: i32, y: i32) {
        use winapi::um::winuser::TrackPopupMenu;
        use winapi::ctypes::c_int;

        if self.handle.blank() { panic!("Menu is not bound"); }
        let (parent_handle, handle) = match self.handle.pop_hmenu() {
            Some(v) => v,
            None => { return; }
        };

        unsafe { 
            TrackPopupMenu(
                handle,
                0,
                x as c_int,
                y as c_int,
                0,
                parent_handle,
                ptr::null()
            );
        }
    }

}

/// A windows menu item. Can be added to a menubar or another menu
#[derive(Default, Debug)]
pub struct MenuItem {
    pub handle: ControlHandle
}

impl MenuItem {

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let (parent_handle, id) = self.handle.hmenu_item().expect(BAD_HANDLE);
        
        unsafe { mh::is_menuitem_enabled(parent_handle, None, Some(id)) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let (parent_handle, id) = self.handle.hmenu_item().expect(BAD_HANDLE);

        unsafe { mh::enable_menuitem(parent_handle, None, Some(id), v); }
    }

}


/// A menu separator. Cannot be added to a menubar
#[derive(Default, Debug)]
pub struct MenuSeparator {
    pub handle: ControlHandle
}
