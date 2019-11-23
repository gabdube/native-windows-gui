/*!
A tree-view control is a window that displays a hierarchical list of items
*/

use winapi::shared::minwindef::{WPARAM, LPARAM};
use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use winapi::um::commctrl::HTREEITEM;
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16};
use crate::{Font, SystemError};
use super::{ControlBase, ControlHandle};
use std::{mem, ptr};

const NOT_BOUND: &'static str = "TreeView is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TreeView handle is not HWND!";


bitflags! {
    pub struct TreeViewFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/// Select the position of a new item that is about to be inserted in a TreeView
#[derive(Copy, Clone, Debug)]
pub enum TreeInsert {
    /// Inserts the item at the beginning of the list. 
    First,

    /// Inserts the item at the end of the list. 
    Last,

    /// Add the item as a root item 
    Root,

    /// Inserts the item into the list in alphabetical order
    Sort,

    /// Insert the item after the choosen item
    After(HTREEITEM)
}

/// Possible state of a tree item regarding the "expanded/collapsed" state
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ExpandState {
    Collapse,
    CollapseReset,
    Expand,
    ExpandPartial,
    Toggle
}

/// A reference to an item in a TreeView
pub struct TreeItem {
    pub handle: HTREEITEM
}

/**
A tree-view control is a window that displays a hierarchical list of items
*/
#[derive(Default, Debug)]
pub struct TreeView {
    pub handle: ControlHandle
} 


impl TreeView {

    pub fn builder<'a>() -> TreeViewBuilder<'a> {
        TreeViewBuilder {
            size: (100, 200),
            position: (0, 0),
            enabled: true,
            flags: None,
            font: None,
            parent: None
        }
    }

    /// Return the root item of the tree view if one is present.
    /// If there is no root in the tree, returns `None`.
    pub fn root(&self) -> Option<TreeItem> {
        use winapi::um::commctrl::{TVM_GETNEXTITEM, TVGN_ROOT};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let handle = wh::send_message(handle, TVM_GETNEXTITEM, TVGN_ROOT, 0) as HTREEITEM;
        if handle.is_null() {
            None
        } else {
            Some(TreeItem { handle })
        }
    }

    /// Return the currently selected item.
    /// If there is no selected item, returns `None`.
    pub fn selected_item(&self) -> Option<TreeItem> {
        use winapi::um::commctrl::{TVM_GETNEXTITEM, TVGN_CARET};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let handle = wh::send_message(handle, TVM_GETNEXTITEM, TVGN_CARET, 0) as HTREEITEM;
        if handle.is_null() {
            None
        } else {
            Some(TreeItem { handle })
        }
    }

    /// Insert a new item into the TreeView and return a reference to new newly added item
    pub fn insert_item<'a>(&self, new: &'a str, parent: Option<TreeItem>, position: TreeInsert) -> TreeItem {
        use winapi::um::commctrl::{TVM_INSERTITEMW, TVINSERTSTRUCTW, TVI_FIRST, TVI_LAST, TVI_ROOT, TVI_SORT, TVIF_TEXT};
        use winapi::um::commctrl::TVINSERTSTRUCTW_u;
        use winapi::um::winnt::LPWSTR;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let insert = match position {
            TreeInsert::First => TVI_FIRST,
            TreeInsert::Last => TVI_LAST,
            TreeInsert::Root => TVI_ROOT,
            TreeInsert::Sort => TVI_SORT,
            TreeInsert::After(i) => i
        };

        let text = to_utf16(new);

        let item = {
            let mut item: TVINSERTSTRUCTW_u = unsafe { mem::zeroed() };
            let i = unsafe { item.item_mut() };
            i.mask = TVIF_TEXT;
            i.pszText = text.as_ptr() as LPWSTR;
            item
        };

        let new_item = TVINSERTSTRUCTW {
            hParent: parent.map(|p| p.handle ).unwrap_or(ptr::null_mut()),
            hInsertAfter: insert,
            u: item
        };

        let ptr = &new_item as *const TVINSERTSTRUCTW;
        let handle = wh::send_message(handle, TVM_INSERTITEMW, 0, ptr as LPARAM) as HTREEITEM;

        TreeItem { handle }
    }

    /// Remove an item and its children from the tree view
    pub fn remove_item(&self, item: &TreeItem) {
        use winapi::um::commctrl::{TVM_DELETEITEM};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TVM_DELETEITEM, 0, item.handle as LPARAM);
    }

    /// Selects the specified tree-view item and scrolls the item into view.
    pub fn select_item(&self, item: TreeItem) {
        use winapi::um::commctrl::{TVITEMW, TVM_SETITEMW, TVIS_SELECTED, TVIF_STATE};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE); 

        let mut item = TVITEMW {
            mask: TVIF_STATE,
            hItem: item.handle,
            state: TVIS_SELECTED,
            stateMask: TVIS_SELECTED,
            pszText: ptr::null_mut(),
            cchTextMax: 0,
            iImage: 0,
            iSelectedImage: 0,
            cChildren: 0,
            lParam: 0
        };

        wh::send_message(handle, TVM_SETITEMW, 0, &mut item as *mut TVITEMW as LPARAM);
    }

    /// Expands or collapses the list of child items associated with the specified parent item, if any. 
    pub fn set_expand_state(&self, item: TreeItem, state: ExpandState) {
        use winapi::um::commctrl::{TVM_EXPAND, TVE_COLLAPSE, TVE_COLLAPSERESET, TVE_EXPAND, TVE_EXPANDPARTIAL, TVE_TOGGLE};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let state = match state {
            ExpandState::Collapse => TVE_COLLAPSE,
            ExpandState::CollapseReset => TVE_COLLAPSE | TVE_COLLAPSERESET,
            ExpandState::Expand => TVE_EXPAND,
            ExpandState::ExpandPartial => TVE_EXPANDPARTIAL,
            ExpandState::Toggle => TVE_TOGGLE,
        };

        wh::send_message(handle, TVM_EXPAND, state as WPARAM, item.handle as LPARAM);
    }

    /// Ensures that a tree-view item is visible, expanding the parent item or scrolling the tree-view control, if necessary.
    pub fn ensure_visible(&self, item: TreeItem) {
        use winapi::um::commctrl::{TVM_ENSUREVISIBLE};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TVM_ENSUREVISIBLE, 0, item.handle as LPARAM);
    }

    /// Remove every item from the treeview by removing the root item
    pub fn clear(&self) {
        use winapi::um::commctrl::{TVM_DELETEITEM, TVI_ROOT};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TVM_DELETEITEM, 0, TVI_ROOT  as LPARAM);
    }

    /// Return the total number of item in the tree view
    pub fn len(&self) -> usize {
        use winapi::um::commctrl::{TVM_GETCOUNT};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TVM_GETCOUNT, 0, 0) as usize
    }

    /// Return the number of item in the tree view visible by the user
    pub fn visible_len(&self) -> usize {
        use winapi::um::commctrl::{TVM_GETVISIBLECOUNT};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TVM_GETVISIBLECOUNT, 0, 0) as usize
    }


    //
    // Common methods
    //

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button.
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        use winapi::um::commctrl::WC_TREEVIEW;
        Some(WC_TREEVIEW)
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        use winapi::um::commctrl::{TVS_HASBUTTONS, TVS_LINESATROOT, TVS_HASLINES};

        WS_VISIBLE | TVS_HASBUTTONS | TVS_LINESATROOT | TVS_HASLINES
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD, WS_BORDER};

        WS_CHILD | WS_BORDER
    }
}

pub struct TreeViewBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    flags: Option<TreeViewFlags>,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}


impl<'a> TreeViewBuilder<'a> {

    pub fn flags(mut self, flags: TreeViewFlags) -> TreeViewBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> TreeViewBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> TreeViewBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn enabled(mut self, e: bool) -> TreeViewBuilder<'a> {
        self.enabled = e;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> TreeViewBuilder<'a> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TreeViewBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut TreeView) -> Result<(), SystemError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(SystemError::ControlWithoutParent)
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        if self.font.is_some() {
            out.set_font(self.font);
        }

        out.set_enabled(self.enabled);

        Ok(())
    }

}


impl PartialEq for TreeItem {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for TreeItem {}
