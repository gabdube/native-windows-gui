/**
    Simple example on how to create a custom window with NWG. This demo creates a window with two scollbars

    In NWG, a custom control is defined by two traits: the template (ControlT) and the control (Control)
    The template is evaluated by the UI and returns an instance of the control. The UI then manages the
    control until it is destroyed (aka unpacked).

    Right now, creating custom control is unsafe. Safe and simpler way to create custom may be added in future version of NWG.

    For simplicity, the custom control source is in the same source file.
    It is a better idea to have a single extern module for every custom control.
*/
extern crate user32;
extern crate winapi;
#[macro_use] extern crate native_windows_gui as nwg;

use std::any::TypeId;
use std::hash::Hash;

use nwg::custom::{Control, ControlT, AnyHandle, SysclassParams, build_sysclass, WindowParams, Event, build_window, hwnd_handle, event_unpack_no_args};
use nwg::{Error, Ui, simple_message, fatal_message, dispatch_events};
use nwg::events as nwge;

use winapi::{HWND, UINT, WPARAM, LPARAM, LRESULT};

// A custom event that tracks the WM_MOUSEWHEEL message
#[allow(non_upper_case_globals)]
const MouseWheel: Event = Event::Single(winapi::WM_MOUSEWHEEL, &event_unpack_no_args, &hwnd_handle);

// The control template. Aka the configuration object that is sent to an UI.
pub struct MyCustomWindowT;

// The actual control. The object that is saved and managed in NWG.
// A reference to this object is returned when nwg_get is used
pub struct MyCustomWindow {
    handle: HWND
}

impl<ID: Hash+Clone> ControlT<ID> for MyCustomWindowT {
    fn type_id(&self) -> TypeId { 
        // This method must return the TypeID of the associated control
        // Used internally by NWG
        
        TypeId::of::<MyCustomWindow>() 
    }

    fn events(&self) -> Vec<nwge::Event> {
        // This method must return an vec of events type that NWG will listen to for each instance of the control

        vec![nwge::Closed, MouseWheel]
    }

    #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use winapi::{WS_HSCROLL, WS_VISIBLE, WS_VSCROLL, WS_OVERLAPPEDWINDOW};

        // This method must create the low level control and return it as a Box<Control>
        // NWG offers both build_sysclass and build_window in order to facilitate the low level window creation
        // If the method returns an error, Ui::commit will return it.
        
        unsafe{
            let cls_params = SysclassParams { 
                class_name: "MyCustomWindowClass",      // The unique identifier of the window class
                sysproc: Some(custom_window_sysproc),   // The low level window procedure of the window
                background: None,                       // The background color of the window (use the default system color)
                style: None                             // The style class (use defaults)
            };

            let style =  WS_HSCROLL | WS_VISIBLE | WS_VSCROLL | WS_OVERLAPPEDWINDOW;
            let params = WindowParams {
                title: "My custom window",              // The window title
                class_name: "MyCustomWindowClass",      // The window class
                position: (200, 200),                   // The window starting position
                size: (500, 500),                       // The window starting size
                flags: style,                           // The window style
                ex_flags: None,                         // The window extended flags
                parent: ::std::ptr::null_mut()          // The parent (don't forget to include WS_CHILD in flags if there is one)
            };

            // Try to create the custom window class, return an error if it fails
            if let Err(e) = build_sysclass(cls_params) {
                return Err(Error::System(e));
            }

            // Try to create the window, return an error if it fails
            match build_window(params) {
                Ok(h) => {
                    Ok(Box::new(MyCustomWindow{handle: h}) as Box<Control>)
                },
                Err(e) => Err(Error::System(e))
            }
        } // unsafe
    }

}

impl Control for MyCustomWindow {
    fn handle(&self) -> AnyHandle {
        // Must return the handle of the control
        // Use internally by NWG, but it can be used to retrieve the low level handle as a user

        AnyHandle::HWND(self.handle)
    }

    fn free(&mut self) {
        // Free any resources allocated in the template build function
        // This is called by the UI when the control is removed

        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }
}

// The custom window proc
#[allow(unused_variables)]
unsafe extern "system" fn custom_window_sysproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::{WM_CREATE, WM_CLOSE};
    use user32::{DefWindowProcW, PostQuitMessage, ShowWindow};

    let handled = match msg {
        WM_CREATE => true,
        WM_CLOSE => {
            ShowWindow(hwnd, 0);
            PostQuitMessage(0);
            true
        }
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}

fn main() {
    let app: Ui<&'static str>;

    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    // Add our custom window to the UI
    app.pack_control(&"MyCustomWindow", MyCustomWindowT);

    // Add a chidlren
    app.pack_control(&"AButton", nwg_button!(parent="MyCustomWindow"; text="Test"; position=(10,10); size=(480, 480)) );

    // Bind an event
    app.bind(&"MyCustomWindow", &"ExitNWG", nwge::Closed, |_,_,_,_| {
        simple_message("Hello", "Goodbye!");
        nwg::exit();
    });

    // Bind the custom event
    app.bind(&"MyCustomWindow", &"Wheel", MouseWheel, |_,control,_,_| {
        println!("Mouse wheel event catched on control {:?}", control);
    });

    dispatch_events();
}
