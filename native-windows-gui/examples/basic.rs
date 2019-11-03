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
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct BasicAppUi {
        inner: BasicApp
    }

    impl nwg::NativeUi<BasicApp, BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<Rc<BasicAppUi>, nwg::SystemError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((300, 115))
                .position((300, 300))
                .title("Basic example")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .size((280, 25))
                .position((10, 10))
                .text("Heisenberg")
                .parent(&data.window)
                .build(&mut data.name_edit)?;

            nwg::Button::builder()
                .size((280, 60))
                .position((10, 40))
                .text("Say my name")
                .parent(&data.window)
                .build(&mut data.hello_button)?;

            // Wrap-up
            let ui = Rc::new(BasicAppUi { inner: data });

            // Events
            let window_handles = [&ui.window.handle];
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
    nwg::init_common_controls().expect("Failed to init common controls");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
