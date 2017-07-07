/*!
    Frame control definition
*/

use std::hash::Hash;
use std::any::TypeId;

use winapi::{HWND};

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;

/// System class identifier
const WINDOW_CLASS_NAME: &'static str = "NWG_BUILTIN_FRAME";

/**
    A template that creates a simple frame

    Control specific events: None

    Members:  
    • `position`: The start position of the label  
    • `size`: The start size of the label  
    • `visible`: If the label should be visible to the user  
    • `disabled`: If the user can or can't click on the label  
    • `align`: The text align of the label
    • `show_edge`: If the frame edge should be visible
    • `parent`: The label parent  
*/
#[derive(Clone)]
pub struct FrameT<ID: Hash+Clone> {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub show_edge: bool,
    pub parent: ID
}

impl<ID: Hash+Clone> ControlT<ID> for FrameT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<Frame>() }

    #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        unsafe{ 
            if let Err(e) = build_sysclass() { return Err(e); }
            match build_window(ui, &self) {
                Ok(h) => { 
                    Ok( Box::new(Frame{handle: h}) as Box<Control> ) 
                },
                Err(e) => Err(e)
            }
        }
    }
}

/**
    A standard label
*/
pub struct Frame {
    handle: HWND
}

impl Frame {
    pub fn get_text(&self) -> String { unsafe{ ::low::window_helper::get_window_text(self.handle) } }
    pub fn set_text<'a>(&self, text: &'a str) { unsafe{ ::low::window_helper::set_window_text(self.handle, text); } }
    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
    pub fn get_font<ID: Hash+Clone>(&self, ui: &Ui<ID>) -> Option<ID> { unsafe{ ::low::window_helper::get_window_font(self.handle, ui) } }
    pub fn set_font<ID: Hash+Clone>(&self, ui: &Ui<ID>, f: Option<&ID>) -> Result<(), Error> { unsafe{ ::low::window_helper::set_window_font(self.handle, ui, f) } }
    pub fn update(&self) { unsafe{ ::low::window_helper::update(self.handle); } }
}

impl Control for Frame {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Frame 
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

/*
    Private unsafe control methods
*/
use winapi::{UINT, WPARAM, LPARAM, LRESULT};

#[allow(unused_variables)]
unsafe extern "system" fn window_sysproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::{UINT, WM_CREATE, WM_PAINT, PAINTSTRUCT};
    use user32::{DefWindowProcW, DrawEdge, BeginPaint, EndPaint};
    use std::mem;

    let mut ps: PAINTSTRUCT = mem::uninitialized();
    const EDGE_RAISED: UINT = 0x0002 | 0x0004;
    const BF_RECT: UINT = 0x0001 | 0x0002 | 0x0004 | 0x0008 | 0x4000;

    let handled = match msg {
        WM_CREATE => true,
        WM_PAINT => {
            let hdc = BeginPaint(hwnd, &mut ps); 
            DrawEdge(hdc, &mut ps.rcPaint, EDGE_RAISED, BF_RECT);
            EndPaint(hwnd, &ps); 
            true
        },
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

    let params = SysclassParams { 
        class_name: WINDOW_CLASS_NAME,
        sysproc: Some(window_sysproc),
        background: None, style: Some(CS_DBLCLKS)
    };
    
    if let Err(e) = build_sysclass(params) {
        Err(Error::System(e))
    } else {
        Ok(())
    }
}

#[inline(always)]
unsafe fn build_window<ID: Hash+Clone>(ui: &Ui<ID>, t: &FrameT<ID>) -> Result<HWND, Error> {
    use low::window_helper::{WindowParams, build_window, handle_of_window};
    use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CLIPCHILDREN, WS_CHILD, WS_CLIPSIBLINGS,
     WS_EX_NOACTIVATE, WS_EX_COMPOSITED};

    let flags: DWORD = WS_CLIPCHILDREN | WS_CHILD | WS_CLIPSIBLINGS |
    if t.visible    { WS_VISIBLE }   else { 0 } |
    if t.disabled   { WS_DISABLED }  else { 0 } ;

     // Get the parent handle
    let parent = match handle_of_window(ui, &t.parent, "The parent of a frame must be a window-like control.") {
        Ok(h) => h,
        Err(e) => { return Err(e); }
    };

    let params = WindowParams {
        title: "",
        class_name: WINDOW_CLASS_NAME,
        position: t.position.clone(),
        size: t.size.clone(),
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
