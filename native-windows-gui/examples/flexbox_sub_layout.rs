/*!
    A very simple application that show how to use a flexbox layout.

    Requires the following features: `cargo run --example flexbox --features "flexbox"`
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct FlexBoxApp {
    window: nwg::Window,
    layout: nwg::FlexboxLayout,
    button1: nwg::Button,
    layout2: nwg::FlexboxLayout,
    button2: nwg::Button,
    button3: nwg::Button,
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
        inner: Rc<FlexBoxApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>
    }

    impl nwg::NativeUi<FlexBoxAppUi> for FlexBoxApp {
        fn build_ui(mut data: FlexBoxApp) -> Result<FlexBoxAppUi, nwg::NwgError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .size((500, 500))
                .position((300, 300))
                .title("Flexbox example")
                .build(&mut data.window)?;

            nwg::Button::builder()
                .text("Btn 1")
                .parent(&data.window)
                .focus(true)
                .build(&mut data.button1)?;

            nwg::Button::builder()
                .text("Btn 2")
                .parent(&data.window)
                .focus(true)
                .build(&mut data.button2)?;

            nwg::Button::builder()
                .text("Btn 3")
                .parent(&data.window)
                .focus(true)
                .build(&mut data.button3)?;

            // Wrap-up
            let ui =FlexBoxAppUi {
                inner:  Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnWindowClose => 
                            if &handle == &evt_ui.window {
                                FlexBoxApp::exit(&evt_ui);
                            },
                        _ => {}
                    }
                }
            };

           *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));


            // Layout
            use nwg::stretch::{geometry::Size, style::{Dimension as D, FlexDirection}};

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Column)
                .child(&ui.button2)
                    .child_size(Size { width: D::Auto, height: D::Points(200.0) })
                .child(&ui.button3)
                    .child_flex_grow(2.0)
                    .child_size(Size { width: D::Auto, height: D::Auto })
                .build_partial(&ui.layout2)?;
                
            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Row)
                .child(&ui.button1)
                    .child_flex_grow(2.0)
                    .child_size(Size { width: D::Auto, height: D::Auto })
                .child_layout(&ui.layout2)
                    .child_size(Size { width: D::Points(300.0), height: D::Auto })
                .build(&ui.layout)?;

            
            return Ok(ui);
        }
    }

    impl Drop for FlexBoxAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
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
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _ui = FlexBoxApp::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
