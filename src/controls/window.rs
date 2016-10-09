/*!
    A blank custom control.
*/

use std::hash::Hash;
use std::mem;
use std::ptr;

use controls::ControlTemplate;
use controls::base::{WindowBase, create_base, set_window_text, get_window_text, show_message,
  get_window_pos, set_window_pos, get_window_size, set_window_size, get_window_parent,
  set_window_parent, get_window_enabled, set_window_enabled, get_window_visibility,
  set_window_visibility, get_window_display, set_window_display, get_window_children,
  get_window_descendant, get_control_type, close_window, to_utf16, set_handle_data_off,
  get_handle_data_off};
use actions::{Action, ActionReturn};
use events::Event;
use constants::ControlType;

use winapi::{HWND, HINSTANCE, WNDCLASSEXW, UINT, CS_HREDRAW, CS_VREDRAW, IDC_ARROW,
  COLOR_WINDOW, WPARAM, LPARAM, LRESULT, WM_CLOSE, WM_CREATE};

use user32::{RegisterClassExW, LoadCursorW, PostQuitMessage, DefWindowProcW};
use kernel32::{GetLastError, GetModuleHandleW};

const CLASS_NAME: &'static str = "RustWindow";

/**
    Configuration properties to create a window

    * caption: Window title (in the upper bar)
    * size: Window size (width, height) in pixels
    * position: Starting position (x, y) of the window 
    * visible: If the window should be visible from the start
    * resizable: If the window should be resizable by the user
*/
pub struct Window {
    pub caption: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub visible: bool,
    pub resizable: bool,
    pub exit_on_close: bool
}

struct PrivateWindowData {
    pub exit_on_close: bool
}

impl<ID: Eq+Clone+Hash > ControlTemplate<ID> for Window {

    fn create(&self, ui: &mut ::Ui<ID>, id: ID) -> Result<HWND, ()> { 
        if unsafe { !register_custom_class() } {
            return Err(());
        }

        let base = WindowBase::<ID> {
            text: self.caption.clone(),
            size: self.size.clone(),
            position: self.position.clone(),
            visible: self.visible,
            resizable: self.resizable,
            extra_style: 0,
            class: CLASS_NAME.to_string(),
            parent: None
        };

        if let Ok(handle) = unsafe{ create_base::<ID>(ui, base) } {
            unsafe{ set_handle_data_off(handle, PrivateWindowData{exit_on_close: self.exit_on_close}, 0); }
            Ok(handle)
        } else {
            Err(())
        } 
    }

    fn supported_events(&self) -> Vec<Event> {
        vec![Event::MouseUp, Event::MouseDown, Event::Focus, Event::Removed]
    }

    fn evaluator(&self) -> ::ActionEvaluator<ID> {
        Box::new( |ui, id, handle, action| {
            match action {
                Action::Message(p) => show_message(handle, *p),
                Action::GetText => get_window_text(handle),
                Action::SetText(t) => set_window_text(handle, *t),
                Action::GetPosition => get_window_pos(handle, false),
                Action::SetPosition(x, y) => set_window_pos(handle, x, y),
                Action::GetSize => get_window_size(handle),
                Action::SetSize(w, h) => set_window_size(handle, w, h),
                Action::GetParent => get_window_parent(handle),
                Action::SetParent(p) => set_window_parent(ui, handle, p, false),
                Action::GetChildren => get_window_children(handle),
                Action::GetDescendants => get_window_descendant(handle),
                Action::GetEnabled => get_window_enabled(handle),
                Action::SetEnabled(e) => set_window_enabled(handle, e),
                Action::GetVisibility => get_window_visibility(handle),
                Action::SetVisibility(v) => set_window_visibility(handle, v),
                Action::GetControlType => get_control_type(handle),

                Action::GetWindowDisplay => get_window_display(handle),
                Action::SetWindowDisplay(d) => set_window_display(handle, d),
                
                Action::Close => close_window(handle),

                _ => ActionReturn::NotSupported
            }            
        })
    }

    fn control_type(&self) -> ControlType {
        ControlType::Window
    }

}


/**
    Register a new window class. Return true if the class already exists 
    or the creation was successful and false if it failed.
*/
unsafe fn register_custom_class() -> bool {
    let name = to_utf16(CLASS_NAME.to_string());
    let hmod = GetModuleHandleW(ptr::null());
    let class =
        WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc), 
            cbClsExtra: 0,
            cbWndExtra: mem::size_of::<usize>() as i32,
            hInstance: hmod as HINSTANCE,
            hIcon: ptr::null_mut(),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            hbrBackground: mem::transmute(COLOR_WINDOW as i64),
            lpszMenuName: ptr::null(),
            lpszClassName: name.as_ptr(),
            hIconSm: ptr::null_mut()
        };


    let class_token = RegisterClassExW(&class);
    if class_token == 0 && GetLastError() != 1410 {
        // If the class registration failed and the reason is not that
        // the class already exists (1410), return false.
        false
    } else {
        true
    }
}

/**
    Custom window procedure for Window types
*/
unsafe extern "system" fn wndproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => 0,
        WM_CLOSE => {
            if let Some(d) = get_handle_data_off::<PrivateWindowData>(hwnd, 0) {
                if d.exit_on_close { PostQuitMessage(0); }
            }
            
            0
        },
        _ =>  DefWindowProcW(hwnd, msg, w, l)
    }
}