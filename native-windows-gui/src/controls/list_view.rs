use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use winapi::um::commctrl::{
    LVS_ICON, LVS_SMALLICON, LVS_LIST, LVS_REPORT, LV_VIEW_TILE, LVS_NOCOLUMNHEADER, LVCOLUMNW, LVCFMT_LEFT, LVCFMT_RIGHT, LVCFMT_CENTER, LVCFMT_JUSTIFYMASK,
    LVCFMT_IMAGE, LVCFMT_BITMAP_ON_RIGHT, LVCFMT_COL_HAS_IMAGES, LVCF_FMT
};
use super::{ControlBase, ControlHandle};
use crate::win32::window_helper as wh;
use crate::NwgError;
use std::ptr;


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
    */
    pub struct ListViewFlags: u32 {
        const NONE = LVS_REPORT;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;

        const NO_HEADER = LVS_NOCOLUMNHEADER;

        const ICON_LIST = LVS_ICON;
        const SMALL_ICON_LIST = LVS_SMALLICON;
        const SIMPLE_LIST = LVS_LIST;
        const DETAILED_LIST = LVS_REPORT;
        const TILE_LIST = LV_VIEW_TILE;
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


/// Represents a column in a detailed list view
pub struct ListViewColumn {
    data: LVCOLUMNW
}

impl ListViewColumn {

    pub fn format(&mut self, fmt: Option<ListViewColumnFlags>) {
        let mut data = &mut self.data;
        match fmt {
            Some(fmt) => {
                data.mask |= LVCF_FMT;
                data.fmt = fmt.bits as i32;
            },
            None => {
                data.mask &= !LVCF_FMT;
                data.fmt = 0;
            }
        }
    }

}


/**
A list-view control is a window that displays a collection of items.
List-view controls provide several ways to arrange and display items and are much more flexible than simple ListBox.
*/
#[derive(Default, Eq, PartialEq)]
pub struct ListView {
    pub handle: ControlHandle
}

impl ListView {

    pub fn builder() -> ListViewBuilder {
        ListViewBuilder {
            size: (300, 300),
            position: (0, 0),
            flags: None,
            parent: None
        }
    }

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
        LVS_REPORT | WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD, WS_BORDER};

        WS_CHILD | WS_BORDER
    }

}


pub struct ListViewBuilder {
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<ListViewFlags>,
    parent: Option<ControlHandle>
}

impl ListViewBuilder {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ListViewBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut ListView) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

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

        Ok(())
    }

}

impl Default for ListViewColumn {

    fn default() -> ListViewColumn {
        let data = LVCOLUMNW {
            mask: 0,
            fmt: 0,
            cx: 0,
            pszText: ptr::null_mut(),
            cchTextMax: 0,
            iSubItem: 0,
            iImage: 0,
            iOrder: 0,
            cxMin: 0,
            cxDefault: 0,
            cxIdeal: 0,
        };

        ListViewColumn { data }
    }

}
