use winapi::shared::minwindef::{WPARAM, LPARAM, BOOL};
use winapi::shared::windef::HWND;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::{EnumChildWindows, WS_VISIBLE, WS_DISABLED, WS_EX_COMPOSITED};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16};
use crate::{SystemError, Font};
use super::{ControlBase, ControlHandle};
use std::mem;

const NOT_BOUND: &'static str = "TabsContainer/Tab is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TabsContainer/Tab handle is not HWND!";


bitflags! {
    pub struct TabsContainerFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}

/**
A push button is a rectangle containing an application-defined text label, an icon, or a bitmap
that indicates what the button does when the user selects it.
*/
#[derive(Default, Debug)]
pub struct TabsContainer {
    pub handle: ControlHandle,
}

impl TabsContainer {

    pub fn builder<'a>() -> TabsContainerBuilder<'a> {
        TabsContainerBuilder {
            size: (300, 300),
            position: (0, 0),
            parent: None,
            font: None,
            flags: None,
        }
    }

    /// Return the index of the currently selected tab
    /// May return `usize::max_value()` if no tab is selected
    pub fn selected_tab(&self) -> usize {
        use winapi::um::commctrl::{TCM_GETCURSEL};
        
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TCM_GETCURSEL, 0, 0) as usize
    }

    /// Set the currently selected tab by index
    pub fn set_selected_tab(&self, index: usize) {
        use winapi::um::commctrl::{TCM_SETCURSEL};

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

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
        use winapi::um::commctrl::{TCM_GETITEMCOUNT};
            
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, TCM_GETITEMCOUNT, 0, 0) as usize
    }

    //
    // Default methods
    //

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

    /// Return the size of the tabs container in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the tabs container in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the tabs container in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the tabs container in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

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

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        use winapi::um::commctrl::WC_TABCONTROL;
        Some(WC_TABCONTROL)
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        //use winapi::um::commctrl::{TCS_SINGLELINE};
        use winapi::um::winuser::{WS_CHILD};

        WS_CHILD
    }

    //
    // Private
    //

    /// The tab widget lacks basic functionalities on it's own. This fix it. 
    fn hook_tabs(&self) {
        use crate::bind_raw_event_handler;
        use winapi::shared::minwindef::{HIWORD, LOWORD};
        use winapi::um::winuser::{NMHDR, WM_SIZE, WM_NOTIFY};
        use winapi::um::commctrl::{TCM_GETCURSEL, TCN_SELCHANGE};
        use winapi::um::winuser::{SendMessageW};
        

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let parent_handle = ControlHandle::Hwnd(wh::get_window_parent(handle));

        bind_raw_event_handler(&parent_handle, 0, move |_hwnd, msg, _w, l| { unsafe {
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

        bind_raw_event_handler(&self.handle, 0, move |hwnd, msg, _w, l| { unsafe {
            match msg {
                WM_SIZE => {
                    let mut data = (hwnd, LOWORD(l as u32) as u32, HIWORD(l as u32) as u32);
                    
                    if data.1 > 11 { data.1 -= 11; }
                    if data.2 > 30 { data.2 -= 30; }
                    
                    let data_ptr = &data as *const (HWND, u32, u32);
                    EnumChildWindows(hwnd, Some(resize_direct_children), mem::transmute(data_ptr));
                },
                _ => {}
            }

            None
        } } );
    }
}


pub struct TabsContainerBuilder<'a> {
    size: (i32, i32),
    position: (i32, i32),
    parent: Option<ControlHandle>,
    font: Option<&'a Font>,
    flags: Option<TabsContainerFlags>,
}

impl<'a> TabsContainerBuilder<'a> {

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

    pub fn build(self, out: &mut TabsContainer) -> Result<(), SystemError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(SystemError::ControlWithoutParent)
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .ex_flags(WS_EX_COMPOSITED)
            .size(self.size)
            .position(self.position)
            .parent(Some(parent))
            .build()?;

        out.hook_tabs();

        if self.font.is_some() {
            out.set_font(self.font);
        }

        Ok(())
    }
}


/**
    A subwindow in a TabContainer widget
*/
#[derive(Default, Debug)]
pub struct Tab {
    pub handle: ControlHandle
}

impl Tab {

    pub fn builder<'a>() -> TabBuilder<'a> {
        TabBuilder {
            text: "Tab",
            parent: None
        }
    }

    /// Set the title of the tab
    pub fn set_text<'a>(&self, text: &'a str) {
        use winapi::um::commctrl::{TCM_SETITEMW, TCIF_TEXT, TCITEMW};
        use winapi::um::winuser::GWL_USERDATA;

        if self.handle.blank() { panic!(NOT_BOUND); }
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
    

    //
    // Other methods
    //

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("NWG_TAB")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        winapi::um::winuser::WS_CLIPCHILDREN
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        //use winapi::um::commctrl::{TCS_SINGLELINE};
        use winapi::um::winuser::{WS_CHILD};

        WS_CHILD
    }

    /// Set and initialize a tab as active
    unsafe fn init(current_handle: HWND, tab_view_handle: HWND, index: usize) {
        use winapi::um::winuser::GWL_USERDATA;

        // Save the index of the tab in the window data
        wh::set_window_long(current_handle, GWL_USERDATA, index);

        // Resize the tabs so that they match the tab view size and hide all children tabs
        let (w, h) = wh::get_window_size(tab_view_handle);
        let width = w - 11;
        let height = h - 30;

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

        if self.handle.blank() { panic!(NOT_BOUND); }
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

pub struct TabBuilder<'a> {
    text: &'a str,
    parent: Option<ControlHandle>
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

    pub fn build(self, out: &mut Tab) -> Result<(), SystemError> {
        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(SystemError::ControlWithoutParent)
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(out.flags())
            .text(self.text)
            .parent(Some(parent))
            .build()?;

        out.bind_container(self.text);

        Ok(())
    }
}


unsafe extern "system" fn resize_direct_children(handle: HWND, params: LPARAM) -> BOOL {
    let &(parent, w, h): &(HWND, u32, u32) = mem::transmute(params);
    if wh::get_window_parent(handle) == parent {
        wh::set_window_size(handle, w, h, false);
    }

    1
}

unsafe extern "system" fn count_children(_handle: HWND, params: LPARAM) -> BOOL {
    let count: &mut usize = ::std::mem::transmute(params);
    *count += 1;
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
