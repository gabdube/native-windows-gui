/**
    A treeview control
*/

use std::hash::Hash;
use std::any::TypeId;
use std::ptr;
use std::mem;

use winapi::{c_int, HWND, UINT, HTREEITEM, WPARAM, LPARAM};
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
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD, WS_BORDER, TVS_HASLINES, TVS_HASBUTTONS, TVS_LINESATROOT};

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a treeview must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        let flags: DWORD = WS_CHILD | WS_BORDER | TVS_HASLINES | TVS_HASBUTTONS | TVS_LINESATROOT |
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

    /**
        Return the ID of the currently selected item in the treeview. If none are selected, return `None`.

        Arguments:
            • ui: The Ui object containing the item and the treeview  
    */
    pub fn get_selected_item<ID: Hash+Clone+'static>(&self, ui: &Ui<ID>) -> Option<ID> {
        use winapi::{TVM_GETNEXTITEM, TVGN_CARET};
        let selected_item = unsafe{ SendMessageW(self.handle, TVM_GETNEXTITEM, TVGN_CARET, 0) as HTREEITEM };
        if selected_item.is_null() {
            None
        } else {
            let handle = AnyHandle::HTREE_ITEM(selected_item, self.handle);
            match ui.id_from_handle(&handle) {
                Ok(id) => Some(id),
                Err(_) => None
            }
        }
    }

    /**
        Set the selected item in the treeview. If item is null, remove any active item in the tree view.

        Arguments:
            • ui: The Ui object containing the item and the treeview  
            • item: A reference to the item to set
    */
    pub fn set_selected_item<ID: Hash+Clone+'static>(&self, ui: &Ui<ID>, item: Option<&ID>) -> Result<(), Error> {
        use winapi::{TVM_SELECTITEM, TVGN_CARET};

        if !ui.has_handle(&self.handle()) {
            return Err(Error::BadUi("Tree control must be in the same Ui.".to_string()));
        }

        let item_handle = if let Some(id) = item {
            match ui.handle_of(id) {
                Ok(AnyHandle::HTREE_ITEM(item, _)) => item,
                Ok(h) => { return Err(Error::BadResource(format!("An TreeItem control is required, got {:?}", h))) },
                Err(e) => { return Err(e); }
            }
        } else {
            ptr::null_mut()
        };

        unsafe{ SendMessageW(self.handle, TVM_SELECTITEM, TVGN_CARET, item_handle as LPARAM) };

        Ok(())
    }

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

impl Control for TreeView {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::TreeView 
    }

    fn children(&self) -> Vec<AnyHandle> {
        unsafe{ list_tree_children(self.handle) }
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
        let tree_handle: HWND;
        let parent_handle = ui.handle_of(&self.parent);
        
        // Check if the parent handle is valid
        match &parent_handle {
            &Ok(AnyHandle::HWND(_)) => {
                match ui.type_of_control(&self.parent) {
                    Ok(ControlType::TreeView) => { /* OK */ },
                    Ok(t) => { return Err(Error::BadParent(format!("TreeView or TreeViewItem parent required got \"{:?}\" control", t))); }
                    Err(e) => { return Err(e); }
                }
            },
            &Ok(AnyHandle::HTREE_ITEM(_, _)) => { /* OK */ },
            &Ok(ref h) => { return Err(Error::BadParent(format!("TreeView or TreeViewItem parent required got \"{}\" control", h.human_name()))); },
            &Err(ref e) => { return Err(e.clone()); }
        }

        // Build the insert information
        let mut insert = ItemOptions {
            tree: ptr::null_mut(), parent: ptr::null_mut(), item: ptr::null_mut(),
            text: Some(self.text.clone().into()),
            integral: None, has_children: false
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

    fn children(&self) -> Vec<AnyHandle> {
        unsafe{ list_tree_item_children(self.tree, self.handle) }
    }

    fn free(&mut self) {
        use winapi::TVM_DELETEITEM;

        unsafe{ SendMessageW(self.tree, TVM_DELETEITEM, 0, self.handle as LPARAM); }
    }
}


// Private functions / structures / enum
use winapi::{TVIF_TEXT, TVIF_INTEGRAL, TVIF_CHILDREN, TVIF_HANDLE};
use low::other_helper::to_utf16;

struct ItemOptions {
    tree: HWND,
    parent: HTREEITEM,
    item: HTREEITEM,
    text: Option<String>,
    integral: Option<c_int>,
    has_children: bool
}

impl ItemOptions {
    fn mask(&self) -> UINT {
        let mut mask: UINT = TVIF_CHILDREN;

        if self.text.is_some() { mask |= TVIF_TEXT; }
        if self.integral.is_some() { mask |= TVIF_INTEGRAL; }
        if !self.item.is_null() { mask |= TVIF_HANDLE; }

        mask
    }

}

#[allow(unused_variables)]
unsafe fn update_item(i: ItemOptions) {
    use winapi::{TVM_SETITEMW, TVITEMEXW};

    let mask = i.mask();
    let children = i.has_children as c_int;
    let integral = i.integral.unwrap_or(0); 
    let (text_ptr, text) = match &i.text {
        &Some(ref t) => {
            let mut text_raw = to_utf16(t);
            (text_raw.as_mut_ptr(), text_raw)
        },
        &None => (ptr::null_mut(), Vec::new())
    };

    let mut item = TVITEMEXW{
        mask: mask,
        hItem: i.item,
        state: 0,
        stateMask: 0,
        pszText: text_ptr,
        cchTextMax: 0,
        iImage: 0,
        iSelectedImage: 0,
        cChildren: children,
        lParam: 0,
        iIntegral: integral,
        uStateEx: 0,
        hwnd: ptr::null_mut(), iExpandedImage: 0, iReserved: 0    
    };

    SendMessageW(i.tree, TVM_SETITEMW, 0, mem::transmute(&mut item));
}

#[allow(unused_variables)]
unsafe fn insert_item(i: ItemOptions) -> Result<HTREEITEM, SystemError> {
    use winapi::{TVI_LAST, TVM_INSERTITEMW, TVINSERTSTRUCTW, TVITEMEXW, TVI_ROOT};

    let mask = i.mask();
    let parent = i.parent;
    let insert_loc = if parent.is_null() { TVI_ROOT } else { TVI_LAST };
    let integral = i.integral.unwrap_or(0);
    let (text_ptr, text) = match &i.text {
        &Some(ref t) => {
            let mut text_raw = to_utf16(t);
            (text_raw.as_mut_ptr(), text_raw)
        },
        &None => (ptr::null_mut(), Vec::new())
    };

    // If parent is not null, update the item to indicates that it has children
    if !parent.is_null() {
        let update = ItemOptions {
            tree: i.tree, parent: ptr::null_mut(), item: parent,
            text: None, integral: None, has_children: true
        };
        update_item(update);
    }

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
            hwnd: ptr::null_mut(), iExpandedImage: 0, iReserved: 0    
        }
    };

    let tree_item  = SendMessageW(i.tree, TVM_INSERTITEMW, 0, mem::transmute(&mut insert_data)) as HTREEITEM;
    if tree_item.is_null() {
        Err(SystemError::TreeItemCreation)
    } else {
        Ok(tree_item)
    }
}

/**
    Return a list of tree item handles
*/
unsafe fn list_tree_item_children(tree: HWND, item: HTREEITEM) -> Vec<AnyHandle> {
    use winapi::{TVM_GETNEXTITEM, TVGN_CHILD, TVGN_NEXT};

    let mut children = Vec::with_capacity(10);
    let mut child_item = SendMessageW(tree, TVM_GETNEXTITEM, TVGN_CHILD as WPARAM, item as LPARAM) as HTREEITEM;
    while !child_item.is_null() {
        children.push(AnyHandle::HTREE_ITEM(child_item, tree));
        children.append(&mut list_tree_item_children(tree, child_item));
        child_item = SendMessageW(tree, TVM_GETNEXTITEM, TVGN_NEXT as WPARAM, child_item as LPARAM) as HTREEITEM;
    }

    children
}

/**
    Return a list of of every treeitem handle in a tree
*/
unsafe fn list_tree_children(tree: HWND) -> Vec<AnyHandle> {
    use winapi::{TVM_GETNEXTITEM, TVGN_ROOT};

    let mut children = Vec::with_capacity(10);
    let root_item = SendMessageW(tree, TVM_GETNEXTITEM, TVGN_ROOT as WPARAM, 0) as HTREEITEM;
    children.push(AnyHandle::HTREE_ITEM(root_item, tree));
    children.append(&mut list_tree_item_children(tree, root_item));

    children
}