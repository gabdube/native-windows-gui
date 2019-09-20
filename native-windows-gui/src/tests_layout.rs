use crate::*;

#[derive(Default)]
pub struct TestLayoutApp {
    // HBoxLayout
    hbox_window: Window,
    button1: Button,
    button2: Button,
    button3: Button,
    
    // VBoxLayout
    vbox_window: Window,
    button4: Button,
    button5: Button,
    button6: Button,

    // GridLayout

    // FormLayout
}

#[allow(unused_imports)]
mod test_layout_app_ui {
    use super::*;
    use crate::{NativeUi, ControlBase, ControlHandle, SystemError, bind_event_handler, WindowFlags};
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct TestLayoutAppUi {
        inner: TestLayoutApp
    }

    impl NativeUi<TestLayoutApp, TestLayoutAppUi> for TestLayoutApp {
        fn build_ui(mut data: TestLayoutApp) -> Result<Rc<TestLayoutAppUi>, SystemError> {
            use crate::Event as E;

            let hbox_window = ControlBase::build_hwnd()
                .class_name(data.hbox_window.class_name())
                .forced_flags(data.hbox_window.forced_flags())
                .flags(Some(((WindowFlags::WINDOW | WindowFlags::VISIBLE | WindowFlags::RESIZABLE).bits(), 0)).unwrap_or(data.hbox_window.flags()))
                .size((400, 100))
                .position((300, 300))
                .text("HBOX test")
                .build()?;
            data.hbox_window.handle = hbox_window.handle.clone();
        
            let button1 = ControlBase::build_hwnd()
              .class_name(data.button1.class_name())
              .forced_flags(data.button1.forced_flags())
              .flags(data.button1.flags())
              .text("Button 1")
              .parent(Some(&hbox_window))
              .build()?;
            data.button1.handle = button1.handle.clone();

            let button2 = ControlBase::build_hwnd()
              .class_name(data.button2.class_name())
              .forced_flags(data.button2.forced_flags())
              .flags(data.button2.flags())
              .text("Button 2")
              .parent(Some(&hbox_window))
              .build()?;
            data.button2.handle = button2.handle.clone();

            let button3 = ControlBase::build_hwnd()
              .class_name(data.button3.class_name())
              .forced_flags(data.button3.forced_flags())
              .flags(data.button3.flags())
              .text("Button 3")
              .parent(Some(&hbox_window))
              .build()?;
            data.button3.handle = button3.handle.clone();

            let vbox_window = ControlBase::build_hwnd()
                .class_name(data.vbox_window.class_name())
                .forced_flags(data.vbox_window.forced_flags())
                .flags(Some(((WindowFlags::WINDOW | WindowFlags::VISIBLE | WindowFlags::RESIZABLE).bits(), 0)).unwrap_or(data.vbox_window.flags()))
                .size((250, 400))
                .position((300, 450))
                .text("VBOX test")
                .build()?;
            data.vbox_window.handle = vbox_window.handle.clone();

            let button4 = ControlBase::build_hwnd()
              .class_name(data.button4.class_name())
              .forced_flags(data.button4.forced_flags())
              .flags(data.button4.flags())
              .text("Button 4")
              .parent(Some(&vbox_window))
              .build()?;
            data.button4.handle = button4.handle.clone();

            let button5 = ControlBase::build_hwnd()
              .class_name(data.button5.class_name())
              .forced_flags(data.button5.forced_flags())
              .flags(data.button5.flags())
              .text("Button 5")
              .parent(Some(&vbox_window))
              .build()?;
            data.button5.handle = button5.handle.clone();

            let button6 = ControlBase::build_hwnd()
              .class_name(data.button6.class_name())
              .forced_flags(data.button6.forced_flags())
              .flags(data.button6.flags())
              .text("Button 6")
              .parent(Some(&vbox_window))
              .build()?;
            data.button6.handle = button6.handle.clone();

            // Wrap-up
            let ui = Rc::new(TestLayoutAppUi { inner: data });

            // Events
            let window_handles = [&ui.hbox_window.handle, &ui.vbox_window.handle];
            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, handle: ControlHandle| {
                    match evt {
                        E::OnWindowClose => 
                            if handle == evt_ui.hbox_window.handle {
                                super::close(&evt_ui.inner, evt);
                            } else if handle == evt_ui.vbox_window.handle {
                                super::close(&evt_ui.inner, evt);
                            }
                        _ => {}
                    }
                };

                bind_event_handler(handle, handle_events);
            }

            // Layouts
            HBoxLayout::builder()
                .parent(&ui.hbox_window.handle)
                .child(0, &ui.button1.handle)
                .child(1, &ui.button2.handle)
                .child(2, &ui.button3.handle)
                .build();

            VBoxLayout::builder()
                .parent(&ui.vbox_window.handle)
                .child(0, &ui.button4.handle)
                .child(1, &ui.button5.handle)
                .child(2, &ui.button6.handle)
                .build();

            return Ok(ui);
        }
    }

    impl Deref for TestLayoutAppUi {
        type Target = TestLayoutApp;

        fn deref(&self) -> &TestLayoutApp {
            &self.inner
        }
    }
}

fn close(_app: &TestLayoutApp, _e: Event) {
    stop_thread_dispatch();
}

#[test]
fn test_layouts() {
    enable_visual_styles();
    init_common_controls();
    
    let app = TestLayoutApp::build_ui(Default::default()).expect("Failed to build UI");

    app.hbox_window.set_focus();

    dispatch_thread_events();
}
