use crate::*;
use std::cell::RefCell;

mod control_test;
use control_test::*;

mod thread_test;
use thread_test::*;

mod freeing_test;
use freeing_test::*;

mod other;


#[derive(Default)]
pub struct TestControlPanel {
    window: Window,

    layout: FlexboxLayout,

    controls_test_button: Button,
    thread_test_button: Button,
    free_test_button: Button,

    controls_tests: ControlsTest,
    thread_tests: ThreadTest,
    freeing_tests: FreeingTest,
}

mod test_control_panel_ui {
    use super::*;
    use crate::{NativeUi, NwgError};
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct TestControlPanelUi {
        inner: Rc<TestControlPanel>,
        default_handler: RefCell<Vec<EventHandler>>
    }

    impl NativeUi<TestControlPanelUi> for TestControlPanel {
        fn build_ui(mut data: TestControlPanel) -> Result<TestControlPanelUi, NwgError> {
            use crate::Event as E;

            // Controls
            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((200, 200))
                .position((1100, 300))
                .title("Tests Control Panel")
                .build(&mut data.window)?;

            Button::builder()
                .text("Control tests")
                .focus(true)
                .parent(&data.window)
                .build(&mut data.controls_test_button)?;

            Button::builder()
                .text("Thread tests")
                .parent(&data.window)
                .build(&mut data.thread_test_button)?;

            Button::builder()
                .text("Freeing")
                .parent(&data.window)
                .build(&mut data.free_test_button)?;

            // Partials
            ControlsTest::build_partial(&mut data.controls_tests, Some(&data.window))?;
            ThreadTest::build_partial(&mut data.thread_tests, Some(&data.window))?;
            FreeingTest::build_partial(&mut data.freeing_tests, Some(&data.window))?;

            // Wrap-up
            let ui = TestControlPanelUi { 
                inner: Rc::new(data),
                default_handler: Default::default()
            };

            // Events
            let mut window_handles = vec![&ui.window.handle];
            window_handles.append(&mut ui.controls_tests.handles());
            window_handles.append(&mut ui.thread_tests.handles());
            window_handles.append(&mut ui.freeing_tests.handles());

            for handle in window_handles.iter() {
                let evt_ui = Rc::downgrade(&ui.inner);
                let handle_events = move |evt, evt_data, handle| {
                    if let Some(evt_ui) = evt_ui.upgrade() {
                        evt_ui.controls_tests.process_event(evt, &evt_data, handle);
                        evt_ui.thread_tests.process_event(evt, &evt_data, handle);
                        evt_ui.freeing_tests.process_event(evt, &evt_data, handle);

                        match evt {
                            E::OnButtonClick =>
                                if &handle == &evt_ui.controls_test_button {
                                    show_control_test(&evt_ui);
                                } else if &handle == &evt_ui.thread_test_button {
                                    show_thread_test(&evt_ui);
                                } else if &handle == &evt_ui.free_test_button {
                                    show_freeing_test(&evt_ui);
                                },
                            E::OnInit => 
                                if handle == evt_ui.window.handle {
                                    show(&evt_ui);
                                },
                            E::OnWindowClose => 
                                if handle == evt_ui.window.handle {
                                    close();
                                },
                            _ => {}
                        }
                    }
                };

                let handler = full_bind_event_handler(handle, handle_events);

                ui.default_handler.borrow_mut().push(handler);
            }

            // Layouts
            FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(stretch::style::FlexDirection::Column)
                .child(&ui.controls_test_button)
                .child(&ui.thread_test_button)
                .child(&ui.free_test_button)
                .build(&ui.layout)?;

            Ok(ui)
        }
    }

    impl Drop for TestControlPanelUi {

        fn drop(&mut self) {
            self.freeing_tests.destroy();

            let mut handlers = self.default_handler.borrow_mut();
            let handlers_count = handlers.len();
            for handler in handlers.drain(0..handlers_count) {
                unbind_event_handler(&handler);
            }
        }

    }

    impl Deref for TestControlPanelUi {
        type Target = TestControlPanel;

        fn deref(&self) -> &TestControlPanel {
            &self.inner
        }
    }

}

fn show_control_test(app: &TestControlPanel) {
    app.controls_tests.window.set_visible(true);
    app.controls_tests.panel.set_visible(true);
    app.controls_tests.window.set_focus();
}

fn show_thread_test(app: &TestControlPanel) {
    app.thread_tests.window.set_visible(true);
    app.thread_tests.window.set_focus();
}

fn show_freeing_test(app: &TestControlPanel) {
    app.freeing_tests.window.set_visible(true);
    app.freeing_tests.window.set_focus();
}

fn show(app: &TestControlPanel) {
    let text = "Hello World from Native windows GUI!";
    Clipboard::set_data_text(&app.window, text);
    assert!(Some(text) == Clipboard::data_text(&app.window).as_ref().map(|s| s as &str));

    app.window.set_visible(true);
}

fn close() {
    stop_thread_dispatch();
}

#[test]
fn everything() {
    #[cfg(feature = "high-dpi")]
    {
        unsafe {
            crate::win32::high_dpi::set_dpi_awareness();
        }
    }

    init().expect("Failed to init Native Windows GUI");
    Font::set_global_family("Segoe UI").expect("Failed to set default font");
    
    let app = TestControlPanel::build_ui(Default::default()).expect("Failed to build UI");

    app.window.set_focus();

    dispatch_thread_events();
}
