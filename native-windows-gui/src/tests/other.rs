use crate::*;
use std::cell::RefCell;

#[derive(Default)]
pub struct OtherTests {
    window: Window,
    layout: FlexboxLayout,
    test: Button,
}

mod other_tests_ui {
    use super::*;
    use crate::{NativeUi, NwgError};
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct OtherTestsUi {
        inner: Rc<OtherTests>,
        default_handler: RefCell<Option<EventHandler>>
    }

    impl NativeUi<OtherTestsUi> for OtherTests {
        fn build_ui(mut data: OtherTests) -> Result<OtherTestsUi, NwgError> {
            use crate::Event as E;

            // Controls
            Window::builder()
                .size((340, 150))
                .position((500, 300))
                .title("Other Tests")
                .build(&mut data.window)?;

            Button::builder()
                .text("Test")
                .focus(true)
                .parent(&data.window)
                .build(&mut data.test)?;
            
            let ui = OtherTestsUi { inner: Rc::new(data), default_handler: Default::default() };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => 
                            if &handle == &evt_ui.test {
                                test_stuff(&evt_ui);
                            },
                        E::OnWindowClose => 
                            if &handle == &evt_ui.window {
                                stop_thread_dispatch();
                            },
                        _ => {}
                    }
                }
            };

           *ui.default_handler.borrow_mut() = Some(full_bind_event_handler(&ui.window.handle, handle_events));

            FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(stretch::style::FlexDirection::Column)
                .child(&ui.test)
                .build(&ui.layout)?;
            
            Ok(ui)
        }
    }

    impl Drop for OtherTestsUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
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

fn test_stuff(_t: &OtherTests) {

}


/// Just some scaffolding if I ever want to add new specific tests
#[test]
#[allow(unused)]
fn other_tests() {
    init().expect("Failed to init Native Windows GUI");
    let _app = OtherTests::build_ui(Default::default()).expect("Failed to build UI");
    //dispatch_thread_events();
}