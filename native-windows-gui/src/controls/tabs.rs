use winapi::shared::minwindef::{WPARAM, LPARAM, BOOL};
use winapi::shared::windef::HWND;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{EnumChildWindows, WS_VISIBLE, WS_DISABLED, WS_EX_CONTROLPARENT};
use crate::win32::{base_helper::{to_utf16, check_hwnd}, window_helper as wh};
use crate::{NwgError, Font, RawEventHandler, unbind_raw_event_handler};
use super::{ControlBase, ControlHandle};
use std::{mem, cell::RefCell};

#[cfg(feature="image-list")]
use crate::ImageList;

#[cfg(feature="image-list")]
use std::ptr;

const NOT_BOUND: &'static str = "TabsContainer/Tab is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TabsContainer/Tab handle is not HWND!";


bitflags! {
    pub struct TabsContainerFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/**
A tabs container is a frame-like control that can contain `Tab` control.
Tabs are added by specifying the `TabsContainer` as parent in the `Tab` builder.

Do not add other control type as children to the TabsContainer

Requires the `tabs` feature

**Builder parameters:**
  * `parent`:     **Required.** The button parent container.
  * `position`:   The tab container position.
  * `font`:       The font used for the tabs title
  * `flags`:      A combination of the `TabsContainerFlags` values.
  * `ex_flags`: A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `image_list`: The image list specifying the tabs icons


**Control events:**
  * `TabsContainerChanged`: The select tab of a TabsContainer changed
  * `TabsContainerChanging`: The selected tab of a TabsContainer is about to be changed
  * `MousePress(_)`: Generic mouse press events on the button
  * `OnMouseMove`: Generic mouse mouse event
  * `OnMouseWheel`: Generic mouse wheel event

*/
#[derive(Default)]
pub struct TabsContainer {
    pub handle: ControlHandle,
    handler0: RefCell<Option<RawEventHandler>>,
    handler1: RefCell<Option<RawEventHandler>>,
}

impl TabsContainer {

    pub fn builder<'a>() -> TabsContainerBuilder<'a> {
        TabsContainerBuilder {
            size: (300, 300),
            position: (0, 0),
            parent: None,
            font: None,
            flags: None,
            ex_flags: 0,

            #[cfg(feature = "image-list")]
            image_list: None
        }
    }

    /// Return the index of the currently selected tab
    /// May return `usize::max_value()` if no tab is selected
    pub fn selected_tab(&self) -> usize {
        use winapi::um::commctrl::{TCM_GETCURSEL};
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TCM_GETCURSEL, 0, 0) as usize
    }

    /// Set the currently selected tab by index
    pub fn set_selected_tab(&self, index: usize) {
        use winapi::um::commctrl::TCM_SETCURSEL;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TCM_SETCURSEL, index as WPARAM, 0);

        // Update the visible state of the tabs (this is not done automatically)
        let data: (HWND, i32) = (handle, index as i32);
        let data_ptr = &data as *const (HWND, i32);
        
        unsafe {
            EnumChildWindows(handle, Some(toggle_children_tabs), data_ptr as LPARAM);
        }
    }

    /// Return the number of tabs in the view
    pub fn tab_count(&self) -> usize {
        use winapi::um::commctrl::TCM_GETITEMCOUNT;
            
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TCM_GETITEMCOUNT, 0, 0) as usize
    }

    /**
        Sets the image list of the tab container. Pass None to remove the image list.

        This is only available is the feature "image-list" is enabled.
    */
    #[cfg(feature = "image-list")]
    pub fn set_image_list(&self, list: Option<&ImageList>) {
        use winapi::um::commctrl::TCM_SETIMAGELIST;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let list_handle = match list {
            None => 0,
            Some(list) => list.handle as _
        };

        wh::send_message(handle, TCM_SETIMAGELIST, 0, list_handle);
    }

    /**
        Returns a reference to the current image list in the tab container. The image list
        is not owned and dropping it won't free the resources.

        This is only available is the feature "image-list" is enabled.
    */
    #[cfg(feature = "image-list")]
    pub fn image_list(&self) -> Option<ImageList> {
        use winapi::um::commctrl::TCM_GETIMAGELIST;

        let control_handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let handle = wh::send_message(control_handle, TCM_GETIMAGELIST, 0, 0);
        match handle == 0 {
            true => None,
            false => Some(ImageList {
                handle: handle as _,
                owned: false,
            })
        }
    }

    //
    // Default methods
    //

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

    /// Return the size of the tabs container in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the tabs container in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the tabs container in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the tabs container in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
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

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        winapi::um::commctrl::WC_TABCONTROL
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD, WS_CLIPCHILDREN};
        //use winapi::um::commctrl::TCS_OWNERDRAWFIXED;

        WS_CHILD | WS_CLIPCHILDREN //| TCS_OWNERDRAWFIXED
    }

    //
    // Private
    //

    /// The tab widget lacks basic functionalities on it's own. This fix it. 
    fn hook_tabs(&self) {
        use crate::bind_raw_event_handler_inner;
        use winapi::shared::minwindef::{HIWORD, LOWORD};
        use winapi::um::winuser::{NMHDR, WM_SIZE, WM_NOTIFY};
        use winapi::um::commctrl::{TCM_GETCURSEL, TCN_SELCHANGE};
        use winapi::um::winuser::SendMessageW;

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let parent_handle_raw = wh::get_window_parent(handle);
        let parent_handle = ControlHandle::Hwnd(parent_handle_raw);
       
        let handler0 = bind_raw_event_handler_inner(&parent_handle, handle as usize, move |_hwnd, msg, _w, l| { unsafe {
            match msg {
                WM_NOTIFY => {
                    let nmhdr: &NMHDR = mem::transmute(l);
                    if nmhdr.code == TCN_SELCHANGE {
                        let index = SendMessageW(handle, TCM_GETCURSEL, 0, 0) as i32;
                        let data: (HWND, i32) = (handle, index);
                        let data_ptr = &data as *const (HWND, i32);
                        EnumChildWindows(handle, Some(toggle_children_tabs), data_ptr as LPARAM);
                    }
                },
                _ => {}
            }

            None
        } });

        let handler1 = bind_raw_event_handler_inner(&self.handle, handle as usize, move |hwnd, msg, _w, l| { unsafe {
            match msg {
                WM_SIZE => {
                    use winapi::shared::windef::{RECT, HGDIOBJ};
                    use winapi::um::winuser::{GetDC, DrawTextW, ReleaseDC, DT_CALCRECT, DT_LEFT};
                    use winapi::um::wingdi::SelectObject;

                    let size = l as u32;
                    let width = LOWORD(size) as i32;
                    let height = HIWORD(size) as i32;
                    let (w, h) = crate::win32::high_dpi::physical_to_logical(width, height);

                    let mut data = ResizeDirectChildrenParams {
                        parent: hwnd,
                        width: w as u32,
                        height: h as u32,
                        tab_offset_y: 0
                    };

                    // Get the height of the tabs
                    let font_handle = wh::get_window_font(hwnd);
                    let mut r: RECT = mem::zeroed();
                    let dc = GetDC(hwnd);
                    let old = SelectObject(dc, font_handle as HGDIOBJ);
                    let calc: [u16;2] = [75, 121];
                    DrawTextW(dc, calc.as_ptr(), 2, &mut r, DT_CALCRECT | DT_LEFT);
                    SelectObject(dc, old);
                    ReleaseDC(hwnd, dc);

                    // Fix the width/height of the tabs
                    const BORDER_SIZE: u32 = 11;
                    let tab_height = r.bottom as u32 + BORDER_SIZE;
                    if data.width > BORDER_SIZE { data.width -= BORDER_SIZE; }
                    if data.height > tab_height { 
                        data.height -= (tab_height + BORDER_SIZE).min(data.height);
                    }
                    data.tab_offset_y = tab_height;
                    
                    let data_ptr = &data as *const ResizeDirectChildrenParams;
                    EnumChildWindows(hwnd, Some(resize_direct_children), mem::transmute(data_ptr));
                },
                _ => {}
            }

            None
        } } );

        *self.handler0.borrow_mut() = Some(handler0.unwrap());
        *self.handler1.borrow_mut() = Some(handler1.unwrap());
    }
}

impl Drop for TabsContainer {
    fn drop(&mut self) {
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            drop(unbind_raw_event_handler(h));
        }

        let handler = self.handler1.borrow();
        if let Some(h) = handler.as_ref() {
            drop(unbind_raw_event_handler(h));
        }
    
        self.handle.destroy();
    }
}

impl PartialEq for TabsContainer {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}


pub struct TabsContainerBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    parent: Option<ControlHandle>,
    font: Option<&'a Font>,
    flags: Option<TabsContainerFlags>,
    ex_flags: u32,

    #[cfg(feature = "image-list")]
    image_list: Option<&'a ImageList>
}

impl<'a> TabsContainerBuilder<'a> {

    pub fn flags(mut self, flags: TabsContainerFlags) -> TabsContainerBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> TabsContainerBuilder<'a> {
        self.ex_flags = flags;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> TabsContainerBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> TabsContainerBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TabsContainerBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> TabsContainerBuilder<'a> {
        self.font = font;
        self
    }

    #[cfg(feature = "image-list")]
    pub fn image_list(mut self, list: Option<&'a ImageList>) -> TabsContainerBuilder<'a> {
        self.image_list = list;
        self
    }

    pub fn build(self, out: &mut TabsContainer) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("TabsContainer"))
        }?;

        *out = Default::default();

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .ex_flags(WS_EX_CONTROLPARENT | self.ex_flags)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        out.hook_tabs();

        if self.font.is_some() {
            out.set_font(self.font);
        } else {
            out.set_font(Font::global_default().as_ref());
        }

        // Image list
        #[cfg(feature = "image-list")]
        fn set_image_list(b: &TabsContainerBuilder, out: &mut TabsContainer) {
            if b.image_list.is_some() {
                out.set_image_list(b.image_list);
            }
        }

        #[cfg(not(feature = "image-list"))]
        fn set_image_list(_b: &TabsContainerBuilder, _out: &mut TabsContainer) {}

        set_image_list(&self, out);

        Ok(())
    }
}


/**
A subwindow in a `TabContainer` widget. A Tab control can only be added as a child of a `TabContainer`. 

A Tab controls doesn't do much on its own. See `TabContainer` for the tab specific events.

**Builder parameters:**
  * `parent`:      **Required.** The Tab parent container.
  * `text`:        The tab text
  * `image_index`: The tab icon index in the tab container image list
*/
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Tab {
    pub handle: ControlHandle
}

impl Tab {

    pub fn builder<'a>() -> TabBuilder<'a> {
        TabBuilder {
            text: "Tab",
            parent: None,

            #[cfg(feature = "image-list")]
            image_index: None,
        }
    }

    /// Sets the title of the tab
    pub fn set_text<'a>(&self, text: &'a str) {
        use winapi::um::commctrl::{TCM_SETITEMW, TCIF_TEXT, TCITEMW};
        use winapi::um::winuser::GWL_USERDATA;

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let tab_index = (wh::get_window_long(handle, GWL_USERDATA) - 1) as WPARAM;

        let tab_view_handle = wh::get_window_parent(handle);

        let text = to_utf16(text);
        let item = TCITEMW {
            mask: TCIF_TEXT,
            dwState: 0,
            dwStateMask: 0,
            pszText: text.as_ptr() as LPWSTR,
            cchTextMax: 0,
            iImage: -1,
            lParam: 0
        };

        let item_ptr = &item as *const TCITEMW;
        wh::send_message(tab_view_handle, TCM_SETITEMW, tab_index, item_ptr as LPARAM);
    }

    /**
        Sets the image of the tab. index is the index of the image in the tab container image list.

        This is only available if the "image-list" feature is enabled
    */
    #[cfg(feature = "image-list")]
    pub fn set_image_index(&self, index: Option<i32>) {
        use winapi::um::commctrl::{TCM_SETITEMW, TCIF_IMAGE, TCITEMW};
        use winapi::um::winuser::GWL_USERDATA;

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let tab_index = (wh::get_window_long(handle, GWL_USERDATA) - 1) as WPARAM;
        let tab_view_handle = wh::get_window_parent(handle);

        let item = TCITEMW {
            mask: TCIF_IMAGE,
            dwState: 0,
            dwStateMask: 0,
            pszText: ptr::null_mut(),
            cchTextMax: 0,
            iImage: index.unwrap_or(-1),
            lParam: 0
        };

        let item_ptr = &item as *const TCITEMW;
        wh::send_message(tab_view_handle, TCM_SETITEMW, tab_index, item_ptr as LPARAM);
    }

    /**
        Returns the index of image of the tab.
        The index maps to the image list of the tab container.

        This is only available if the "image-list" feature is enabled
    */
    #[cfg(feature = "image-list")]
    pub fn image_index(&self) -> Option<i32> {
        None
    }

    /// Returns true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }
    

    //
    // Other methods
    //

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "NWG_TAB"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        0
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        //use winapi::um::commctrl::{TCS_SINGLELINE};
        use winapi::um::winuser::{WS_CHILD, WS_CLIPCHILDREN};

        WS_CHILD | WS_CLIPCHILDREN
    }

    /// Set and initialize a tab as active
    unsafe fn init(current_handle: HWND, tab_view_handle: HWND, index: usize) {
        use winapi::um::winuser::GWL_USERDATA;

        // Save the index of the tab in the window data
        wh::set_window_long(current_handle, GWL_USERDATA, index);

        // Resize the tabs so that they match the tab view size and hide all children tabs
        let (w, h) = wh::get_window_size(tab_view_handle);
        let width = w - 11;
        let height = h - 33;

        // Resize the tab to match the tab view
        wh::set_window_size(current_handle, width, height, false);

        // Move the tab under the headers
        wh::set_window_position(current_handle, 5, 25);

        // Make the current tab visible
        if index == 1 {
            wh::set_window_visibility(current_handle, true);
        }
    }

    fn next_index(tab_view_handle: HWND) -> usize {
        let mut count = 0;
        let count_ptr = &mut count as *mut usize;

        unsafe {
            EnumChildWindows(tab_view_handle, Some(count_children), mem::transmute(count_ptr));
        }

        count
    }

    /// Bind the tab to a tab view
    fn bind_container<'a>(&self, text: &'a str) {
        use winapi::um::commctrl::{TCITEMW, TCM_INSERTITEMW, TCIF_TEXT};

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let tab_view_handle = wh::get_window_parent(handle);
        let next_index = Tab::next_index(tab_view_handle);

        unsafe {
            Tab::init(handle, tab_view_handle, next_index);
        }

        let text = to_utf16(&text);
        let tab_info = TCITEMW {
            mask: TCIF_TEXT,
            dwState: 0,
            dwStateMask: 0,
            pszText: text.as_ptr() as LPWSTR,
            cchTextMax: 0,
            iImage: -1,
            lParam: 0
        };

        let tab_info_ptr = &tab_info as *const TCITEMW;
        wh::send_message(tab_view_handle, TCM_INSERTITEMW, next_index as WPARAM, tab_info_ptr as LPARAM);
    }

}

impl Drop for Tab {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

pub struct TabBuilder<'a> {
    text: &'a str,
    parent: Option<ControlHandle>,

    #[cfg(feature = "image-list")]
    image_index: Option<i32>,
}

impl<'a> TabBuilder<'a> {

    pub fn text(mut self, text: &'a str) -> TabBuilder<'a> {
        self.text = text;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TabBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    #[cfg(feature = "image-list")]
    pub fn image_index(mut self, index: Option<i32>) -> TabBuilder<'a> {
        self.image_index = index;
        self
    }

    pub fn build(self, out: &mut Tab) -> Result<(), NwgError> {
        use winapi::um::commctrl::WC_TABCONTROL;

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("Tab"))
        }?;

        *out = Default::default();

        match &parent {
            &ControlHandle::Hwnd(h) => {
                let class_name = unsafe { wh::get_window_class_name(h) };
                if &class_name != WC_TABCONTROL {
                    Err(NwgError::control_create("Tab requires a TabsContainer parent."))
                } else {
                    Ok(())
                }
            },
            _ => Err(NwgError::control_create("Tab requires a TabsContainer parent."))
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .ex_flags(WS_EX_CONTROLPARENT)
            .flags(out.flags())
            .text(self.text)
            .parent(Some(parent))
            .build()?;

        out.bind_container(self.text);

        // Image index

        #[cfg(feature = "image-list")]
        fn set_image_index<'a>(b: &TabBuilder<'a>, out: &mut Tab) {
            if b.image_index.is_some() {
                out.set_image_index(b.image_index);
            }
        }

        #[cfg(not(feature = "image-list"))]
        fn set_image_index<'a>(_b: &TabBuilder<'a>, _out: &mut Tab) {}

        set_image_index(&self, out);

        Ok(())
    }
}


struct ResizeDirectChildrenParams {
    parent: HWND,
    width: u32,
    height: u32,
    tab_offset_y: u32
}

unsafe extern "system" fn resize_direct_children(handle: HWND, params: LPARAM) -> BOOL {
    let params: &ResizeDirectChildrenParams = &*(params as *const ResizeDirectChildrenParams);
    if wh::get_window_parent(handle) == params.parent {
        wh::set_window_size(handle, params.width, params.height, false);

        let (x, _y) = wh::get_window_position(handle);
        wh::set_window_position(handle, x, params.tab_offset_y as i32);
    }

    1
}

unsafe extern "system" fn count_children(handle: HWND, params: LPARAM) -> BOOL {
    use winapi::um::winuser::GWL_USERDATA;

    if &wh::get_window_class_name(handle) == "NWG_TAB" {
        let tab_index = (wh::get_window_long(handle, GWL_USERDATA)) as WPARAM;
        let count: &mut usize = ::std::mem::transmute(params);
        *count = usize::max(tab_index+1, *count);
    }
    
    1
}

/// Toggle the visibility of the active and inactive tab.
unsafe extern "system" fn toggle_children_tabs(handle: HWND, params: LPARAM) -> BOOL {
    use winapi::um::winuser::GWL_USERDATA;
    
    let &(parent, index): &(HWND, i32) = mem::transmute(params);
    if wh::get_window_parent(handle) == parent {
        let tab_index = wh::get_window_long(handle, GWL_USERDATA) as i32;
        let visible = tab_index == index + 1;
        wh::set_window_visibility(handle, visible);
    }

    1
}
