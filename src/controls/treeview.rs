/**
    A treeview control
*/

use std::hash::Hash;
use std::any::TypeId;

use winapi::{HWND};

use ui::Ui;
use error::Error;
use controls::{Control, ControlT, ControlType, AnyHandle};

/**
    A template that creates a treeview

    Treeview specific events:  
    ``

    Members:  

*/
#[derive(Clone)]
pub struct TreeViewT<ID: Hash+Clone> {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID
}

impl<ID: Hash+Clone> ControlT<ID> for TreeViewT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<TreeView>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{handle_of_window, build_window, WindowParams};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, WS_BORDER, TVS_HASLINES};

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a treeview must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        let flags: DWORD = WS_CHILD | WS_BORDER | TVS_HASLINES |
        if self.visible    { WS_VISIBLE }   else { 0 } |
        if self.disabled   { WS_DISABLED }  else { 0 };

        let params = WindowParams {
            title: "",
            class_name: "SysTreeView32",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                Ok( Box::new(TreeView{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}

/**
    A TreeView control
*/
pub struct TreeView {
    handle: HWND
}

impl Control for TreeView {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::TreeView 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }
}