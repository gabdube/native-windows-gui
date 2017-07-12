/*!
    A tabs control
*/

// Developer note: The tab switching is implemented in low::events

use std::hash::Hash;
use std::any::TypeId;
use std::mem;

use winapi::{HWND, HFONT, UINT, UINT_PTR, DWORD_PTR, LPARAM, WPARAM, LRESULT, BOOL};
use user32::SendMessageW;

use ui::Ui;
use error::{Error, SystemError};
use controls::{Control, ControlT, ControlType, AnyHandle};

/// System class identifier
const TAB_CLASS_NAME: &'static str = "NWG_BUILTIN_TAB";

/// Tabview subclass identifier
const TABVIEWS_SUBCLASS_ID: UINT_PTR = 4359;

//
// TabView
//

/**
    A template that creates a tabview

    tabview specific events:  

    Members:  
        • position: The initial position of the control  
        • size: The inital size of the control  
        • visible: If the control should be visible  
        • disable: If the control should be disabled  
        • parent: The control parent          
*/
#[derive(Clone)]
pub struct TabViewT<ID: Hash+Clone> {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub parent: ID,
    pub font: Option<ID>
}

impl<ID: Hash+Clone> ControlT<ID> for TabViewT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<TabView>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{handle_of_window, build_window, handle_of_font, set_window_font_raw, WindowParams};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD};

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a tabsview must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a label must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let flags: DWORD = WS_CHILD | 
        if self.visible    { WS_VISIBLE }   else { 0 } |
        if self.disabled   { WS_DISABLED }  else { 0 };

        let params = WindowParams {
            title: "",
            class_name: "SysTabControl32",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                unsafe{ 
                    set_window_font_raw(h, font_handle, true);
                    hook_view(h);
                }
                Ok( Box::new(TabView{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}


pub struct TabView {
    handle: HWND
}

/**
    A TabView control
*/
impl TabView {

    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
    pub fn update(&self) { unsafe{ ::low::window_helper::update(self.handle); } }
    pub fn focus(&self) { unsafe{ ::user32::SetFocus(self.handle); } }

}


impl Control for TabView {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::TabsView 
    }

    fn children(&self) -> Vec<AnyHandle> {
        use low::window_helper::list_window_children;
        unsafe{ list_window_children(self.handle) }
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ 
            unhook_view(self.handle);
            DestroyWindow(self.handle) 
        };
    }
}


//
// Tab
//

/**
    A templates that creates a tab in a tabview


*/
pub struct TabT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub text: S,
    pub parent: ID
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for TabT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<Tab>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use user32::DestroyWindow;

        // Check if the parent handle is valid
        let view_handle = ui.handle_of(&self.parent);
        let view_handle = match view_handle {
            Ok(AnyHandle::HWND(h)) => {
                match ui.type_of_control(&self.parent) {
                    Ok(ControlType::TabsView) => { h },
                    Ok(t) => { return Err(Error::BadParent(format!("TabView parent required got \"{:?}\" control", t))); }
                    Err(e) => { return Err(e); }
                }
            },
            Ok(ref h) => { return Err(Error::BadParent(format!("TabViewparent required got \"{}\" control", h.human_name()))); },
            Err(ref e) => { return Err(e.clone()); }
        };

        // Build the tab window
        let handle = unsafe {
            if let Err(e) = build_sysclass() { return Err(e); }
            match build_window(view_handle, &self) {
                Ok(h) => { h },
                Err(e) => { return Err(e) }
            }
        };

        let text = self.text.clone().into();
        match unsafe{ insert_tab(view_handle, handle, text) } {
            Ok(_) => { 
                let tab = Tab{ view: view_handle, handle: handle };
                Ok( Box::new(tab) )
            },
            Err(e) => {
                unsafe{ DestroyWindow(handle); }
                Err(Error::System(e))
            }
        }
    }
}

/**
    A Tab in a TabView
*/
pub struct Tab {
    view: HWND,
    handle: HWND
}

impl Tab {

    pub fn view<ID: Hash+Clone>(&self, ui: &Ui<ID>) -> Result<ID, Error> {
        match ui.id_from_handle(&AnyHandle::HWND(self.view)) {
            Ok(id) => Ok(id),
            Err(e) => Err(e)
        }
    }

}

impl Control for Tab {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Tab 
    }

    fn children(&self) -> Vec<AnyHandle> {
        use low::window_helper::list_window_children;
        unsafe{ list_window_children(self.handle) }
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }
}

//
// Tab Custom Window
//

#[allow(unused_variables)]
unsafe extern "system" fn tab_sysproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::{WM_CREATE};
    use user32::{DefWindowProcW};

    let handled = match msg {
        WM_CREATE => true,
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}

#[inline(always)]
unsafe fn build_sysclass() -> Result<(), Error> {
    use low::window_helper::{SysclassParams, build_sysclass};
    use winapi::CS_DBLCLKS;
    use std::ptr;

    let params = SysclassParams { 
        class_name: TAB_CLASS_NAME,
        sysproc: Some(tab_sysproc),
        background: Some(ptr::null_mut()), style: Some(CS_DBLCLKS)
    };
    
    if let Err(e) = build_sysclass(params) {
        Err(Error::System(e))
    } else {
        Ok(())
    }
}

#[inline(always)]
unsafe fn build_window<S: Clone+Into<String>, ID: Hash+Clone>(parent: HWND, t: &TabT<S, ID>) -> Result<HWND, Error> {
    use low::window_helper::{WindowParams, build_window};
    use winapi::{DWORD, WS_VISIBLE, WS_CLIPCHILDREN, WS_CHILD, WS_CLIPSIBLINGS,
     WS_EX_NOACTIVATE, WS_EX_COMPOSITED};

    let flags: DWORD = WS_CLIPCHILDREN | WS_CHILD | WS_CLIPSIBLINGS;

    let params = WindowParams {
        title: "",
        class_name: TAB_CLASS_NAME,
        position: (5, 30),
        size: (100, 100),
        flags: flags,
        ex_flags: Some(WS_EX_NOACTIVATE | WS_EX_COMPOSITED),
        parent: parent
    };

    match build_window(params) {
        Ok(h) => {
            Ok(h)
        },
        Err(e) => Err(Error::System(e))
    }
}


///
/// Tabview subclass hook
///

unsafe extern "system" fn resize_direct_children(handle: HWND, params: LPARAM) -> BOOL {
    use std::mem;
    use user32::GetParent;
    use low::window_helper::set_window_size;

    let &(parent, w, h): &(HWND, u32, u32) = mem::transmute(params);
    if GetParent(handle) == parent {
        set_window_size(handle, w-10, h-35, false);
    }

    1
}


#[allow(unused_variables)]
unsafe extern "system" fn process_events(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT {
    use comctl32::DefSubclassProc;
    use user32::EnumChildWindows;
    use winapi::{WM_SIZE, TCN_SELCHANGE};
    use std::mem;
    use low::window_helper::get_window_size;

    if msg == WM_SIZE {
        let (w, h) = get_window_size(hwnd);
        let data: (HWND, u32, u32) = (hwnd, w, h);
        EnumChildWindows(hwnd, Some(resize_direct_children), mem::transmute(&data));
    }

    DefSubclassProc(hwnd, msg, w, l)
}

unsafe fn hook_view(handle: HWND) {
    use comctl32::SetWindowSubclass;
    SetWindowSubclass(handle, Some(process_events), TABVIEWS_SUBCLASS_ID, 0);
}

unsafe fn unhook_view(handle: HWND) {
    use comctl32::RemoveWindowSubclass;
    RemoveWindowSubclass(handle, Some(process_events), TABVIEWS_SUBCLASS_ID);
}

//
// Other Private functions
//

unsafe fn insert_tab(view: HWND, child: HWND, text: String) -> Result<(), SystemError> {
    use winapi::{TCM_INSERTITEMW, TCM_GETITEMCOUNT, TCITEMW, TCIF_TEXT};
    use low::window_helper::set_window_visibility;
    use low::other_helper::to_utf16;

    let mut text = to_utf16(&text);

    let mut info = TCITEMW {
        mask: TCIF_TEXT,
        dwState: 0,
        dwStateMask: 0,
        pszText: text.as_mut_ptr(),
        cchTextMax: 0,
        iImage: -1,
        lParam: 0
    };

    let info_ptr: LPARAM = mem::transmute(&mut info);
    let count = SendMessageW(view, TCM_GETITEMCOUNT, 0, 0);

    if SendMessageW(view, TCM_INSERTITEMW, count as WPARAM, info_ptr) != -1 {
        set_window_visibility(child, count==0); // Set the first tab inserted as visible
        Ok(())
    } else {
        Err(SystemError::SystemMessageFailed("Could insert tab in tabview".to_owned()))
    }
}
