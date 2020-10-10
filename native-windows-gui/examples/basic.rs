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
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    fn say_goodbye(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
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
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct BasicAppUi {
        inner: Rc<BasicApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>
    }

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((300, 135))
                .position((300, 300))
                .title("Basic example")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .size((280, 35))
                .position((10, 10))
                .text("Heisenberg")
                .parent(&data.window)
                .focus(true)
                .build(&mut data.name_edit)?;

            nwg::Button::builder()
                .size((280, 70))
                .position((10, 50))
                .text("Say my name")
                .parent(&data.window)
                .build(&mut data.hello_button)?;

            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => 
                            if &handle == &ui.hello_button {
                                BasicApp::say_hello(&ui);
                            },
                        E::OnWindowClose => 
                            if &handle == &ui.window {
                                BasicApp::say_goodbye(&ui);
                            },
                        _ => {}
                    }
                }
            };

           *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

            return Ok(ui);
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
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
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
