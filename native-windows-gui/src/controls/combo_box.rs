use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{LPARAM, WPARAM};
use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED};
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::win32::window_helper as wh;
use crate::{Font, NwgError};
use super::{ControlHandle, ControlBase};
use std::cell::{Ref, RefMut, RefCell};
use std::fmt::Display;
use std::mem;

const NOT_BOUND: &'static str = "Combobox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Combobox handle is not HWND!";


bitflags! {
    /**
        The ComboBox flags

        * NONE:     No flags. Equivalent to a invisible combobox.
        * VISIBLE:  The combobox is immediatly visible after creation
        * DISABLED: The combobox cannot be interacted with by the user. It also has a grayed out look.
    */
    pub struct ComboBoxFlags: u32 {
        const NONE = 0;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/**
A combo box consists of a list and a selection field. The list presents the options that a user can select,
and the selection field displays the current selection.

Requires the `combobox` feature. 

**Builder parameters:**
  * `parent`:         **Required.** The combobox parent container.
  * `size`:           The combobox size.
  * `position`:       The combobox position.
  * `enabled`:        If the combobox can be used by the user. It also has a grayed out look if disabled.
  * `flags`:          A combination of the ComboBoxFlags values.
  * `font`:           The font used for the combobox text
  * `collection`:     The default collection of the combobox
  * `selected_index`: The default selected index. None means no values are selected.  

**Control events:**
  * `OnComboBoxClosed`: When the combobox dropdown is closed
  * `OnComboBoxDropdown`: When the combobox dropdown is opened
  * `OnComboxBoxSelection`: When a new value in a combobox is choosen
  * `MousePress(_)`: Generic mouse press events on the checkbox
  * `OnMouseMove`: Generic mouse mouse event


```rust
use native_windows_gui as nwg;
fn build_combobox(combo: &mut nwg::ComboBox<&'static str>, window: &nwg::Window) {
    let data = vec!["one", "two"];
    
    nwg::ComboBox::builder()
        .size((200, 300))
        .collection(data)
        .selected_index(Some(0))
        .parent(window)
        .build(combo);
}
```
*/
#[derive(Default)]
pub struct ComboBox<D: Display+Default> {
    pub handle: ControlHandle,
    collection: RefCell<Vec<D>>
}

impl<D: Display+Default> ComboBox<D> {

    pub fn builder<'a>() -> ComboBoxBuilder<'a, D> {
        ComboBoxBuilder {
            size: (100, 25),
            position: (0, 0),
            enabled: true,
            flags: None,
            font: None,
            collection: None,
            selected_index: None,
            parent: None
        }
    }

    /// Remove the item at the selected index and returns it.
    /// Panic of the index is out of bounds
    pub fn remove(&self, index: usize) -> D {
        use winapi::um::winuser::CB_DELETESTRING;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, CB_DELETESTRING, index as WPARAM, 0);

        let mut col_ref = self.collection.borrow_mut();
        col_ref.remove(index)
    }

    /// Sort the inner collection by the display value of it's items and update the view
    /// Internally this uses `Vec.sort_unstable_by`.
    pub fn sort(&self) {
        use winapi::um::winuser::{CB_ADDSTRING};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        self.clear_inner(handle);

        let mut col = self.collection.borrow_mut();
        col.sort_unstable_by(|a, b| {
            let astr = format!("{}", a);
            let bstr = format!("{}", b);
            astr.cmp(&bstr)
        });

        for item in col.iter() {
            let display = format!("{}", item);
            let display_os = to_utf16(&display);
            wh::send_message(handle, CB_ADDSTRING, 0, display_os.as_ptr() as LPARAM);
        }
    }

    /// Show or hide the dropdown of the combox
    pub fn dropdown(&self, v: bool) {
        use winapi::um::winuser::{CB_SHOWDROPDOWN};
        
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, CB_SHOWDROPDOWN, v as usize, 0);
    }

    /// Return the index of the currencty selected item. Return `None` if no item is selected.
    pub fn selection(&self) -> Option<usize> {
        use winapi::um::winuser::{CB_GETCURSEL, CB_ERR};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let index = wh::send_message(handle, CB_GETCURSEL, 0, 0);

        if index == CB_ERR { None }
        else { Some(index as usize) }
    }

    /// Return the display value of the currenctly selected item
    /// Return `None` if no item is selected. This reads the visual value.
    pub fn selection_string(&self) -> Option<String> {
        use winapi::um::winuser::{CB_GETCURSEL, CB_GETLBTEXTLEN, CB_GETLBTEXT, CB_ERR};
        use winapi::shared::ntdef::WCHAR;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let index = wh::send_message(handle, CB_GETCURSEL, 0, 0);

        if index == CB_ERR { None }
        else {
            let index = index as usize;
            let length = wh::send_message(handle, CB_GETLBTEXTLEN, index, 0) as usize;
            let mut buffer: Vec<WCHAR> = Vec::with_capacity(length);
            unsafe { 
                buffer.set_len(length); 
                wh::send_message(handle, CB_GETLBTEXT, index, buffer.as_ptr() as LPARAM);
            }

            Some(from_utf16(&buffer))
        }
    }

    /// Set the currently selected item in the combobox.
    /// Does nothing if the index is out of bound
    /// If the value is None, remove the selected value
    pub fn set_selection(&self, index: Option<usize>) {
        use winapi::um::winuser::CB_SETCURSEL;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let index = index.unwrap_or(-1isize as usize);
        wh::send_message(handle, CB_SETCURSEL, index, 0);
    }

    /// Search an item that begins by the value and select the first one found.
    /// The search is not case sensitive, so this string can contain any combination of uppercase and lowercase letters.
    /// Return the index of the selected string or None if the search was not successful
    pub fn set_selection_string(&self, value: &str) -> Option<usize> {
        use winapi::um::winuser::{CB_SELECTSTRING, CB_ERR};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        
        let os_string = to_utf16(value);

        let index = wh::send_message(handle, CB_SELECTSTRING, 0, os_string.as_ptr() as LPARAM);
        if index == CB_ERR {
            None
        } else {
            Some(index as usize)
        }
    }

    /// Add a new item to the combobox. Sort the collection if the combobox is sorted.
    pub fn push(&self, item: D) {
        use winapi::um::winuser::CB_ADDSTRING;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let display = format!("{}", item);
        let display_os = to_utf16(&display);

        wh::send_message(handle, CB_ADDSTRING, 0, display_os.as_ptr() as LPARAM);

        self.collection.borrow_mut().push(item);
    }

    /// Insert an item in the collection and the control.
    ///
    /// SPECIAL behaviour! If index is `std::usize::MAX`, the item is added at the end of the collection.
    /// The method will still panic if `index > len` with every other values.
    pub fn insert(&self, index: usize, item: D) {
        use winapi::um::winuser::CB_INSERTSTRING;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let display = format!("{}", item);
        let display_os = to_utf16(&display);

        let mut col = self.collection.borrow_mut();
        if index == std::usize::MAX {
            col.push(item);
        } else {
            col.insert(index, item);
        }

        wh::send_message(handle, CB_INSERTSTRING, index, display_os.as_ptr() as LPARAM);
    }

    /// Update the visual of the control with the inner collection.
    /// This rebuild every item in the combobox and can take some time on big collections.
    pub fn sync(&self) {
        use winapi::um::winuser::CB_ADDSTRING;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        self.clear_inner(handle);

        for item in self.collection.borrow().iter() {
            let display = format!("{}", item);
            let display_os = to_utf16(&display);
            
            wh::send_message(handle, CB_ADDSTRING, 0, display_os.as_ptr() as LPARAM);
        }
    }

    /// Set the item collection of the combobox. Return the old collection
    pub fn set_collection(&self, mut col: Vec<D>) -> Vec<D> {
        use winapi::um::winuser::CB_ADDSTRING;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        self.clear_inner(handle);

        for item in col.iter() {
            let display = format!("{}", item);
            let display_os = to_utf16(&display);
            wh::send_message(handle, CB_ADDSTRING, 0, display_os.as_ptr() as LPARAM);
        }

        let mut col_ref = self.collection.borrow_mut();
        mem::swap::<Vec<D>>(&mut col_ref, &mut col);

        col
    }

    /// Return the number of items in the control. NOT the inner rust collection
    pub fn len(&self) -> usize {
        use winapi::um::winuser::CB_GETCOUNT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let count = wh::send_message(handle, CB_GETCOUNT, 0, 0);

        count as usize
    }

    //
    // Common control functions
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

    /// Get read-only access to the inner collection of the combobox
    /// This call refcell.borrow under the hood. Be sure to drop the value before
    /// calling other combobox methods
    pub fn collection(&self) -> Ref<Vec<D>> {
        self.collection.borrow()
    }

    /// Get mutable access to the inner collection of the combobox. Does not update the visual
    /// control. Call `sync` to update the view. This call refcell.borrow_mut under the hood. 
    /// Be sure to drop the value before calling other combobox methods
    pub fn collection_mut(&self) -> RefMut<Vec<D>> {
        self.collection.borrow_mut()
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "COMBOBOX"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{CBS_DROPDOWNLIST, WS_BORDER, WS_CHILD};
        CBS_DROPDOWNLIST | WS_CHILD | WS_BORDER
    }

    /// Remove all value displayed in the control without touching the rust collection
    fn clear_inner(&self, handle: HWND) {
        use winapi::um::winuser::CB_RESETCONTENT;
        wh::send_message(handle, CB_RESETCONTENT, 0, 0);
    }

}


pub struct ComboBoxBuilder<'a, D: Display+Default> {
    size: (i32, i32),
    position: (i32, i32),
    enabled: bool,
    flags: Option<ComboBoxFlags>,
    font: Option<&'a Font>,
    collection: Option<Vec<D>>,
    selected_index: Option<usize>,
    parent: Option<ControlHandle>
}

impl<'a, D: Display+Default> ComboBoxBuilder<'a, D> {

    pub fn flags(mut self, flags: ComboBoxFlags) -> ComboBoxBuilder<'a, D> {
        self.flags = Some(flags);
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> ComboBoxBuilder<'a, D> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> ComboBoxBuilder<'a, D> {
        self.position = pos;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> ComboBoxBuilder<'a, D> {
        self.font = font;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> ComboBoxBuilder<'a, D> {
        self.parent = Some(p.into());
        self
    }

    pub fn collection(mut self, collection: Vec<D>) -> ComboBoxBuilder<'a, D> {
        self.collection = Some(collection);
        self
    }

    pub fn selected_index(mut self, index: Option<usize>) -> ComboBoxBuilder<'a, D> {
        self.selected_index = index;
        self
    }

    pub fn enabled(mut self, e: bool) -> ComboBoxBuilder<'a, D> {
        self.enabled = e;
        self
    }

    pub fn build(self, out: &mut ComboBox<D>) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("ComboBox"))
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

        if self.collection.is_some() {
            out.set_collection(self.collection.unwrap());
        }

        if self.selected_index.is_some() {
            out.set_selection(self.selected_index);
        }

        out.set_enabled(self.enabled);

        Ok(())
    }

}

impl<D: Display+Default> PartialEq for ComboBox<D> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
