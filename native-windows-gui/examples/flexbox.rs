/*!
    A very simple application that show your name in a message box.
    See `basic_d` for the derive version
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct FlexBoxApp {
    window: nwg::Window,
}

impl FlexBoxApp {

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

//
// ALL of this stuff is handled by native-windows-derive
//
mod flexbox_app_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct FlexBoxAppUi {
        inner: FlexBoxApp,
        default_handler: RefCell<Option<nwg::EventHandler>>
    }

    impl nwg::NativeUi<FlexBoxApp, Rc<FlexBoxAppUi>> for FlexBoxApp {
        fn build_ui(mut data: FlexBoxApp) -> Result<Rc<FlexBoxAppUi>, nwg::NwgError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((300, 115))
                .position((300, 300))
                .title("Basic example")
                .build(&mut data.window)?;

            // Wrap-up
            let ui = Rc::new(FlexBoxAppUi {
                inner: data,
                default_handler: Default::default(),
            });

            // Events
            let evt_ui = ui.clone();
            let handle_events = move |evt, _evt_data, handle| {
                match evt {
                    E::OnWindowClose => 
                        if &handle == &evt_ui.window {
                            FlexBoxApp::exit(&evt_ui.inner);
                        },
                    _ => {}
                }
            };

           *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

            return Ok(ui);
        }
    }

    impl FlexBoxAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        pub fn destroy(&self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for FlexBoxAppUi {
        type Target = FlexBoxApp;

        fn deref(&self) -> &FlexBoxApp {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let ui = FlexBoxApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
    ui.destroy();
}
