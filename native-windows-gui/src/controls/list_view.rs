use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_TABSTOP};
use winapi::um::commctrl::{
    LVS_ICON, LVS_SMALLICON, LVS_LIST, LVS_REPORT, LVS_NOCOLUMNHEADER, LVCOLUMNW, LVCFMT_LEFT, LVCFMT_RIGHT, LVCFMT_CENTER, LVCFMT_JUSTIFYMASK,
    LVCFMT_IMAGE, LVCFMT_BITMAP_ON_RIGHT, LVCFMT_COL_HAS_IMAGES, LVITEMW, LVIF_TEXT, LVCF_WIDTH, LVCF_TEXT
};
use super::{ControlBase, ControlHandle};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::to_utf16;
use crate::{NwgError, RawEventHandler, unbind_raw_event_handler};
use std::{mem, cell::RefCell};


const NOT_BOUND: &'static str = "ListView is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ListView handle is not HWND!";


bitflags! {
    /**
        The list view flags:

        * NONE:     Default list view. Equivalent to an invisible detailed list
        * VISIBLE:  The list view is immediatly visible after creation
        * DISABLED: The list view cannot be interacted with by the user. It also has a grayed out look. The user can drag the items to any location in the list-view window.
        * NO_HEADER: The list do not have a header.

        List view type (only one of those flags should be set):

        * ICON_LIST: A list where each item appears as a full-sized icon with a label below it. The user can drag the items to any location in the list-view window.
        * SMALL_ICON_LIST: A list where each item appears as a small icon with the label to the right of it
        * SIMPLE_LIST: Each item appears as a small icon with a label to the right of it. Items are arranged in columns and the user cannot drag them to an arbitrary location.
        * DETAILED_LIST: The leftmost column is always left justified and contains the small icon and label. Subsequent columns contain subitems as specified by the application. Each column has a header, unless you also specify the NO_HEADER flag.
        * TILE_LIST: Each item appears as a full-sized icon with a label of one or more lines beside it.
        * TAB_STOP: The control can be selected using tab navigation
    */
    pub struct ListViewFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;

        // Remove the headers in Detailed view (always ON, see "Windows is Shit" section in ListView docs as of why)
        const NO_HEADER = LVS_NOCOLUMNHEADER;
    }
}

bitflags! {
    /**
        The format flags for a list view column. Not all combination are valid.
        The alignment of the leftmost column is always LEFT.

        * LEFT: Text is left-aligned. 
        * RIGHT: Text is right-aligned
        * CENTER: Text is centered
        * JUSTIFY_MASK: A bitmask used to select those bits of fmt that control field justification. 
        * IMAGE: The items under to column displays an image from an image list
        * IMAGE_RIGHT: The bitmap appears to the right of text
        * IMAGE_COL: The header item contains an image in the image list.
    */
    pub struct ListViewColumnFlags: u32 {
        const LEFT = LVCFMT_LEFT as u32;
        const RIGHT = LVCFMT_RIGHT as u32;
        const CENTER = LVCFMT_CENTER as u32;
        const JUSTIFY_MASK = LVCFMT_JUSTIFYMASK as u32;
        const IMAGE = LVCFMT_IMAGE as u32;
        const IMAGE_RIGHT = LVCFMT_BITMAP_ON_RIGHT as u32;
        const IMAGE_COL = LVCFMT_COL_HAS_IMAGES as u32;
    }
}


#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ListViewStyle {
    Simple,
    Detailed,
    Icon,
    SmallIcon,
}

impl ListViewStyle {
    fn from_bits(bits: u32) -> ListViewStyle {
        let bits = bits & 0b11;
        match bits {
            LVS_ICON => ListViewStyle::Icon,
            LVS_REPORT => ListViewStyle::Detailed,
            LVS_SMALLICON => ListViewStyle::SmallIcon,
            LVS_LIST => ListViewStyle::Simple,
            _ => unreachable!()
        }
    }

    fn bits(&self) -> u32 {
        match self {
            ListViewStyle::Simple => LVS_LIST,
            ListViewStyle::Detailed => LVS_REPORT,
            ListViewStyle::Icon => LVS_ICON,
            ListViewStyle::SmallIcon => LVS_SMALLICON,
        }
    }
}

/// Represents a column in a detailed list view
pub struct InsertListViewColumn {
    /// Index of the column
    pub index: Option<i32>,

    /// Format of the column
    pub fmt: Option<i32>,

    /// Width of the column in pixels
    pub width: Option<i32>,

    /// Text of the column to insert
    pub text: String
}


/// Represents a list view item parameters
#[derive(Default, Clone)]
pub struct InsertListViewItem {
    /// Index of the item to be inserted
    /// If None and `insert_item` is used, the item is added at the end of the list
    pub index: Option<i32>,

    /// Text of the item to insert
    pub text: String
}


/**
A list-view control is a window that displays a collection of items.
List-view controls provide several ways to arrange and display items and are much more flexible than simple ListBox.

Requires the `list-view` feature. 

Builder parameters:
    * `parent`:     **Required.** The list view parent container.
    * `size`:       The list view size.
    * `position`:   The list view position.
    * `flags`:      A combination of the ListViewFlags values.
    * `style`:      One of the value of `ListViewStyle`
    * `item_count`: Number of item to preallocate
    * `focus`:      The control receive focus after being created


Windows is Shit:
- The win32 header controls leaks megabytes of memory per seconds because it is shit. As such, NO_HEADER is always ON.

*/
#[derive(Default)]
pub struct ListView {
    pub handle: ControlHandle,
    handler0: RefCell<Option<RawEventHandler>>,
}

impl ListView {

    pub fn builder() -> ListViewBuilder {
        ListViewBuilder {
            size: (300, 300),
            position: (0, 0),
            focus: false,
            flags: None,
            style: ListViewStyle::Simple,
            parent: None,
            item_count: 0
        }
    }

    /// Insert a column in the report. Column are only used with the Detailed list view style.
    pub fn insert_column<I: Into<InsertListViewColumn>>(&self, insert: I) {
        use winapi::um::commctrl::LVM_INSERTCOLUMNW;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        match self.list_style() {
            ListViewStyle::Detailed => {},
            _ => { return; }
        }

        let insert = insert.into();

        let mut mask = LVCF_TEXT;
        let mut text = to_utf16(&insert.text);

        if insert.width.is_some() { mask |= LVCF_WIDTH; }

        let mut item: LVCOLUMNW = unsafe { mem::zeroed() };
        item.mask = mask;
        item.cx = insert.width.unwrap_or(100);
        item.pszText = text.as_mut_ptr();
        item.cchTextMax = insert.text.len() as i32;

        let col_count = self.column_len() as i32;
    
        wh::send_message(
            handle, 
            LVM_INSERTCOLUMNW, 
            insert.index.unwrap_or(col_count) as usize, 
            (&item as *const LVCOLUMNW) as _
        );
    }

    /// Insert a new item into the list view
    pub fn insert_item<I: Into<InsertListViewItem>>(&self, insert: I) {
        use winapi::um::commctrl::LVM_INSERTITEMW;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let insert = insert.into();

        let mask = LVIF_TEXT;
        let mut text = to_utf16(&insert.text);

        let mut item: LVITEMW = unsafe { mem::zeroed() };
        item.mask = mask;
        item.iItem = insert.index.unwrap_or(i32::max_value());
        item.pszText = text.as_mut_ptr();
        item.cchTextMax = insert.text.len() as i32;

        wh::send_message(handle, LVM_INSERTITEMW , 0, &item as *const LVITEMW as _);
    }

    /// Insert multiple items into the control. Basically a loop over `insert_item`.
    pub fn insert_items<I: Copy+Into<InsertListViewItem>>(&self, insert: &[I]) {
        for &i in insert {
            self.insert_item(i);
        }
    }

    /// Return the current style of the list view
    pub fn list_style(&self) -> ListViewStyle {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let style = wh::get_style(handle);
        
        ListViewStyle::from_bits(style)
    }

    /// Sets the list view style of the control
    pub fn set_list_style(&self, style: ListViewStyle) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let mut old_style = wh::get_style(handle);
        old_style = old_style & !0b11;

        wh::set_style(handle, old_style | style.bits());
    }

    /// Returns the number of items in the list view
    pub fn len(&self) -> usize {
        use winapi::um::commctrl::LVM_GETITEMCOUNT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, LVM_GETITEMCOUNT , 0, 0) as usize
    }

    /// Returns the number of columns in the list view
    pub fn column_len(&self) -> usize {
        use winapi::um::commctrl::LVM_GETCOLUMNWIDTH ;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let mut count = 0;
        while wh::send_message(handle, LVM_GETCOLUMNWIDTH, count, 0) != 0 {
            count += 1;
        }

        count
    }

    /// Preallocate space for n number of item in the whole control.
    /// For example calling this method with n=1000 while the list has 500 items will add space for 500 new items.
    pub fn set_item_count(&self, n: u32) {
        use winapi::um::commctrl::LVM_SETITEMCOUNT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, LVM_SETITEMCOUNT, n as _, 0);
    }

    /// Enable or disable the redrawing of the control when a new item is added.
    /// When inserting a large number of items, it's better to disable redraw and reenable it after the items are inserted.
    pub fn set_redraw(&self, enabled: bool) {
        use winapi::um::winuser::WM_SETREDRAW;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, WM_SETREDRAW, enabled as _, 0);
    }

    /// Removes all item from the listview
    pub fn clear(&self) {
        use winapi::um::commctrl::LVM_DELETEALLITEMS;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, LVM_DELETEALLITEMS, 0, 0);
    }

    /// Returns true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Sets the keyboard focus on the button
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Returns true if the control user can interact with the control, return false otherwise
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

    /// Returns true if the control is visible to the user. Will return true even if the 
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

    /// Returns the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Sets the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, true) }
    }

    /// Returns the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Sets the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        ::winapi::um::commctrl::WC_LISTVIEW
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | WS_TABSTOP
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD, WS_BORDER};

        WS_CHILD | WS_BORDER | LVS_NOCOLUMNHEADER
    }

}

impl Drop for ListView {
    fn drop(&mut self) {
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            unbind_raw_event_handler(h);
        }

        self.handle.destroy();
    }
}

pub struct ListViewBuilder {
    size: (i32, i32),
    position: (i32, i32),
    focus: bool,
    flags: Option<ListViewFlags>,
    style: ListViewStyle,
    item_count: u32,
    parent: Option<ControlHandle>
}

impl ListViewBuilder {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ListViewBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn flags(mut self, flags: ListViewFlags) -> ListViewBuilder {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ListViewBuilder {
        self.size = size;
        self
    }

    pub fn position(mut self, position: (i32, i32)) -> ListViewBuilder {
        self.position = position;
        self
    }

    pub fn item_count(mut self, count: u32) -> ListViewBuilder {
        self.item_count = count;
        self
    }

    pub fn list_style(mut self, style: ListViewStyle) -> ListViewBuilder {
        self.style = style;
        self
    }

    pub fn focus(mut self, focus: bool) -> ListViewBuilder {
        self.focus = focus;
        self
    }

    pub fn build(self, out: &mut ListView) -> Result<(), NwgError> {
        let mut flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());
        flags |= self.style.bits();

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("ListView"))
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .text("")
            .parent(Some(parent))
            .build()?;

        if self.item_count > 0 {
            out.set_item_count(self.item_count);
        }

        if self.focus {
            out.set_focus();
        }

        Ok(())
    }

}

impl<'a> From<&'a str> for InsertListViewItem {
    fn from(i: &'a str) -> Self {
        InsertListViewItem {
            index: None,
            text: i.to_string()
        }
    }
}

impl From<String> for InsertListViewItem {
    fn from(i: String) -> Self {
        InsertListViewItem {
            index: None,
            text: i
        }
    }
}

impl<'a> From<&'a str> for InsertListViewColumn {
    fn from(i: &'a str) -> Self {
        InsertListViewColumn {
            index: None,
            fmt: None,
            width: Some(100),
            text: i.to_string()
        }
    }
}

impl From<String> for InsertListViewColumn {
    fn from(i: String) -> Self {
        InsertListViewColumn {
            index: None,
            fmt: None,
            width: Some(100),
            text: i
        }
    }
}
