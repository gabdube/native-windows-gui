use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, WS_TABSTOP};
use winapi::um::commctrl::{
    LVS_ICON, LVS_SMALLICON, LVS_LIST, LVS_REPORT, LVS_NOCOLUMNHEADER, LVCOLUMNW, LVCFMT_LEFT, LVCFMT_RIGHT, LVCFMT_CENTER, LVCFMT_JUSTIFYMASK,
    LVCFMT_IMAGE, LVCFMT_BITMAP_ON_RIGHT, LVCFMT_COL_HAS_IMAGES, LVITEMW, LVIF_TEXT, LVCF_WIDTH, LVCF_TEXT, LVS_EX_GRIDLINES, LVS_EX_BORDERSELECT,
    LVS_EX_AUTOSIZECOLUMNS, LVM_SETEXTENDEDLISTVIEWSTYLE, LVS_EX_FULLROWSELECT, LVS_SINGLESEL, LVCF_FMT, LVIF_IMAGE, LVS_SHOWSELALWAYS,
    LVS_EX_HEADERDRAGDROP, LVS_EX_HEADERINALLVIEWS
};
use super::{ControlBase, ControlHandle};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16, from_utf16, check_hwnd};
use crate::{NwgError, RawEventHandler, unbind_raw_event_handler};
use std::{mem, cell::RefCell};

#[cfg(feature="image-list")]
use crate::ImageList;


const NOT_BOUND: &'static str = "ListView is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ListView handle is not HWND!";


bitflags! {
    /**
        The list view flags:

        * VISIBLE:  The list view is immediatly visible after creation
        * DISABLED: The list view cannot be interacted with by the user. It also has a grayed out look. The user can drag the items to any location in the list-view window.
        * TAB_STOP: The control can be selected using tab navigation
        * NO_HEADER: Remove the headers in Detailed view (always ON, see "Listview header problem" section in ListView docs as of why)
        * SINGLE_SELECTION: Only one item can be selected
        * ALWAYS_SHOW_SELECTION: Shows the selected list view item when the control is not in focus
    */
    pub struct ListViewFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;

        const SINGLE_SELECTION = LVS_SINGLESEL;

        const ALWAYS_SHOW_SELECTION = LVS_SHOWSELALWAYS;

        // Remove the headers in Detailed view (always ON, see "Listview header problem" section in ListView docs as of why)
        const NO_HEADER = LVS_NOCOLUMNHEADER;
    }
}

bitflags! {
    /**
        The list view extended flags (to use with ListViewBuilder::ex_flags):

        * NONE:  Do not use any extended styles
        * GRID:  The list view has a grid. Only if the list view is in report mode.
        * BORDER_SELECT: Only highlight the border instead of the full item. COMMCTRL version 4.71 or later
        * AUTO_COLUMN_SIZE: Automatically resize to column
        * FULL_ROW_SELECT: When an item is selected, the item and all its subitems are highlighted. Only in detailed view 
        * HEADER_DRAG_DROP: The user can drag and drop the headers to rearrage them 
        * HEADER_IN_ALL_VIEW: Show the header in all view (not just report)
    */
    pub struct ListViewExFlags: u32 {
        const NONE = 0;
        const GRID = LVS_EX_GRIDLINES;
        const BORDER_SELECT = LVS_EX_BORDERSELECT;
        const AUTO_COLUMN_SIZE = LVS_EX_AUTOSIZECOLUMNS;
        const FULL_ROW_SELECT = LVS_EX_FULLROWSELECT;
        const HEADER_DRAG_DROP = LVS_EX_HEADERDRAGDROP;
        const HEADER_IN_ALL_VIEW = LVS_EX_HEADERINALLVIEWS;

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


#[cfg(feature="image-list")]
#[derive(Copy, Clone, Debug)]
pub enum ListViewImageListType {
    /// Normal sized icons
    Normal,

    /// Small icons
    Small,

    /// State icons
    State,

    /// Group header list (not yet implemented)
    GroupHeader
}

#[cfg(feature="image-list")]
impl ListViewImageListType {

    fn to_raw(&self) -> i32 {
        use winapi::um::commctrl::{LVSIL_NORMAL, LVSIL_SMALL, LVSIL_STATE, LVSIL_GROUPHEADER};

        match self {
            Self::Normal => LVSIL_NORMAL,
            Self::Small => LVSIL_SMALL,
            Self::State => LVSIL_STATE,
            Self::GroupHeader => LVSIL_GROUPHEADER,
        }
    }

}

#[derive(Default, Clone, Debug)]
/// Represents a column in a detailed list view
pub struct InsertListViewColumn {
    /// Index of the column
    pub index: Option<i32>,

    /// Format of the column
    pub fmt: Option<i32>,

    /// Width of the column in pixels
    pub width: Option<i32>,

    /// Text of the column to insert
    pub text: Option<String>
}

/// The data of a list view column
#[derive(Default, Clone, Debug)]
pub struct ListViewColumn {
    pub fmt: i32,
    pub width: i32,
    pub text: String,
}


/// Represents a list view item parameters
#[derive(Default, Clone, Debug)]
pub struct InsertListViewItem {
    /// Index of the item to be inserted
    /// If None and `insert_item` is used, the item is added at the end of the list
    pub index: Option<i32>,

    /// Index of the column
    pub column_index: i32,

    /// Text of the item to insert
    pub text: Option<String>,

    /// Index of the image in the image list
    /// Icons are only supported at column 0
    #[cfg(feature="image-list")]
    pub image: Option<i32>
}

/// The data of a list view item
#[derive(Default, Clone, Debug)]
pub struct ListViewItem {
    pub row_index: i32,
    pub column_index: i32,
    pub text: String,

    /// If the item is currently selected
    pub selected: bool,

    #[cfg(feature="image-list")]
    pub image: i32,
}

/**
A list-view control is a window that displays a collection of items.
List-view controls provide several ways to arrange and display items and are much more flexible than simple ListBox.

Requires the `list-view` feature. 

Builder parameters:
  * `parent`:           **Required.** The list view parent container.
  * `size`:             The list view size.
  * `position`:         The list view position.
  * `background_color`: The list view background color in RGB format
  * `text_color`:       The list view text color in RGB format
  * `flags`:            A combination of the ListViewFlags values.
  * `ex_flags`:         A combination of the ListViewExFlags values.
  * `style`:            One of the value of `ListViewStyle`
  * `item_count`:       Number of item to preallocate
  * `list_style`:       The default style of the listview
  * `focus`:            The control receive focus after being created

**Control events:**
  * `MousePress(_)`:   Generic mouse press events on the tree view
  * `OnMouseMove`:     Generic mouse mouse event
  * `OnMouseWheel`:    Generic mouse wheel event
  * `OnKeyPress`:      Generic key press event
  * `OnKeyRelease`:    Generic key release event
  * `OnListViewClear`: When all the items in a list view are destroyed
  * `OnListViewItemRemoved`: When an item is about to be removed from the list view
  * `OnListViewItemInsert`: When a new item is inserted in the list view
  * `OnListViewItemActivated`: When an item in the list view is activated by the user
  * `OnListViewItemChanged`: When an item is selected/unselected in the listview
  * `OnListViewFocus`: When the list view has received focus
  * `OnListViewFocusLost`: When the list view has lost focus

Listview header problem:
- The win32 header controls leaks megabytes of memory per seconds because of some issues with the rendering. 
As such, NO_HEADER is always ON.  

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
            background_color: None,
            text_color: None,
            focus: false,
            flags: None,
            ex_flags: None,
            style: ListViewStyle::Simple,
            parent: None,
            item_count: 0
        }
    }

    /// Sets the image list of the listview
    /// A listview can accept different kinds of image list. See `ListViewImageListType`
    #[cfg(feature="image-list")]
    pub fn set_image_list(&self, list: Option<&ImageList>, list_type: ListViewImageListType) {
        use winapi::um::commctrl::LVM_SETIMAGELIST;
        use std::ptr;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let list_handle = list.map(|l| l.handle).unwrap_or(ptr::null_mut());
        wh::send_message(handle, LVM_SETIMAGELIST, list_type.to_raw() as _, list_handle as _);
        
        self.invalidate();
    }

    /// Returns the current image list for the selected type. The returned image list will not be owned.
    /// Can return `None` if there is no assocaited image list
    #[cfg(feature="image-list")]
    pub fn image_list(&self, list_type: ListViewImageListType) -> Option<ImageList> {
        use winapi::um::commctrl::LVM_GETIMAGELIST;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        match wh::send_message(handle, LVM_GETIMAGELIST, list_type.to_raw() as _, 0) {
            0 => None,
            handle => Some( ImageList {
                handle: handle as _,
                owned: false
            })
        }
    }

    /// Sets the text color of the list view
    pub fn set_text_color(&self, r: u8, g: u8, b: u8) {
        use winapi::um::commctrl::LVM_SETTEXTCOLOR;
        use winapi::um::wingdi::RGB;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let color = RGB(r, g, b);

        wh::send_message(handle, LVM_SETTEXTCOLOR, 0, color as _);

        self.invalidate();
    }

    /// Returns the current text color
    pub fn text_color(&self) -> [u8; 3] {
        use winapi::um::commctrl::LVM_GETTEXTCOLOR;
        use winapi::um::wingdi::{GetRValue, GetGValue, GetBValue};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let col = wh::send_message(handle, LVM_GETTEXTCOLOR, 0, 0) as u32;

        [
            GetRValue(col),
            GetGValue(col),
            GetBValue(col),
        ]
    }

    /// Sets the background color of the list view
    pub fn set_background_color(&self, r: u8, g: u8, b: u8) {
        use winapi::um::commctrl::LVM_SETBKCOLOR;
        use winapi::um::wingdi::RGB;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let color = RGB(r, g, b);

        wh::send_message(handle, LVM_SETBKCOLOR, 0, color as _);

        self.invalidate();
    }

    /// Returns the background color of the list view
    pub fn background_color(&self) -> [u8; 3] {
        use winapi::um::commctrl::LVM_GETBKCOLOR;
        use winapi::um::wingdi::{GetRValue, GetGValue, GetBValue};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let col = wh::send_message(handle, LVM_GETBKCOLOR, 0, 0) as u32;

        [
            GetRValue(col),
            GetGValue(col),
            GetBValue(col),
        ]
    }

    /// Returns the index of the selected column. Only available if Comclt32.dll version is >= 6.0.
    pub fn selected_column(&self) -> usize {
        use winapi::um::commctrl::LVM_GETSELECTEDCOLUMN;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_GETSELECTEDCOLUMN, 0, 0) as usize
    }

    /// Sets the selected column. Only available if Comclt32.dll version is >= 6.0.
    pub fn set_selected_column(&self, index: usize) {
        use winapi::um::commctrl::LVM_SETSELECTEDCOLUMN;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_SETSELECTEDCOLUMN, index as _, 0);
    }

    /// Returns the number of selected items
    pub fn selected_count(&self) -> usize {
        use winapi::um::commctrl::LVM_GETSELECTEDCOUNT;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_GETSELECTEDCOUNT, 0, 0) as usize
    }

    /// Inserts a column in the report. Column are only used with the Detailed list view style.
    pub fn insert_column<I: Into<InsertListViewColumn>>(&self, insert: I) {
        use winapi::um::commctrl::LVM_INSERTCOLUMNW;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        match self.list_style() {
            ListViewStyle::Detailed => {},
            _ => { return; }
        }

        let insert = insert.into();

        let mut mask = LVCF_TEXT | LVCF_WIDTH;

        let text = insert.text.unwrap_or("".to_string());
        let mut text = to_utf16(&text);

        if insert.fmt.is_some() { mask |= LVCF_FMT; }

        let mut item: LVCOLUMNW = unsafe { mem::zeroed() };
        item.mask = mask;
        item.fmt = insert.fmt.unwrap_or(0);
        item.cx = insert.width.unwrap_or(100);
        item.pszText = text.as_mut_ptr();
        item.cchTextMax = text.len() as i32;

        let col_count = self.column_len() as i32;
    
        wh::send_message(
            handle, 
            LVM_INSERTCOLUMNW, 
            insert.index.unwrap_or(col_count) as usize, 
            (&item as *const LVCOLUMNW) as _
        );
    }

    /// Checks if there is a column at the selected index
    pub fn has_column(&self, index: usize) -> bool {
        use winapi::um::commctrl::LVM_GETCOLUMNW;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut col: LVCOLUMNW = unsafe { mem::zeroed() };

        wh::send_message(handle, LVM_GETCOLUMNW, index as _, &mut col as *mut LVCOLUMNW as _) != 0
    }

    /// Returns the information of a column.
    /// Because there's no way to fetch the actual text length, it's up to you to set the maximum buffer size
    pub fn column(&self, index: usize, text_buffer_size: i32) -> Option<ListViewColumn> {
        use winapi::um::commctrl::LVM_GETCOLUMNW;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut text_buffer: Vec<u16> = Vec::with_capacity(text_buffer_size as _);
        unsafe { text_buffer.set_len(text_buffer_size as _); }

        let mut col: LVCOLUMNW = unsafe { mem::zeroed() };
        col.mask = LVCF_TEXT | LVCF_WIDTH | LVCF_FMT;
        col.pszText = text_buffer.as_mut_ptr();
        col.cchTextMax = text_buffer_size;

        match wh::send_message(handle, LVM_GETCOLUMNW, index as _, &mut col as *mut LVCOLUMNW as _) == 0 {
            true => None,
            false => Some(ListViewColumn {
                fmt: col.fmt,
                width: col.cx,
                text: from_utf16(&text_buffer),
            })
        }
    }

    /// Sets the information of a column. Does nothing if there is no column at the selected index
    pub fn update_column<I: Into<InsertListViewColumn>>(&self, index: usize, column: I) {
        use winapi::um::commctrl::LVM_SETCOLUMNW;

        if !self.has_column(index) {
            return;
        }

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let insert = column.into();

        let use_text = insert.text.is_some();
        let use_width = insert.width.is_some();
        let use_fmt = insert.fmt.is_some();

        let mut mask = 0;
        if use_text { mask |= LVCF_TEXT; }
        if use_width { mask |= LVCF_WIDTH; }
        if use_fmt { mask |= LVCF_FMT; }

        let text = insert.text.unwrap_or("".to_string());
        let mut text = to_utf16(&text);

        let mut item: LVCOLUMNW = unsafe { mem::zeroed() };
        item.mask = mask;
        item.fmt = insert.fmt.unwrap_or(0);
        item.cx = insert.width.unwrap_or(0);

        if use_text {
            item.pszText = text.as_mut_ptr();
            item.cchTextMax = text.len() as i32;
        }

        wh::send_message(handle, LVM_SETCOLUMNW, index as _, &mut item as *mut LVCOLUMNW as _);
    }

    /// Deletes a column in a list view. Removing the column at index 0 is only available if ComCtl32.dll is version 6 or later.
    pub fn remove_column(&self, column_index: usize) {
        use winapi::um::commctrl::LVM_DELETECOLUMN;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_DELETECOLUMN , column_index as _, 0);
    }

    /// Select or unselect an item at `row_index`. Does nothing if the index is out of bounds.
    pub fn select_item(&self, row_index: usize, selected: bool) {
        use winapi::um::commctrl::{LVM_SETITEMW, LVIF_STATE, LVIS_SELECTED};

        if !self.has_item(row_index, 0) {
            return;
        }

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut item: LVITEMW = unsafe { mem::zeroed() };
        item.iItem = row_index as _;
        item.mask = LVIF_STATE;
        item.state = match selected { true => LVIS_SELECTED, false => 0 };
        item.stateMask = LVIS_SELECTED;

        wh::send_message(handle, LVM_SETITEMW , 0, &mut item as *mut LVITEMW as _);
    }

    /// Returns the index of the first selected item.
    /// If there's more than one item selected, use `selected_items`
    pub fn selected_item(&self) -> Option<usize> {
        use winapi::um::commctrl::{LVM_GETNEXTITEMINDEX, LVNI_SELECTED, LVITEMINDEX};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let mut index = None;

        let mut i_data = LVITEMINDEX { iItem: -1, iGroup: -1 };

        if wh::send_message(handle, LVM_GETNEXTITEMINDEX, &mut i_data as *mut LVITEMINDEX as _, LVNI_SELECTED) != 0 {
            index = Some(i_data.iItem as usize);
        }

        index
    }

    /// Returns the indices of every selected items.
    pub fn selected_items(&self) -> Vec<usize> {
        use winapi::um::commctrl::{LVM_GETNEXTITEMINDEX, LVNI_SELECTED, LVITEMINDEX};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let mut indices = Vec::with_capacity(self.len());

        let mut i_data = LVITEMINDEX { iItem: -1, iGroup: -1 };
        
        while wh::send_message(handle, LVM_GETNEXTITEMINDEX, &mut i_data as *mut LVITEMINDEX as _, LVNI_SELECTED) != 0 {
            indices.push(i_data.iItem as usize);
        }

        indices
    }

    /// Inserts a new item into the list view
    pub fn insert_item<I: Into<InsertListViewItem>>(&self, insert: I) {
        use winapi::um::commctrl::{LVM_INSERTITEMW, LVM_SETITEMW};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let insert = insert.into();

        let row_insert = insert.index.unwrap_or(i32::max_value());
        let column_insert = insert.column_index;
        if column_insert > 0 && !self.has_item(row_insert as _, 0) {
            self.insert_item(InsertListViewItem { 
                index: Some(row_insert),
                column_index: 0,
                text: None,

                #[cfg(feature="image-list")]
                image: None,
            });
        }

        let mask = LVIF_TEXT | check_image_mask(&insert);
        let image = check_image(&insert);
        let text = insert.text.unwrap_or("".to_string());
        let mut text = to_utf16(&text);

        let mut item: LVITEMW = unsafe { mem::zeroed() };
        item.mask = mask;
        item.iItem = row_insert;
        item.iImage = image;
        item.iSubItem = column_insert;
        item.pszText = text.as_mut_ptr();
        item.cchTextMax = text.len() as i32;

        if column_insert == 0 {
            wh::send_message(handle, LVM_INSERTITEMW , 0, &mut item as *mut LVITEMW as _);
        } else {
            wh::send_message(handle, LVM_SETITEMW , 0, &mut item as *mut LVITEMW as _);
        }
    }

    /// Checks if the item at the selected row is visible
    pub fn item_is_visible(&self, index: usize) -> bool {
        use winapi::um::commctrl::LVM_ISITEMVISIBLE;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_ISITEMVISIBLE , index as _, 0) == 1
    }

    /// Returns `true` if an item exists at the selected index or `false` otherwise.
    pub fn has_item(&self, row_index: usize, column_index: usize) -> bool {
        use winapi::um::commctrl::LVM_GETITEMW;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut item: LVITEMW = unsafe { mem::zeroed() };
        item.iItem = row_index as _;
        item.iSubItem = column_index as _;

        wh::send_message(handle, LVM_GETITEMW , 0, &mut item as *mut LVITEMW as _) == 1
    }

    /// Returns data of an item in the list view. Returns `None` if there is no data at the selected index
    /// Because there is no way to fetch the actual text size, `text_buffer_size` must be set manually
    pub fn item(&self, row_index: usize, column_index: usize, text_buffer_size: usize) -> Option<ListViewItem> {
        use winapi::um::commctrl::{LVM_GETITEMW, LVIF_STATE, LVIS_SELECTED};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut item: LVITEMW = unsafe { mem::zeroed() };
        item.iItem = row_index as _;
        item.iSubItem = column_index as _;
        item.mask = LVIF_IMAGE | LVIF_TEXT | LVIF_STATE;
        item.stateMask = LVIS_SELECTED;

        let mut text_buffer: Vec<u16> = Vec::with_capacity(text_buffer_size);
        unsafe { text_buffer.set_len(text_buffer_size); }
        item.pszText = text_buffer.as_mut_ptr();
        item.cchTextMax = text_buffer_size as _;


        let found = wh::send_message(handle, LVM_GETITEMW , 0, &mut item as *mut LVITEMW as _) == 1;
        if !found {
            return None;
        }

        Some ( build_list_view_image(row_index, column_index, item.state, &text_buffer, item.iImage)  )
    }

    /// Updates the item at the selected position
    /// Does nothing if there is no item at the selected position
    pub fn update_item<I: Into<InsertListViewItem>>(&self, row_index: usize, data: I) {
        use winapi::um::commctrl::LVM_SETITEMW;

        if !self.has_item(row_index, 0) {
            return;
        }

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let insert = data.into();

        let mut mask = check_image_mask(&insert);
        if insert.text.is_some() {
            mask |= LVIF_TEXT;
        }

        let image = check_image(&insert);

        let use_text = insert.text.is_some();
        let text = insert.text.unwrap_or("".to_string());
        let mut text = to_utf16(&text);

        let mut item: LVITEMW = unsafe { mem::zeroed() };
        item.mask = mask;
        item.iItem = row_index as _;
        item.iImage = image;
        item.iSubItem = insert.column_index as _;

        if use_text {
            item.pszText = text.as_mut_ptr();
            item.cchTextMax = text.len() as i32;
        }
        
        wh::send_message(handle, LVM_SETITEMW , 0, &mut item as *mut LVITEMW as _);
    }

    /// Remove all items on the seleted row. Returns `true` if an item was removed or false otherwise.
    /// To "remove" an item without deleting the row, use `update_item` and set the text to "".
    pub fn remove_item(&self, row_index: usize) -> bool {
        use winapi::um::commctrl::LVM_DELETEITEM;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_DELETEITEM , row_index as _, 0) == 1
    }

    /// Inserts multiple items into the control. Basically a loop over `insert_item`.
    pub fn insert_items<I: Clone+Into<InsertListViewItem>>(&self, insert: &[I]) {
        for i in insert.iter() {
            self.insert_item(i.clone());
        }
    }

    /// Insert multiple item at the selected row or at the end of the list if `None` was used.
    /// This method overrides the `index` and the `column_index` of the items.
    /// Useful when inserting strings into a single row. Ex: `list.insert_items_row(None, &["Hello", "World"]);`
    pub fn insert_items_row<I: Clone+Into<InsertListViewItem>>(&self, row_index: Option<i32>, insert: &[I]) {
        let mut column_index = 0;
        let row_index = row_index.or(Some(self.len() as _));
        
        for i in insert.iter() {
            let mut item: InsertListViewItem = i.clone().into();
            item.index = row_index;
            item.column_index = column_index;

            self.insert_item(item);

            column_index += 1;
        }
    }

    /// Returns the current style of the list view
    pub fn list_style(&self) -> ListViewStyle {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        ListViewStyle::from_bits(wh::get_style(handle))
    }

    /// Sets the list view style of the control
    pub fn set_list_style(&self, style: ListViewStyle) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut old_style = wh::get_style(handle);
        old_style = old_style & !0b11;

        wh::set_style(handle, old_style | style.bits());
    }

    /// Returns the number of items in the list view
    pub fn len(&self) -> usize {
        use winapi::um::commctrl::LVM_GETITEMCOUNT;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_GETITEMCOUNT , 0, 0) as usize
    }

    /// Returns the number of columns in the list view
    pub fn column_len(&self) -> usize {
        use winapi::um::commctrl::LVM_GETCOLUMNWIDTH;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

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

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_SETITEMCOUNT, n as _, 0);
    }

    /// Enable or disable the redrawing of the control when a new item is added.
    /// When inserting a large number of items, it's better to disable redraw and reenable it after the items are inserted.
    pub fn set_redraw(&self, enabled: bool) {
        use winapi::um::winuser::WM_SETREDRAW;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, WM_SETREDRAW, enabled as _, 0);
    }

    /// Sets the spacing between icons in list-view controls that have the ICON style.
    /// `dx` specifies the distance, in pixels, to set between icons on the x-axis
    /// `dy` specifies the distance, in pixels, to set between icons on the y-axis
    pub fn set_icon_spacing(&self, dx: u16, dy: u16) {
        use winapi::um::commctrl::LVM_SETICONSPACING;
        use winapi::shared::minwindef::MAKELONG;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let spacing = MAKELONG(dx, dy);
        wh::send_message(handle, LVM_SETICONSPACING, 0 as _, spacing as _);

        self.invalidate();
    }

    // Common methods

    /// Invalidate the whole drawing region.
    pub fn invalidate(&self) {
        use winapi::um::winuser::InvalidateRect;
        use std::ptr;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { InvalidateRect(handle, ptr::null(), 1); }
    }

    /// Removes all item from the listview
    pub fn clear(&self) {
        use winapi::um::commctrl::LVM_DELETEALLITEMS;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LVM_DELETEALLITEMS, 0, 0);
    }

    /// Returns true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Sets the keyboard focus on the button
    pub fn set_focus(&self) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Returns true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Returns true if the control is visible to the user. Will return true even if the 
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

    /// Returns the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Sets the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, true) }
    }

    /// Returns the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Sets the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        ::winapi::um::commctrl::WC_LISTVIEW
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | WS_TABSTOP | LVS_SHOWSELALWAYS
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
            drop(unbind_raw_event_handler(h));
        }

        self.handle.destroy();
    }
}

pub struct ListViewBuilder {
    size: (i32, i32),
    position: (i32, i32),
    background_color: Option<[u8; 3]>,
    text_color: Option<[u8; 3]>,
    focus: bool,
    flags: Option<ListViewFlags>,
    ex_flags: Option<ListViewExFlags>,
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

    pub fn ex_flags(mut self, flags: ListViewExFlags) -> ListViewBuilder {
        self.ex_flags = Some(flags);
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

    pub fn background_color(mut self, color: [u8; 3]) -> ListViewBuilder {
        self.background_color = Some(color);
        self
    }

    pub fn text_color(mut self, color: [u8; 3]) -> ListViewBuilder {
        self.text_color = Some(color);
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

        if let Some(flags) = self.ex_flags {
            let flags = flags.bits();
            wh::send_message(out.handle.hwnd().unwrap(), LVM_SETEXTENDEDLISTVIEWSTYLE, flags as _, flags as _);
        }

        if let Some([r, g, b]) = self.background_color {
            out.set_background_color(r, g, b);
        }

        if let Some([r, g, b]) = self.text_color {
            out.set_text_color(r, g, b);
        }

        Ok(())
    }

}

impl<'a> From<&'a str> for InsertListViewItem {
    fn from(i: &'a str) -> Self {
        InsertListViewItem {
            index: None,
            column_index: 0,
            text: Some(i.to_string()),

            #[cfg(feature="image-list")]
            image: None
        }
    }
}

impl From<String> for InsertListViewItem {
    fn from(i: String) -> Self {
        InsertListViewItem {
            index: None,
            column_index: 0,
            text: Some(i),

            #[cfg(feature="image-list")]
            image: None
        }
    }
}

impl<'a> From<&'a str> for InsertListViewColumn {
    fn from(i: &'a str) -> Self {
        InsertListViewColumn {
            index: None,
            fmt: None,
            width: Some(100),
            text: Some(i.to_string())
        }
    }
}

impl From<String> for InsertListViewColumn {
    fn from(i: String) -> Self {
        InsertListViewColumn {
            index: None,
            fmt: None,
            width: Some(100),
            text: Some(i)
        }
    }
}

 // Feature check

#[cfg(feature="image-list")]
fn check_image_mask(i: &InsertListViewItem) -> u32 {
    if i.image.is_some() { 
        LVIF_IMAGE
    } else {
        0
    }
}

#[cfg(feature="image-list")]
fn check_image(i: &InsertListViewItem) -> i32 { i.image.unwrap_or(0) }

#[cfg(not(feature="image-list"))]
fn check_image_mask(_i: &InsertListViewItem) -> u32 { 0 }

#[cfg(not(feature="image-list"))]
fn check_image(_i: &InsertListViewItem) -> i32 { 0 }

#[cfg(feature="image-list")]
fn build_list_view_image(row_index: usize, column_index: usize, state: u32, text_buffer: &[u16], image: i32) -> ListViewItem {
    use winapi::um::commctrl::LVIS_SELECTED;
    
    ListViewItem {
        row_index: row_index as _,
        column_index: column_index as _,
        text: from_utf16(&text_buffer),
        selected: state & LVIS_SELECTED == LVIS_SELECTED,
        image
    }
}

#[cfg(not(feature="image-list"))]
fn build_list_view_image(row_index: usize, column_index: usize, state: u32, text_buffer: &[u16], _image: i32) -> ListViewItem {
    use winapi::um::commctrl::LVIS_SELECTED;

    ListViewItem {
        row_index: row_index as _,
        column_index: column_index as _,
        text: from_utf16(&text_buffer),
        selected: state & LVIS_SELECTED == LVIS_SELECTED,
    }
}
