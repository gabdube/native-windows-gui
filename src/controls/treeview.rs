/**
    A treeview control
*/

use std::hash::Hash;
use std::any::TypeId;
use std::ptr;
use std::mem;

use winapi::{c_int, HWND, UINT, HTREEITEM};
use user32::SendMessageW;

use ui::Ui;
use error::{Error, SystemError};
use controls::{Control, ControlT, ControlType, AnyHandle};

/**
    A template that creates a treeview

    Treeview specific events:  
    ``

    Members:  
        • position: The initial position of the control  
        • size: The inital size of the control  
        • visible: If the control should be visible  
        • disable: If the control should be disabled  
        • parent: The control parent          
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

impl TreeView {

    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, false); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
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

/**
    A template that creates a treeview item

    Treeview item specific events:  
    ``

    Members:  
        • text: A nwg::Tree collection    
        • parent: The TreeView or TreeViewItem parent    
*/
#[derive(Clone)]
pub struct TreeViewItemT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub parent: ID
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for TreeViewItemT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<TreeViewItem>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        let parent_handle = ui.handle_of(&self.parent);
        let tree_handle: HWND;
        
        // Check if the parent handle is valid
        match &parent_handle {
            &Ok(AnyHandle::HWND(_)) | &Ok(AnyHandle::HTREE_ITEM(_, _)) => { /* OK */ },
            &Ok(ref h) => { return Err(Error::BadParent(format!("TreeView or TreeViewItem parent required got \"{}\" control", h.human_name()))); },
            &Err(ref e) => { return Err(e.clone()); }
        }

        // Build the insert information
        let mut insert = InsertItemOptions {
            tree: ptr::null_mut(),
            parent: ptr::null_mut(),
            text: Some(self.text.clone().into()),
            integral: None
        };

        // Create the item
        let tree_item = match parent_handle.unwrap() {
            AnyHandle::HWND(h) => {
                tree_handle = h;
                insert.tree = h;
                unsafe{ insert_item(insert) }
            },
            AnyHandle::HTREE_ITEM(h, tree) => {
                tree_handle=tree;
                insert.tree = tree;
                insert.parent = h;
                unsafe{ insert_item(insert) }
            },
            _ => { unreachable!() }
        };

        match tree_item {
            Ok(h) => {
                Ok( Box::new(TreeViewItem{handle: h, tree: tree_handle}) )
            },
            Err(e) => { Err(Error::System(e)) }
        }
    }
}

pub struct TreeViewItem {
    tree: HWND,
    handle: HTREEITEM
}

impl Control for TreeViewItem {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HTREE_ITEM(self.handle, self.tree)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::TreeViewItem 
    }

    fn free(&mut self) {
        
    }
}


// Private functions / structures / enum
use winapi::{TVIF_TEXT, TVIF_INTEGRAL};

struct InsertItemOptions {
    tree: HWND,
    parent: HTREEITEM,
    text: Option<String>,
    integral: Option<c_int>
}

impl InsertItemOptions {
    fn mask(&self) -> UINT {
        let mut mask: UINT = 0;

        if self.text.is_some() { mask |= TVIF_TEXT; }
        if self.integral.is_some() { mask |= TVIF_INTEGRAL; }

        mask
    }

}

#[allow(unused_variables)]
unsafe fn insert_item(i: InsertItemOptions) -> Result<HTREEITEM, SystemError> {
    use winapi::{TVI_LAST, TVM_INSERTITEMW, TVINSERTSTRUCTW, TVITEMEXW, TVI_ROOT};
    use low::other_helper::to_utf16;

    let mask = i.mask();
    let parent = i.parent;
    let insert_loc = if parent.is_null() { TVI_ROOT } else { TVI_LAST };
    let integral = i.integral.unwrap_or(1);
    let (text_ptr, text) = match &i.text {
        &Some(ref t) => {
            let mut text_raw = to_utf16(t);
            (text_raw.as_mut_ptr(), text_raw)
        },
        &None => (ptr::null_mut(), Vec::new())
    };

    let mut insert_data = TVINSERTSTRUCTW {
        hParent: parent,
        hInsertAfter: insert_loc,
        itemex: TVITEMEXW{
            mask: mask,
            hItem: ptr::null_mut(),
            state: 0,
            stateMask: 0,
            pszText: text_ptr,
            cchTextMax: 0,
            iImage: 0,
            iSelectedImage: 0,
            cChildren: 0,
            lParam: 0,
            iIntegral: integral,
            uStateEx: 0,
            hwnd: ptr::null_mut(),
            iExpandedImage: 0,
            iReserved: 0    
        }
    };

    let tree_item  = SendMessageW(i.tree, TVM_INSERTITEMW, 0, mem::transmute(&mut insert_data)) as HTREEITEM;
    if tree_item.is_null() {
        Err(SystemError::TreeItemCreation)
    } else {
        Ok(tree_item)
    }
}
