use crate::*;
use std::cell::RefCell;

#[derive(Default)]
pub struct OtherTests {
    window: Window,
    layout: FlexboxLayout,
    test_combobox: Button,
}

mod other_tests_ui {
    use super::*;
    use crate::{NativeUi, NwgError};
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct OtherTestsUi {
        inner: OtherTests,
        default_handler: RefCell<Option<EventHandler>>
    }

    impl NativeUi<OtherTests, Rc<OtherTestsUi>> for OtherTests {
        fn build_ui(mut data: OtherTests) -> Result<Rc<OtherTestsUi>, NwgError> {
            use crate::Event as E;

            // Controls
            Window::builder()
                .size((340, 150))
                .position((500, 300))
                .title("Other Tests")
                .build(&mut data.window)?;

            Button::builder()
                .text("Combobox")
                .focus(true)
                .parent(&data.window)
                .build(&mut data.test_combobox)?;
            
            let ui = Rc::new(OtherTestsUi { inner: data, default_handler: Default::default() });

            // Events
            let evt_ui = ui.clone();
            let handle_events = move |evt, _evt_data, handle| {
                match evt {
                    E::OnWindowClose => 
                        if &handle == &evt_ui.window {
                            stop_thread_dispatch();
                        },
                    _ => {}
                }
            };

           *ui.default_handler.borrow_mut() = Some(full_bind_event_handler(&ui.window.handle, handle_events));

            FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(stretch::style::FlexDirection::Column)
                .child(&ui.test_combobox)
                .build(&ui.layout);
            
            Ok(ui)
        }
    }

    impl OtherTestsUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        pub fn destroy(&self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for OtherTestsUi {
        type Target = OtherTests;

        fn deref(&self) -> &OtherTests {
            &self.inner
        }
    }
}



#[test]
fn heap_corruption() {
    init().expect("Failed to init Native Windows GUI");
    
    let app = OtherTests::build_ui(Default::default()).expect("Failed to build UI");
    dispatch_thread_events();
    app.destroy();
}