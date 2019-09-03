#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    name_edit: nwg::TextInput,
    hello_button: nwg::Button
}

impl BasicApp {

    fn say_hello(&self, _event: nwg::Event) {
        nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    fn say_goodbye(&self, _event: nwg::Event) {
        nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use nwg;
    use super::BasicApp;
    use std::rc::Rc;
    use std::ops::Deref;

    #[allow(dead_code)]
    pub struct BasicAppUi {
        inner: BasicApp
    }

    impl nwg::NativeUi<BasicApp, BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<Rc<BasicAppUi>, nwg::SystemError> {
            use nwg::Event as E;
            
            // Controls
            let window = nwg::ControlBase::build_hwnd()
                .class_name(data.window.class_name())
                .forced_flags(data.window.forced_flags())
                .flags(Some(((nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE ).bits(), 0)).unwrap_or(data.window.flags()))
                .size((300, 115))
                .position((300, 300))
                .text("Basic example")
                .build()?;
            data.window.handle = window.handle.clone();

            let name_edit =  nwg::ControlBase::build_hwnd()
                .class_name(data.name_edit.class_name())
                .forced_flags(data.name_edit.forced_flags())
                .flags(data.name_edit.flags())
                .size((280, 25))
                .position((10, 10))
                .text("Heisenberg")
                .parent(Some(&window))
                .build()?;
            data.name_edit.handle = name_edit.handle.clone();

            let hello_button = nwg::ControlBase::build_hwnd()
                .class_name(data.hello_button.class_name())
                .forced_flags(data.hello_button.forced_flags())
                .flags(data.hello_button.flags())
                .size((280, 60))
                .position((10, 40))
                .text("Say my name")
                .parent(Some(&window))
                .build()?;
            data.hello_button.handle = hello_button.handle.clone();

            // Wrap-up
            let ui = Rc::new(BasicAppUi { inner: data });

            // Events
            let window_handles = [&window.handle];
            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, handle| {
                    match evt {
                        E::OnButtonClick => {
                            if handle == evt_ui.hello_button.handle {
                                BasicApp::say_hello(&evt_ui.inner, evt);
                            }
                        },
                        E::OnWindowClose => {
                            if handle == evt_ui.window.handle {
                                BasicApp::say_goodbye(&evt_ui.inner, evt);
                            }
                        },
                        _ => {}
                    }
                };

                nwg::bind_event_handler(handle, handle_events);
            }
            
            return Ok(ui);
        }
    }


    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }

}



fn main() {
    nwg::enable_visual_styles();

    let app = BasicApp::build_ui(Default::default());

    app.expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
