/*!
A tree-view control is a window that displays a hierarchical list of items
*/

use winapi::shared::minwindef::{WPARAM, LPARAM};
use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_TABSTOP};
use winapi::um::commctrl::{HTREEITEM, TVIS_EXPANDED, TVIS_SELECTED, TVS_SHOWSELALWAYS, TVITEMW};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{check_hwnd, to_utf16, from_utf16};
use crate::{Font, NwgError};
use super::{ControlBase, ControlHandle};
use std::{mem, ptr};

#[cfg(feature="image-list")]
use winapi::um::commctrl::HIMAGELIST;

#[cfg(feature="image-list")]
use crate::ImageList;

const NOT_BOUND: &'static str = "TreeView is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TreeView handle is not HWND!";


bitflags! {
    /**
        The tree view flags

        * VISIBLE:  The tree view is immediatly visible after creation
        * DISABLED: The tree view cannot be interacted with by the user. It also has a grayed out look.
        * TAB_STOP: The tree view can be selected using tab navigation
    */
    pub struct TreeViewFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;
        const ALWAYS_SHOW_SELECTION = TVS_SHOWSELALWAYS;
    }
}

bitflags! {
    /**
        A tree item state

        * SELECTED:  The tree view is immediatly visible after creation
        * DISABLED: The tree view cannot be interacted with by the user. It also has a grayed out look.
        * TAB_STOP: The tree view can be selected using tab navigation
    */
    pub struct TreeItemState: u32 {
        const SELECTED = TVIS_SELECTED;
        const EXPANDED = TVIS_EXPANDED;
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


/// An action that can be applied to a tree item. Used in events
#[derive(Copy, Clone, Debug)]
pub enum TreeItemAction {
    /// An unexpected value was passed to NWG
    Unknown,

    /// A tree item was expanded or collapsed. 
    Expand(ExpandState),

    /// The state of the item was changed
    State { old: TreeItemState, new: TreeItemState }
}

/// A reference to an item in a TreeView
#[derive(Debug)]
pub struct TreeItem {
    pub handle: HTREEITEM
}

impl TreeItem {
    /// Checks if the inner handle is null
    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }
}

/**
A tree-view control is a window that displays a hierarchical list of items.

While a treeview can support selected multiple item programatically (using `select_item`), this is not fully supported
by the winapi implementation.

Requires the `tree-view` feature

**Builder parameters:**
  * `parent`:     **Required.** The tree-view parent container.
  * `position`:   The treeview position.
  * `enabled`:    If the treeview can be used by the user. It also has a grayed out look if disabled.
  * `focus`:      The control receive focus after being created
  * `flags`:      A combination of the `TreeViewFlags` values.
  * `ex_flags`:   A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `font`:       The font used for the treeview text
  * `parent`:     The treeview parent container.
  * `image_list`: Image list containing the icon to use in the tree-view

**Control events:**
  * `MousePress(_)`: Generic mouse press events on the tree view
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event
  * `OnTreeViewClick`: When the user has clicked the left mouse button within the control.
  * `OnTreeViewDoubleClick`: When the user has clicked the left mouse button within the control twice rapidly.
  * `OnTreeViewRightClick`: When the user has clicked the right mouse button within the control.
  * `OnTreeFocusLost`: When the control has lost the input focus
  * `OnTreeFocus`: When the control has acquired the input focus
  * `OnTreeItemDelete`: Just before an item is deleted. Also sent for all the children.
  * `OnTreeItemExpanded`: After an item was expanded or collapsed. Sends a `EventData::OnTreeItemUpdate`.
  * `OnTreeItemChanged`: After the state of an item was changed. Sends a `EventData::OnTreeItemUpdate`.
  * `OnTreeItemSelectionChanged`: After the current selection was changed. Sends a `EventData::OnTreeItemChanged`.
*/
#[derive(Default, PartialEq, Eq)]
pub struct TreeView {
    pub handle: ControlHandle
} 


impl TreeView {

    pub fn builder<'a>() -> TreeViewBuilder<'a> {
        TreeViewBuilder {
            size: (100, 200),
            position: (0, 0),
            enabled: true,
            focus: false,
            flags: None,
            ex_flags: 0,
            font: None,
            parent: None,

            #[cfg(feature="image-list")]
            image_list: None,
        }
    }

    /// Sets the image list of the treeview
    #[cfg(feature="image-list")]
    pub fn set_image_list(&self, list: Option<&ImageList>) {
        use winapi::um::commctrl::{TVM_SETIMAGELIST, TVSIL_NORMAL};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let list_handle = list.map(|l| l.handle).unwrap_or(ptr::null_mut());

        wh::send_message(handle, TVM_SETIMAGELIST, TVSIL_NORMAL, list_handle as _);
    }

    /// Returns the image list of the treeview or None if there is none.
    /// The returned image list is not owned
    #[cfg(feature="image-list")]
    pub fn image_list(&self) -> Option<ImageList> {
        use winapi::um::commctrl::{TVM_GETIMAGELIST, TVSIL_NORMAL};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let handle = wh::send_message(handle, TVM_GETIMAGELIST, TVSIL_NORMAL, 0) as HIMAGELIST;
        if handle.is_null() {
            None
        } else {
            Some(ImageList { handle, owned: false })
        }
    }

    /// Sets the image that will appear left to the item text. `index` is the index of the image in the image-list
    /// Won't do anything if the control do not have a image list or if the item is not in the tree
    /// If `on_select` is set to true, sets the icon that is used when an item is active
    #[cfg(feature="image-list")]
    pub fn set_item_image(&self, item: &TreeItem, index: i32, on_select: bool) {
        use winapi::um::commctrl::{TVM_SETITEMW, TVIF_IMAGE, TVIF_SELECTEDIMAGE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let mut tree_item = blank_item();
        tree_item.hItem = item.handle;

        tree_item.mask = match on_select {
            true => TVIF_SELECTEDIMAGE,
            false => TVIF_IMAGE
        };

        match on_select {
            true => { tree_item.iSelectedImage = index; },
            false => { tree_item.iImage = index; }
        }

        wh::send_message(handle, TVM_SETITEMW, 0, &mut tree_item as *mut TVITEMW as LPARAM);
    }

    /// Returns the index of the image in the tree view image list.
    /// If there is no image list in the control or the item is not in the control, 0 will be returned.
    /// If `on_select` is set to true, returns the icon that is used when an item is active
    #[cfg(feature="image-list")]
    pub fn item_image(&self, item: &TreeItem, on_select: bool) -> i32 {
        use winapi::um::commctrl::{TVM_GETITEMW, TVIF_IMAGE, TVIF_SELECTEDIMAGE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let mut tree_item = blank_item();
        tree_item.mask = TVIF_IMAGE | TVIF_SELECTEDIMAGE;
        tree_item.hItem = item.handle;

        match wh::send_message(handle, TVM_GETITEMW, 0, &mut tree_item as *mut TVITEMW as LPARAM) {
            0 => 0,
            _ => match on_select {
                true => tree_item.iSelectedImage,
                false => tree_item.iImage
            }
        }
    }

    /// Sets the text color in the treeview
    pub fn set_text_color(&self, r: u8, g: u8, b: u8) {
        use winapi::um::commctrl::TVM_SETTEXTCOLOR;
        use winapi::um::wingdi::RGB;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let color = RGB(r, g, b);

        wh::send_message(handle, TVM_SETTEXTCOLOR, 0, color as _);

        self.invalidate();
    }

    /// Returns the text color in the treeview
    pub fn text_color(&self) -> [u8; 3] {
        use winapi::um::commctrl::TVM_GETTEXTCOLOR;
        use winapi::um::wingdi::{GetRValue, GetGValue, GetBValue};
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let col = wh::send_message(handle, TVM_GETTEXTCOLOR, 0, 0) as u32;

        [
            GetRValue(col),
            GetGValue(col),
            GetBValue(col),
        ]
    }

    /// Retrieves the amount, in pixels, that child items are indented relative to their parent items. 
    pub fn indent(&self) -> u32 {
        use winapi::um::commctrl::TVM_GETINDENT;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TVM_GETINDENT, 0, 0) as u32
    }

    /// Sets the width of indentation for a tree-view control and redraws the control to reflect the new width.
    pub fn set_indent(&self, indent: u32) {
        use winapi::um::commctrl::TVM_SETINDENT;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TVM_SETINDENT, indent as _, 0);
    }

    /// Return the root item of the tree view if one is present.
    /// If there is no root in the tree, returns `None`.
    pub fn root(&self) -> Option<TreeItem> {
        use winapi::um::commctrl::TVGN_ROOT;
        next_treeview_item(&self.handle, TVGN_ROOT, ptr::null_mut())
    }

    /// Returns the first child of an item or `None` if the item has no child or if it's not part of the tree view 
    /// To iterate over all the children, use `TreeView.iter_item(&parent_item)`
    pub fn first_child(&self, item: &TreeItem) ->  Option<TreeItem> {
        use winapi::um::commctrl::TVGN_CHILD;
        next_treeview_item(&self.handle, TVGN_CHILD, item.handle)
    }

    /// Returns the next sibling in the tree or `None` if the item has no more sibling or if it's not part of the tree view 
    pub fn next_sibling(&self, item: &TreeItem) ->  Option<TreeItem> {
        use winapi::um::commctrl::TVGN_NEXT;
        next_treeview_item(&self.handle, TVGN_NEXT, item.handle)
    }

    /// Returns the previous sibling in the tree or `None` if the item has no more sibling or if it's not part of the tree view
    pub fn previous_sibling(&self, item: &TreeItem) -> Option<TreeItem> {
        use winapi::um::commctrl::TVGN_PREVIOUS;
        next_treeview_item(&self.handle, TVGN_PREVIOUS, item.handle)
    }

    /// Returns the parent of the item in the tree or `None` if the item is root
    pub fn parent(&self, item: &TreeItem) -> Option<TreeItem> {
        use winapi::um::commctrl::TVGN_PARENT;
        next_treeview_item(&self.handle, TVGN_PARENT, item.handle)
    }

    /// Return the currently selected item. If there are more than one selected item, returns the first one.
    /// If there is no selected item, returns `None`.
    pub fn selected_item(&self) -> Option<TreeItem> {
        use winapi::um::commctrl::{TVM_GETNEXTITEM, TVGN_NEXTSELECTED};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let tree_handle = wh::send_message(handle, TVM_GETNEXTITEM, TVGN_NEXTSELECTED, 0) as HTREEITEM;
        if tree_handle.is_null() {
            None
        } else {
            Some(TreeItem { handle: tree_handle })
        }
    }

    /// Returns the selected items in a Treeview
    /// If there is no selected items, returns an empty `Vec`.
    pub fn selected_items(&self) -> Vec<TreeItem> {
        use winapi::um::commctrl::{TVM_GETNEXTITEM, TVGN_NEXTSELECTED};

        let mut items = Vec::new();

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let mut last_handle = wh::send_message(handle, TVM_GETNEXTITEM, TVGN_NEXTSELECTED, 0);
        while last_handle != 0 {
            items.push(TreeItem { handle: last_handle as _ } );
            last_handle = wh::send_message(handle, TVM_GETNEXTITEM, TVGN_NEXTSELECTED, last_handle as _);
        }

        items
    }

    /// Returns the number of selected item in the tree view
    pub fn selected_item_count(&self) -> usize {
        use winapi::um::commctrl::{TVM_GETNEXTITEM, TVGN_NEXTSELECTED};

        let mut count = 0;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut last_handle = wh::send_message(handle, TVM_GETNEXTITEM, TVGN_NEXTSELECTED, 0);
        while last_handle != 0 {
            count += 1;
            last_handle = wh::send_message(handle, TVM_GETNEXTITEM, TVGN_NEXTSELECTED, last_handle as _);
        }

        count
    }

    /// Insert a new item into the TreeView and return a reference to new newly added item
    pub fn insert_item<'a>(&self, new: &'a str, parent: Option<&TreeItem>, position: TreeInsert) -> TreeItem {
        use winapi::um::commctrl::{TVM_INSERTITEMW, TVINSERTSTRUCTW, TVI_FIRST, TVI_LAST, TVI_ROOT, TVI_SORT, TVIF_TEXT};
        use winapi::um::commctrl::TVINSERTSTRUCTW_u;
        use winapi::um::winnt::LPWSTR;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

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

        self.invalidate();

        TreeItem { handle }
    }

    /// Insert a new item into the TreeView with associated lParam and return a reference to new newly added item
    pub fn insert_item_with_param<'a>(&self, new: &'a str, parent: Option<&TreeItem>, position: TreeInsert, data: isize) -> TreeItem {
        use winapi::um::commctrl::{TVM_INSERTITEMW, TVINSERTSTRUCTW, TVI_FIRST, TVI_LAST, TVI_ROOT, TVI_SORT, TVIF_TEXT, TVIF_PARAM};
        use winapi::um::commctrl::TVINSERTSTRUCTW_u;
        use winapi::um::winnt::LPWSTR;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

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
            i.mask = TVIF_TEXT | TVIF_PARAM;            
            i.pszText = text.as_ptr() as LPWSTR;
            i.lParam = data;
            item
        };

        let new_item = TVINSERTSTRUCTW {
            hParent: parent.map(|p| p.handle ).unwrap_or(ptr::null_mut()),
            hInsertAfter: insert,
            u: item
        };

        let ptr = &new_item as *const TVINSERTSTRUCTW;
        let handle = wh::send_message(handle, TVM_INSERTITEMW, 0, ptr as LPARAM) as HTREEITEM;

        self.invalidate();

        TreeItem { handle }
    }

    /// Remove an item and its children from the tree view
    pub fn remove_item(&self, item: &TreeItem) {
        use winapi::um::commctrl::{TVM_DELETEITEM};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TVM_DELETEITEM, 0, item.handle as LPARAM);
    }

    /// Selects the specified tree-view item and scrolls the item into view.
    pub fn select_item(&self, item: &TreeItem) {
        use winapi::um::commctrl::{TVM_SETITEMW, TVIF_STATE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let mut tree_item = blank_item();
        tree_item.mask = TVIF_STATE;
        tree_item.hItem = item.handle;
        tree_item.state = TVIS_SELECTED;
        tree_item.stateMask = TVIS_SELECTED;

        wh::send_message(handle, TVM_SETITEMW, 0, &mut tree_item as *mut TVITEMW as LPARAM);
    }

    /// Unselects an item from the treeview
    pub fn unselect_item(&self, item: &TreeItem) {
        use winapi::um::commctrl::{TVM_SETITEMW, TVIF_STATE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let mut tree_item = blank_item();
        tree_item.mask = TVIF_STATE;
        tree_item.hItem = item.handle;
        tree_item.state = 0;
        tree_item.stateMask = TVIS_SELECTED;

        wh::send_message(handle, TVM_SETITEMW, 0, &mut tree_item as *mut TVITEMW as LPARAM);
    }

    /// Creates an iterator over the tree view items
    #[cfg(feature="tree-view-iterator")]
    pub fn iter<'a>(&'a self) -> crate::TreeViewIterator<'a> {
        check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        crate::TreeViewIterator::new(self, ptr::null_mut())
    }

    /// Creates an iterator over the children of an item. This does not include the item itself.
    #[cfg(feature="tree-view-iterator")]
    pub fn iter_item<'a>(&'a self, item: &TreeItem) -> crate::TreeViewIterator<'a> {
        check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        crate::TreeViewIterator::new(self, item.handle)
    }

    /// Returns the text of the selected item. Return None if the item is not in the tree view.
    /// The returned text value cannot be bigger than 260 characters
    pub fn item_text(&self, tree_item: &TreeItem) -> Option<String> {
        use winapi::um::commctrl::{TVM_GETITEMW, TVIF_TEXT, TVIF_HANDLE};
        const BUFFER_MAX: usize = 260;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut text_buffer = Vec::with_capacity(BUFFER_MAX);
        unsafe { text_buffer.set_len(BUFFER_MAX); }

        let mut item: TVITEMW = blank_item();
        item.mask = TVIF_TEXT | TVIF_HANDLE;
        item.hItem = tree_item.handle;
        item.pszText = text_buffer.as_mut_ptr();
        item.cchTextMax = BUFFER_MAX as _;
        
        let result = wh::send_message(handle, TVM_GETITEMW, 0, &mut item as *mut TVITEMW as LPARAM);
        if result == 0 {
            return None;
        }

        Some(from_utf16(&text_buffer))
    }
    
    /// Set the text for specified item in the treeview.
    pub fn set_item_text(&self, tree_item: &TreeItem, new_text: &str) {
        use winapi::um::commctrl::{TVM_SETITEMW, TVIF_TEXT};
        use winapi::um::winnt::LPWSTR;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let text = to_utf16(new_text);

        let mut item: TVITEMW = blank_item();
        item.mask = TVIF_TEXT;
        item.hItem = tree_item.handle;
        item.pszText = text.as_ptr() as LPWSTR;

        wh::send_message(handle, TVM_SETITEMW, 0, &mut item as *mut TVITEMW as LPARAM);
    }

    /// Returns the lParam of the selected item. Return None if the item is not in the tree view.
    pub fn item_param(&self, tree_item: &TreeItem) -> Option<isize> {
        use winapi::um::commctrl::{TVM_GETITEMW, TVIF_PARAM, TVIF_HANDLE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut item: TVITEMW = blank_item();
        item.mask = TVIF_HANDLE | TVIF_PARAM;
        item.hItem = tree_item.handle;
        
        let result = wh::send_message(handle, TVM_GETITEMW, 0, &mut item as *mut TVITEMW as LPARAM);
        if result == 0 {
            return None;
        }

        Some(item.lParam)
    }

    /// Returns `true` if the tree view item has children. Returns `None` if the item is not in the tree view.
    pub fn item_has_children(&self, tree_item: &TreeItem) -> Option<bool> {
        use winapi::um::commctrl::{TVM_GETITEMW, TVIF_CHILDREN, TVIF_HANDLE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut item: TVITEMW = blank_item();
        item.hItem = tree_item.handle;
        item.mask = TVIF_CHILDREN | TVIF_HANDLE;
        
        let result = wh::send_message(handle, TVM_GETITEMW, 0, &mut item as *mut TVITEMW as LPARAM);
        if result == 0 {
            return None;
        }

        Some(item.cChildren != 0)
    }

    /// Returns the item state in the tree view or `None` if the item is not in the tree view
    pub fn item_state(&self, tree_item: &TreeItem) -> Option<TreeItemState> {
        use winapi::um::commctrl::{TVM_GETITEMW, TVIF_STATE, TVIF_HANDLE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut item: TVITEMW = unsafe { mem::zeroed() };
        item.hItem = tree_item.handle;
        item.mask = TVIF_STATE | TVIF_HANDLE;
        item.stateMask = 0xFF;
        
        let result = wh::send_message(handle, TVM_GETITEMW, 0, &mut item as *mut TVITEMW as LPARAM);
        if result == 0 {
            return None;
        }

        Some(TreeItemState::from_bits_truncate(item.state))
    }

    /// Expands or collapses the list of child items associated with the specified parent item, if any. 
    pub fn set_expand_state(&self, item: &TreeItem, state: ExpandState) {
        use winapi::um::commctrl::{TVM_EXPAND, TVE_COLLAPSE, TVE_COLLAPSERESET, TVE_EXPAND, TVE_EXPANDPARTIAL, TVE_TOGGLE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

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
    pub fn ensure_visible(&self, item: &TreeItem) {
        use winapi::um::commctrl::{TVM_ENSUREVISIBLE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TVM_ENSUREVISIBLE, 0, item.handle as LPARAM);
    }

    /// Remove every item from the treeview by removing the root item
    pub fn clear(&self) {
        use winapi::um::commctrl::{TVM_DELETEITEM, TVI_ROOT};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TVM_DELETEITEM, 0, TVI_ROOT  as LPARAM);
    }

    /// Return the total number of item in the tree view
    pub fn len(&self) -> usize {
        use winapi::um::commctrl::TVM_GETCOUNT;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TVM_GETCOUNT, 0, 0) as usize
    }

    /// Return the number of item in the tree view visible by the user
    pub fn visible_len(&self) -> usize {
        use winapi::um::commctrl::TVM_GETVISIBLECOUNT;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TVM_GETVISIBLECOUNT, 0, 0) as usize
    }


    //
    // Common methods
    //

    /// Invalidate the whole drawing region.
    pub fn invalidate(&self) {
        use winapi::um::winuser::InvalidateRect;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { InvalidateRect(handle, ptr::null(), 1); }
    }


    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button.
    pub fn set_focus(&self) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        winapi::um::commctrl::WC_TREEVIEW
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        use winapi::um::commctrl::{TVS_HASBUTTONS, TVS_LINESATROOT, TVS_HASLINES, TVS_EDITLABELS};

        WS_VISIBLE | TVS_HASBUTTONS | TVS_LINESATROOT | TVS_HASLINES | WS_TABSTOP | TVS_SHOWSELALWAYS | TVS_EDITLABELS
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD, WS_BORDER};
        use winapi::um::commctrl::TVS_NOTOOLTIPS;

        WS_CHILD | WS_BORDER | TVS_NOTOOLTIPS
    }

    /// Begins to in-place edit the specified item's text.
    /// Return None if Failed.
    /// Return the treeview's handle if successful.
    pub fn edit_label(&self, item: &TreeItem) -> Option<ControlHandle> {
        use winapi::um::commctrl::TVM_EDITLABELW;
        use winapi::shared::windef::HWND;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
    
        let result = wh::send_message(handle,  TVM_EDITLABELW, 0, item.handle as HTREEITEM as LPARAM); 
        
        if result == 0 {
            return None;
        }
        Some(ControlHandle::Hwnd(result as HWND))
    } 

    /// End the in-place editing of the tree item's label.
    /// The parameter f_cancel indicates whether the editing is canceled without being saved to the label. 
    /// If this parameter is TRUE, the system cancels editing without saving the changes. Otherwise, the system saves the changes to the label.
    /// Return true if successful, otherwise return false.
    pub fn end_edit_label_now(&self, f_cancel: bool) -> bool {
        use winapi::um::commctrl::TVM_ENDEDITLABELNOW;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
    
        wh::send_message(handle,  TVM_ENDEDITLABELNOW, f_cancel as WPARAM, 0) != 0
    }
}

impl Drop for TreeView {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}


/// Builder for a TreeView
pub struct TreeViewBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    focus: bool,
    flags: Option<TreeViewFlags>,
    ex_flags: u32,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>,

    #[cfg(feature="image-list")]
    image_list: Option<&'a ImageList>,
}


impl<'a> TreeViewBuilder<'a> {

    pub fn flags(mut self, flags: TreeViewFlags) -> TreeViewBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> TreeViewBuilder<'a> {
        self.ex_flags = flags;
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

    pub fn focus(mut self, focus: bool) -> TreeViewBuilder<'a> {
        self.focus = focus;
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

    #[cfg(feature="image-list")]
    pub fn image_list(mut self, list: Option<&'a ImageList>) -> TreeViewBuilder<'a> {
        self.image_list = list;
        self
    }

    pub fn build(self, out: &mut TreeView) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("TreeView"))
        }?;

        *out = Default::default();

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .ex_flags(self.ex_flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        if self.font.is_some() {
            out.set_font(self.font);
        } else {
            out.set_font(Font::global_default().as_ref());
        }

        builder_set_image_list(&self, out);

        if self.focus {
            out.set_focus();
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


fn next_treeview_item(handle: &ControlHandle, action: usize, item: HTREEITEM) -> Option<TreeItem> {
    use winapi::um::commctrl::TVM_GETNEXTITEM;

    if handle.blank() { panic!("{}", NOT_BOUND); }
    let handle = handle.hwnd().expect(BAD_HANDLE);

    let handle = wh::send_message(handle, TVM_GETNEXTITEM, action as _, item as _) as HTREEITEM;
    if handle.is_null() {
        None
    } else {
        Some(TreeItem { handle })
    }
}

#[cfg(feature="image-list")]
fn builder_set_image_list(builder: &TreeViewBuilder, out: &TreeView) {
    if builder.image_list.is_some() {
        out.set_image_list(builder.image_list);
    }
}

#[cfg(not(feature="image-list"))]
fn builder_set_image_list(_builder: &TreeViewBuilder, _out: &TreeView) {
}

fn blank_item() -> TVITEMW {
    TVITEMW {
        mask: 0,
        hItem: ptr::null_mut(),
        state: 0,
        stateMask: 0,
        pszText: ptr::null_mut(),
        cchTextMax: 0,
        iImage: 0,
        iSelectedImage: 0,
        cChildren: 0,
        lParam: 0
    }
}
