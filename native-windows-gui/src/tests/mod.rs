use crate::*;

mod control_test;
use control_test::*;

mod canvas_test;
use canvas_test::*;

mod thread_test;
use thread_test::*;

mod images_test;
use images_test::*;


#[derive(Default)]
pub struct TestControlPanel {
    window: Window,

    layout: BoxLayout,

    controls_test_button: Button,
    canvas_test_button: Button,
    thread_test_button: Button,
    images_test_button: Button,

    controls_tests: ControlsTest,
    canvas_tests: CanvasTest,
    thread_tests: ThreadTest,
    images_tests: ImagesTest,
}

mod test_control_panel_ui {
    use super::*;
    use crate::{NativeUi, NwgError};
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct TestControlPanelUi {
        inner: TestControlPanel
    }

    impl NativeUi<TestControlPanel, TestControlPanelUi> for TestControlPanel {
        fn build_ui(mut data: TestControlPanel) -> Result<Rc<TestControlPanelUi>, NwgError> {
            use crate::Event as E;

            // Controls
            Window::builder()
                .flags(WindowFlags::WINDOW | WindowFlags::VISIBLE)
                .size((200, 200))
                .position((1100, 300))
                .title("Tests Control Panel")
                .build(&mut data.window)?;

            Button::builder()
                .text("Control tests")
                .parent(&data.window)
                .build(&mut data.controls_test_button)?;

            Button::builder()
                .text("Canvas tests")
                .enabled(cfg!(feature = "canvas"))
                .parent(&data.window)
                .build(&mut data.canvas_test_button)?;

            Button::builder()
                .text("Thread tests")
                .parent(&data.window)
                .build(&mut data.thread_test_button)?;

            Button::builder()
                .text("Image buttons")
                .parent(&data.window)
                .build(&mut data.images_test_button)?;

            // Partials
            ControlsTest::build_partial(&mut data.controls_tests, Some(&data.window))?;
            CanvasTest::build_partial(&mut data.canvas_tests, Some(&data.window))?;
            ThreadTest::build_partial(&mut data.thread_tests, Some(&data.window))?;
            ImagesTest::build_partial(&mut data.images_tests, Some(&data.window))?;

            // Wrap-up
            let ui = Rc::new(TestControlPanelUi { inner: data });

            // Events
            let mut window_handles = vec![&ui.window.handle];
            window_handles.append(&mut ui.controls_tests.handles());
            window_handles.append(&mut ui.canvas_tests.handles());
            window_handles.append(&mut ui.thread_tests.handles());

            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, evt_data, handle| {

                    evt_ui.controls_tests.process_event(evt, &evt_data, handle);
                    evt_ui.canvas_tests.process_event(evt, &evt_data, handle);
                    evt_ui.thread_tests.process_event(evt, &evt_data, handle);
                    evt_ui.images_tests.process_event(evt, &evt_data, handle);

                    match evt {
                        E::OnButtonClick =>
                            if &handle == &evt_ui.controls_test_button {
                                show_control_test(&evt_ui.inner);
                            } else if &handle == &evt_ui.canvas_test_button {
                                show_canvas_test(&evt_ui.inner);
                            } else if &handle == &evt_ui.thread_test_button {
                                show_thread_test(&evt_ui.inner);
                            } else if &handle == &evt_ui.images_test_button {
                                show_images_test(&evt_ui.inner);
                            },
                        E::OnWindowClose => 
                            if handle == evt_ui.window.handle {
                                close(&evt_ui.inner);
                            },
                        _ => {}
                    }
                };

                full_bind_event_handler(handle, handle_events);
            }

            // Layouts
            BoxLayout::builder()
                .parent(&ui.window)
                .margin([5,5,5,5])
                .child(0, &ui.controls_test_button)
                .child(1, &ui.canvas_test_button)
                .child(2, &ui.thread_test_button)
                .child(3, &ui.images_test_button)
                .build(&ui.layout);

            Ok(ui)
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

fn show_canvas_test(app: &TestControlPanel) {
    app.canvas_tests.window.set_visible(true);
    app.canvas_tests.window.set_focus();
}

fn show_thread_test(app: &TestControlPanel) {
    app.thread_tests.window.set_visible(true);
    app.thread_tests.window.set_focus();
}

fn show_images_test(app: &TestControlPanel) {
    app.images_tests.window.set_visible(true);
    app.images_tests.window.set_focus();
}

fn close(_app: &TestControlPanel) {
    stop_thread_dispatch();
}

#[test]
fn everything() {
    enable_visual_styles();
    init_common_controls().expect("Failed to init controls");
    
    let app = TestControlPanel::build_ui(Default::default()).expect("Failed to build UI");

    app.window.set_focus();

    dispatch_thread_events();
}
