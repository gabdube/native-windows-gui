use crate::win32::menu as mh;
use crate::SystemError;
use super::{ControlBase, ControlHandle};
use std::ptr;

const NOT_BOUND: &'static str = "Menu/MenuItem is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Menu/MenuItem handle is not HMENU!";


/// A windows menu. Can represent a menu in a window menubar or a submenu in another menu
#[derive(Default, Debug)]
pub struct Menu {
    pub handle: ControlHandle
}

impl Menu {

    pub fn builder<'a>() -> MenuBuilder<'a> {
        MenuBuilder {
            text: "Menu",
            disabled: false,
            popup: false,
            parent: None
        }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let (parent_handle, handle) = match self.handle {
            ControlHandle::Menu(parent, menu) => (parent, menu),
            ControlHandle::PopMenu(_, _) => { return true; },
            _ => panic!(BAD_HANDLE)
        };

        unsafe { mh::is_menu_enabled(parent_handle, handle) }
    }

    /// Enable or disable the control
    /// A popup menu cannot be disabled
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let (parent_handle, handle) = match self.handle {
            ControlHandle::Menu(parent, menu) => (parent, menu),
            ControlHandle::PopMenu(_, _) => { return; },
            _ => panic!(BAD_HANDLE)
        };

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

pub struct MenuBuilder<'a> {
    text: &'a str,
    disabled: bool,
    popup: bool,
    parent: Option<ControlHandle>
}

impl<'a> MenuBuilder<'a> {

    pub fn text(mut self, text: &'a str) -> MenuBuilder<'a> {
        self.text = text;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> MenuBuilder<'a> {
        self.disabled = disabled;
        self
    }

    pub fn popup(mut self, popup: bool) -> MenuBuilder<'a> {
        self.popup = popup;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> MenuBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, menu: &mut Menu) -> Result<(), SystemError> {
        if self.parent.is_none() {
            return Err(SystemError::ControlWithoutParent);
        }

        menu.handle = ControlBase::build_hmenu()
            .text(self.text)
            .item(false)
            .popup(self.popup)
            .parent(self.parent.unwrap())
            .build()?;

        if self.disabled {
            menu.set_enabled(false)
        }

        Ok(())
    }
}


/// A windows menu item. Can be added to a menubar or another menu
#[derive(Default, Debug)]
pub struct MenuItem {
    pub handle: ControlHandle
}

impl MenuItem {

    pub fn builder<'a>() -> MenuItemBuilder<'a> {
        MenuItemBuilder {
            text: "Menu Item",
            disabled: false,
            parent: None
        }
    }

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

pub struct MenuItemBuilder<'a> {
    text: &'a str,
    disabled: bool,
    parent: Option<ControlHandle>
}

impl<'a> MenuItemBuilder<'a> {

    pub fn text(mut self, text: &'a str) -> MenuItemBuilder<'a> {
        self.text = text;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> MenuItemBuilder<'a> {
        self.disabled = disabled;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> MenuItemBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, item: &mut MenuItem) -> Result<(), SystemError> {
        if self.parent.is_none() {
            return Err(SystemError::ControlWithoutParent);
        }

        item.handle = ControlBase::build_hmenu()
            .text(self.text)
            .item(true)
            .parent(self.parent.unwrap())
            .build()?;

        if self.disabled {
            item.set_enabled(false)
        }

        Ok(())
    }
}


/// A menu separator. Cannot be added to a menubar
#[derive(Default, Debug)]
pub struct MenuSeparator {
    pub handle: ControlHandle
}

impl MenuSeparator {

    pub fn builder() -> MenuSeparatorBuilder {
        MenuSeparatorBuilder {
            parent: None
        }
    }

}

pub struct MenuSeparatorBuilder {
    parent: Option<ControlHandle>
}

impl MenuSeparatorBuilder {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> MenuSeparatorBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, sep: &mut MenuSeparator) -> Result<(), SystemError> {
        if self.parent.is_none() {
            return Err(SystemError::ControlWithoutParent);
        }

        sep.handle = ControlBase::build_hmenu()
            .separator(true)
            .parent(self.parent.unwrap())
            .build()?;

        Ok(())
    }
}

