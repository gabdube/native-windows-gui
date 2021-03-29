use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{WPARAM, LPARAM};
use winapi::um::winuser::{LBS_MULTIPLESEL, LBS_NOSEL, WS_VISIBLE, WS_DISABLED, WS_TABSTOP};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16, from_utf16, check_hwnd};
use crate::{Font, NwgError};
use super::{ControlBase, ControlHandle};
use std::cell::{Ref, RefMut, RefCell};
use std::fmt::Display;
use std::ops::Range;
use std::mem;

const NOT_BOUND: &'static str = "ListBox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: ListBox handle is not HWND!";


bitflags! {
    /**
        The listbox flags

        * NONE:     No flags. Equivalent to a invisible listbox.
        * VISIBLE:  The listbox is immediatly visible after creation
        * DISABLED: The listbox cannot be interacted with by the user. It also has a grayed out look.
        * MULTI_SELECT: It is possible for the user to select more than 1 item at a time
        * NO_SELECT: It is impossible for the user to select the listbox items
        * TAB_STOP: The control can be selected using tab navigation
    */
    pub struct ListBoxFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const MULTI_SELECT = LBS_MULTIPLESEL;
        const NO_SELECT = LBS_NOSEL;
        const TAB_STOP = WS_TABSTOP;
    }
}

/**
A list box is a control window that contains a simple list of items from which the user can choose.

Requires the `list-box` feature. 

**Builder parameters:**
  * `parent`:          **Required.** The listbox parent container.
  * `size`:            The listbox size.
  * `position`:        The listbox position.
  * `enabled`:         If the listbox can be used by the user. It also has a grayed out look if disabled.
  * `focus`:           The control receive focus after being created
  * `flags`:           A combination of the ListBoxFlags values.
  * `ex_flags`:        A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `font`:            The font used for the listbox text
  * `collection`:      The default collections of the listbox
  * `selected_index`:  The default selected index in the listbox collection
  * `multi_selection`: The collections of indices to set as selected in a multi selection listbox 

**Control events:**
  * `OnListBoxSelect`: When the current listbox selection is changed
  * `OnListBoxDoubleClick`: When a listbox item is clicked twice rapidly
  * `MousePress(_)`: Generic mouse press events on the listbox
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event

```rust
use native_windows_gui as nwg;
fn build_listbox(listb: &mut nwg::ListBox<&'static str>, window: &nwg::Window, font: &nwg::Font) {
    nwg::ListBox::builder()
        .flags(nwg::ListBoxFlags::VISIBLE | nwg::ListBoxFlags::MULTI_SELECT)
        .collection(vec!["Hello", "World", "!!!!"])
        .multi_selection(vec![0, 1, 2])
        .font(Some(font))
        .parent(window)
        .build(listb);
}
```

*/
#[derive(Default)]
pub struct ListBox<D: Display+Default> {
    pub handle: ControlHandle,
    collection: RefCell<Vec<D>>
}

impl<D: Display+Default> ListBox<D> {

    pub fn builder<'a>() -> ListBoxBuilder<'a, D> {
        ListBoxBuilder {
            size: (100, 300),
            position: (0, 0),
            enabled: true,
            focus: false,
            flags: None,
            ex_flags: 0,
            font: None,
            collection: None,
            selected_index: None,
            multi_selection: Vec::new(),
            parent: None
        }
    }

    /// Add a new item to the listbox. Sort the collection if the listbox is sorted.
    pub fn push(&self, item: D) {
        use winapi::um::winuser::LB_ADDSTRING;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let display = format!("{}", item);
        let display_os = to_utf16(&display);

        unsafe {
            wh::send_message(handle, LB_ADDSTRING, 0, mem::transmute(display_os.as_ptr()));
        }

        self.collection.borrow_mut().push(item);
    }

    /// Insert an item in the collection and the control. 
    ///
    /// SPECIAL behaviour! If index is `std::usize::MAX`, the item is added at the end of the collection.
    /// The method will still panic if `index > len` with every other values.
    pub fn insert(&self, index: usize, item: D) {
        use winapi::um::winuser::LB_INSERTSTRING;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let display = format!("{}", item);
        let display_os = to_utf16(&display);

        let mut col = self.collection.borrow_mut();
        if index == std::usize::MAX {
            col.push(item);
        } else {
            col.insert(index, item);
        }

        unsafe {
            wh::send_message(handle, LB_INSERTSTRING, index, mem::transmute(display_os.as_ptr()));
        }
    }


    /// Remove the item at the selected index and returns it.
    /// Panic of the index is out of bounds
    pub fn remove(&self, index: usize) -> D {
        use winapi::um::winuser::LB_DELETESTRING;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LB_DELETESTRING, index as WPARAM, 0);

        let mut col_ref = self.collection.borrow_mut();
        col_ref.remove(index)
    }

    /// Return the index of the currencty selected item for single value list box.
    /// Return `None` if no item is selected.
    pub fn selection(&self) -> Option<usize> {
        use winapi::um::winuser::{LB_GETCURSEL, LB_ERR};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let index = wh::send_message(handle, LB_GETCURSEL , 0, 0);

        if index == LB_ERR { None }
        else { Some(index as usize) }
    }

    /// Return the number of selected item in the list box
    /// Returns 0 for single select list box
    pub fn multi_selection_len(&self) -> usize {
        use winapi::um::winuser::{LB_GETSELCOUNT, LB_ERR};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        match wh::send_message(handle, LB_GETSELCOUNT, 0, 0) {
            LB_ERR => 0,
            value => value as usize
        }
    }

    /// Return a list index
    /// Returns an empty vector for single select list box.
    pub fn multi_selection(&self) -> Vec<usize> {
        use winapi::um::winuser::{LB_GETSELCOUNT, LB_GETSELITEMS, LB_ERR};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let select_count = match wh::send_message(handle, LB_GETSELCOUNT, 0, 0) {
            LB_ERR => usize::max_value(),
            value => value as usize
        };

        if select_count == usize::max_value() || usize::max_value() == 0 {
            return Vec::new();
        }

        let mut indices_buffer: Vec<u32> = Vec::with_capacity(select_count);
        unsafe { indices_buffer.set_len(select_count) };

        wh::send_message(
            handle,
            LB_GETSELITEMS,
            select_count as WPARAM,
            indices_buffer.as_mut_ptr() as LPARAM
        );

        indices_buffer.into_iter().map(|i| i as usize).collect()
    }

    /// Return the display value of the currenctly selected item for single value
    /// Return `None` if no item is selected. This reads the visual value.
    pub fn selection_string(&self) -> Option<String> {
        use winapi::um::winuser::{LB_GETCURSEL, LB_GETTEXTLEN, LB_GETTEXT, LB_ERR};
        use winapi::shared::ntdef::WCHAR;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let index = wh::send_message(handle, LB_GETCURSEL, 0, 0);

        if index == LB_ERR { None }
        else {
            let index = index as usize;
            let length = (wh::send_message(handle, LB_GETTEXTLEN, index, 0) as usize) + 1;  // +1 for the terminating null character
            let mut buffer: Vec<WCHAR> = Vec::with_capacity(length);
            unsafe { 
                buffer.set_len(length); 
                wh::send_message(handle, LB_GETTEXT, index, mem::transmute(buffer.as_ptr()));
            }

            Some(from_utf16(&buffer))
        }
    }

    /// Set the currently selected item in the list box for single value list box.
    /// Does nothing if the index is out of bound
    /// If the value is None, remove the selected value
    pub fn set_selection(&self, index: Option<usize>) {
        use winapi::um::winuser::LB_SETCURSEL;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let index = index.unwrap_or(-1isize as usize);
        wh::send_message(handle, LB_SETCURSEL, index, 0);
    }

    /// Select the item as index `index` in a multi item list box
    pub fn multi_add_selection(&self, index: usize) {
        use winapi::um::winuser::LB_SETSEL;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LB_SETSEL, 1, index as LPARAM);
    }

    /// Unselect the item as index `index` in a multi item list box
    pub fn multi_remove_selection(&self, index: usize) {
        use winapi::um::winuser::LB_SETSEL;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LB_SETSEL, 0, index as LPARAM);
    }

    /// Unselect every item in the list box
    pub fn unselect_all(&self) {
        use winapi::um::winuser::LB_SETSEL;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LB_SETSEL, 0, -1);
    }

    /// Select every item in the list box
    pub fn select_all(&self) {
        use winapi::um::winuser::LB_SETSEL;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LB_SETSEL, 1, -1);
    }

    /// Select a range of items in a multi list box
    pub fn multi_select_range(&self, range: Range<usize>) {
        use winapi::um::winuser::LB_SELITEMRANGEEX;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let start = range.start as WPARAM;
        let end = range.end as LPARAM;
        wh::send_message(handle, LB_SELITEMRANGEEX, start, end);
    }

    /// Unselect a range of items in a multi list box
    pub fn multi_unselect_range(&self, range: Range<usize>) {
        use winapi::um::winuser::LB_SELITEMRANGEEX;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let start = range.start as LPARAM;
        let end = range.end as WPARAM;
        wh::send_message(handle, LB_SELITEMRANGEEX, end, start);
    }

    /// Search an item that begins by the value and select the first one found.
    /// The search is not case sensitive, so this string can contain any combination of uppercase and lowercase letters.
    /// Return the index of the selected string or None if the search was not successful
    pub fn set_selection_string(&self, value: &str) -> Option<usize> {
        use winapi::um::winuser::{LB_SELECTSTRING, LB_ERR};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let os_string = to_utf16(value);

        unsafe {
            let index = wh::send_message(handle, LB_SELECTSTRING, 0, mem::transmute(os_string.as_ptr()));
            if index == LB_ERR {
                None
            } else {
                Some(index as usize)
            }
        }
    }

    /// Check if the item at `index` is selected by the user
    /// Return `false` if the index is out of range.
    pub fn selected(&self, index: usize) -> bool {
        use winapi::um::winuser::LB_GETSEL;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LB_GETSEL, index as WPARAM, 0) > 0
    }

    /// Update the visual of the control with the inner collection.
    /// This rebuild every item in the list box and can take some time on big collections.
    pub fn sync(&self) {
        use winapi::um::winuser::{LB_ADDSTRING, LB_INITSTORAGE};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        self.clear_inner(handle);

        let item_count = self.collection.borrow().len();
        wh::send_message(handle, LB_INITSTORAGE, item_count as WPARAM, (10*item_count) as LPARAM);

        for item in self.collection.borrow().iter() {
            let display = format!("{}", item);
            let display_os = to_utf16(&display);
            
            unsafe {
                wh::send_message(handle, LB_ADDSTRING, 0, mem::transmute(display_os.as_ptr()));
            }
        }
    }

    /// Set the item collection of the list box. Return the old collection
    pub fn set_collection(&self, mut col: Vec<D>) -> Vec<D> {
        use winapi::um::winuser::LB_ADDSTRING;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        self.clear_inner(handle);

        for item in col.iter() {
            let display = format!("{}", item);
            let display_os = to_utf16(&display);
            
            unsafe {
                wh::send_message(handle, LB_ADDSTRING, 0, mem::transmute(display_os.as_ptr()));
            }
        }

        let mut col_ref = self.collection.borrow_mut();
        mem::swap::<Vec<D>>(&mut col_ref, &mut col);

        col
    }

    /// Clears the control and free the underlying collection. Same as `set_collection(Vec::new())`
    pub fn clear(&self) {
        self.set_collection(Vec::new());
    }

    /// Return the number of items in the control. NOT the inner rust collection
    pub fn len(&self) -> usize {
        use winapi::um::winuser::LB_GETCOUNT;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, LB_GETCOUNT, 0, 0) as usize
    }


    //
    // Common control functions
    //

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

    /// Get read-only access to the inner collection of the list box
    /// This call refcell.borrow under the hood. Be sure to drop the value before
    /// calling other list box methods
    pub fn collection(&self) -> Ref<Vec<D>> {
        self.collection.borrow()
    }

    /// Get mutable access to the inner collection of the list box. Does not update the visual
    /// control. Call `sync` to update the view. This call refcell.borrow_mut under the hood. 
    /// Be sure to drop the value before calling other list box methods
    pub fn collection_mut(&self) -> RefMut<Vec<D>> {
        self.collection.borrow_mut()
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "ListBox"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | WS_TABSTOP
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{LBS_HASSTRINGS, WS_BORDER, WS_VSCROLL, LBS_NOTIFY, WS_CHILD};

        LBS_HASSTRINGS | LBS_NOTIFY | WS_BORDER  | WS_CHILD | WS_VSCROLL
    }

    /// Remove all value displayed in the control without touching the rust collection
    fn clear_inner(&self, handle: HWND) {
        use winapi::um::winuser::LB_RESETCONTENT;
        wh::send_message(handle, LB_RESETCONTENT, 0, 0);
    }

}

impl<D: Display+Default> Drop for ListBox<D> {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

pub struct ListBoxBuilder<'a, D: Display+Default> {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    focus: bool,
    flags: Option<ListBoxFlags>,
    ex_flags: u32,
    font: Option<&'a Font>,
    collection: Option<Vec<D>>,
    selected_index: Option<usize>,
    multi_selection: Vec<usize>,
    parent: Option<ControlHandle>
}

impl<'a, D: Display+Default> ListBoxBuilder<'a, D> {

    pub fn flags(mut self, flags: ListBoxFlags) -> ListBoxBuilder<'a, D> {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> ListBoxBuilder<'a, D> {
        self.ex_flags = flags;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ListBoxBuilder<'a, D> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> ListBoxBuilder<'a, D> {
        self.position = pos;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> ListBoxBuilder<'a, D> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ListBoxBuilder<'a, D> {
        self.parent = Some(p.into());
        self
    }

    pub fn collection(mut self, collection: Vec<D>) -> ListBoxBuilder<'a, D> {
        self.collection = Some(collection);
        self
    }

    pub fn selected_index(mut self, index: Option<usize>) -> ListBoxBuilder<'a, D> {
        self.selected_index = index;
        self
    }

    pub fn multi_selection(mut self, select: Vec<usize>) -> ListBoxBuilder<'a, D> {
        self.multi_selection = select;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> ListBoxBuilder<'a, D> {
        self.enabled = enabled;
        self
    }

    pub fn focus(mut self, focus: bool) -> ListBoxBuilder<'a, D> {
        self.focus = focus;
        self
    }

    pub fn build(self, out: &mut ListBox<D>) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("ListBox"))
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

        if let Some(col) = self.collection {
            out.set_collection(col);
        }

        if flags & LBS_MULTIPLESEL == LBS_MULTIPLESEL {
            for i in self.multi_selection {
                out.multi_add_selection(i);
            }
        } else {
            out.set_selection(self.selected_index);
        }

        if self.focus {
            out.set_focus();
        }

        if !self.enabled {
            out.set_enabled(self.enabled);
        }

        Ok(())
    }

}

impl<D: Display+Default> PartialEq for ListBox<D> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
