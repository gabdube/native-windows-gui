/*!
    A very simple application that show your name in a message box.
    See `basic_d` for the derive version
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    name_edit: nwg::TextInput,
    hello_button: nwg::Button
}

impl BasicApp {

    fn say_hello(&self) {
        nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    fn say_goodbye(&self) {
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

    impl nwg::NativeUi<BasicApp, Rc<BasicAppUi>> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<Rc<BasicAppUi>, nwg::NwgError> {
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
            let evt_ui = ui.clone();
            let handle_events = move |evt, _evt_data, handle| {
                match evt {
                    E::OnButtonClick => 
                        if &handle == &evt_ui.hello_button {
                            BasicApp::say_hello(&evt_ui.inner);
                        },
                    E::OnWindowClose => 
                        if &handle == &evt_ui.window {
                            BasicApp::say_goodbye(&evt_ui.inner);
                        },
                    _ => {}
                }
            };

            nwg::full_bind_event_handler(&ui.window.handle, handle_events);

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
    nwg::init().expect("Failed to init Native Windows GUI");

    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
