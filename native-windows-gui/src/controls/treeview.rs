/*!
A tree-view control is a window that displays a hierarchical list of items
*/

use winapi::shared::minwindef::{WPARAM, LPARAM};
use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_TABSTOP};
use winapi::um::commctrl::{HIMAGELIST, HTREEITEM, TVIS_EXPANDED, TVIS_SELECTED};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::{Font, ImageList, NwgError};
use super::{ControlBase, ControlHandle};
use std::{mem, ptr};

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
A tree-view control is a window that displays a hierarchical list of items

Requires the `tree-view` feature

**Builder parameters:**
  * `parent`:     **Required.** The tree-view parent container.
  * `position`:   The treeview position.
  * `enabled`:    If the treeview can be used by the user. It also has a grayed out look if disabled.
  * `focus`:      The control receive focus after being created
  * `flags`:      A combination of the `TreeViewFlags` values.
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
            font: None,
            parent: None,
            image_list: None,
        }
    }

    /// Sets the image list of the treeview
    pub fn set_image_list(&self, list: Option<&ImageList>) {
        use winapi::um::commctrl::{TVM_SETIMAGELIST, TVSIL_NORMAL};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let list_handle = list.map(|l| l.handle).unwrap_or(ptr::null_mut());

        wh::send_message(handle, TVM_SETIMAGELIST, TVSIL_NORMAL, list_handle as _);
    }

    /// Returns the image list of the treeview or None if there is none.
    /// The returned image list is not owned
    pub fn image_list(&self) -> Option<ImageList> {
        use winapi::um::commctrl::{TVM_GETIMAGELIST, TVSIL_NORMAL};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let handle = wh::send_message(handle, TVM_GETIMAGELIST, TVSIL_NORMAL, 0) as HIMAGELIST;
        if handle.is_null() {
            None
        } else {
            Some(ImageList { handle, owned: false })
        }
    }

    /// Retrieves the amount, in pixels, that child items are indented relative to their parent items. 
    pub fn indent(&self) -> u32 {
        use winapi::um::commctrl::TVM_GETINDENT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TVM_GETINDENT, 0, 0) as u32
    }

    /// Sets the width of indentation for a tree-view control and redraws the control to reflect the new width.
    pub fn set_indent(&self, indent: u32) {
        use winapi::um::commctrl::TVM_SETINDENT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TVM_SETINDENT, indent as _, 0);
    }

    /// Return the root item of the tree view if one is present.
    /// If there is no root in the tree, returns `None`.
    pub fn root(&self) -> Option<TreeItem> {
        use winapi::um::commctrl::TVGN_ROOT;
        next_treeview_item(&self.handle, TVGN_ROOT, ptr::null_mut())
    }

    /// Returns the first child of an item or `None` if the item has no child or if it's not part of the tree view 
    /// To iterate over all the children, use `TTreeView.iter_item(&parent_item)`
    pub fn first_child(&self, item: &TreeItem) ->  Option<TreeItem> {
        use winapi::um::commctrl::TVGN_CHILD;
        next_treeview_item(&self.handle, TVGN_CHILD, item.handle)
    }

    /// Return the next sibling in the tree or `None` if the item has no more sibling or if it's not part of the tree view 
    pub fn next_sibling(&self, item: &TreeItem) ->  Option<TreeItem> {
        use winapi::um::commctrl::TVGN_NEXT;
        next_treeview_item(&self.handle, TVGN_NEXT, item.handle)
    }

    /// Return the previous sibling in the tree or `None` if the item has no more sibling or if it's not part of the tree view
    pub fn previous_sibling(&self, item: &TreeItem) ->  Option<TreeItem> {
        use winapi::um::commctrl::TVGN_PREVIOUS;
        next_treeview_item(&self.handle, TVGN_PREVIOUS, item.handle)
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
    pub fn insert_item<'a>(&self, new: &'a str, parent: Option<&TreeItem>, position: TreeInsert) -> TreeItem {
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
    pub fn select_item(&self, item: &TreeItem) {
        use winapi::um::commctrl::{TVITEMW, TVM_SETITEMW, TVIF_STATE};

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

    /// Creates an iterator over the tree view items
    #[cfg(feature="tree-view-iterator")]
    pub fn iter<'a>(&'a self) -> crate::TreeViewIterator<'a> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        self.handle.hwnd().expect(BAD_HANDLE);

        crate::TreeViewIterator::new(self, ptr::null_mut())
    }

    /// Creates an iterator over the children of an item. This does not include the item itself.
    #[cfg(feature="tree-view-iterator")]
    pub fn iter_item<'a>(&'a self, item: &TreeItem) -> crate::TreeViewIterator<'a> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        self.handle.hwnd().expect(BAD_HANDLE);

        crate::TreeViewIterator::new(self, item.handle)
    }

    /// Returns the text of the selected item. Return None if the item is not in the tree view.
    /// The returned text value cannot be bigger than 260 characters
    pub fn item_text(&self, tree_item: &TreeItem) -> Option<String> {
        use winapi::um::commctrl::{TVM_GETITEMW, TVITEMW, TVIF_TEXT, TVIF_HANDLE};
        const BUFFER_MAX: usize = 260;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE); 

        let mut text_buffer = Vec::with_capacity(BUFFER_MAX);
        unsafe { text_buffer.set_len(BUFFER_MAX); }

        let mut item: TVITEMW = unsafe { mem::zeroed() };
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

    /// Returns `true` if the tree view item has children. Returns `None` if the item is not in the tree view.
    pub fn item_has_children(&self, tree_item: &TreeItem) -> Option<bool> {
        use winapi::um::commctrl::{TVM_GETITEMW, TVITEMW, TVIF_CHILDREN, TVIF_HANDLE};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE); 

        let mut item: TVITEMW = unsafe { mem::zeroed() };
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
        use winapi::um::commctrl::{TVM_GETITEMW, TVITEMW, TVIF_STATE, TVIF_HANDLE};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE); 

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
        use winapi::um::commctrl::TVM_GETCOUNT;

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
    pub fn class_name(&self) -> &'static str {
        winapi::um::commctrl::WC_TREEVIEW
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        use winapi::um::commctrl::{TVS_HASBUTTONS, TVS_LINESATROOT, TVS_HASLINES};

        WS_VISIBLE | TVS_HASBUTTONS | TVS_LINESATROOT | TVS_HASLINES
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD, WS_BORDER};
        use winapi::um::commctrl::TVS_NOTOOLTIPS;

        WS_CHILD | WS_BORDER | TVS_NOTOOLTIPS
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
    font: Option<&'a Font>,
    parent: Option<ControlHandle>,
    image_list: Option<&'a ImageList>,
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
        } else {
            out.set_font(Font::global_default().as_ref());
        }

        if self.image_list.is_some() {
            out.set_image_list(self.image_list);
        }

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

    if handle.blank() { panic!(NOT_BOUND); }
    let handle = handle.hwnd().expect(BAD_HANDLE);

    let handle = wh::send_message(handle, TVM_GETNEXTITEM, action as _, item as _) as HTREEITEM;
    if handle.is_null() {
        None
    } else {
        Some(TreeItem { handle })
    }
}
