/*!
    A very simple application that show your name in a message box.
    See `basic_d` for the derive version
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct FlexBoxApp {
    window: nwg::Window,
    layout: nwg::FlexboxLayout,
    button1: nwg::Button,
    button2: nwg::Button,
    button3: nwg::Button
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
                .size((500, 300))
                .position((300, 300))
                .title("Flexbox example")
                .build(&mut data.window)?;

            nwg::Button::builder()
                .text("Btn 1")
                .parent(&data.window)
                .build(&mut data.button1)?;

            nwg::Button::builder()
                .text("Btn 2")
                .parent(&data.window)
                .build(&mut data.button2)?;

            nwg::Button::builder()
                .text("Btn 3")
                .parent(&data.window)
                .build(&mut data.button3)?;

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


            // Layout
            use nwg::stretch::{geometry::{Size, Rect}, style::{Dimension, FlexDirection}};
            pub const Fifty_PC: Dimension = Dimension::Percent(0.5);

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Row)
                .child(&ui.button1)
                    .child_size(Size { width: Fifty_PC, height: Dimension::Auto })
                .child(&ui.button2)
                    .child_size(Size { width: Dimension::Percent(0.25), height: Fifty_PC })
                .child(&ui.button3)
                    .child_size(Size { width: Dimension::Percent(0.25), height: Dimension::Auto })
                .build(&ui.layout);

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
