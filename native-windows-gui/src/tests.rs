use crate::*;
use std::cell::RefCell;

#[derive(Default)]
pub struct TestRun {
    button: bool,
    textedit: bool,
    combo_box: bool,
    menu: bool
}


#[derive(Default)]
pub struct TestApp {
    // App data
    runs: RefCell<TestRun>,

    // Controls
    window: Window,
    test_button: Button,
    test_input: TextInput, 
    test_combobox: ComboBox<&'static str>,

    // Menu
    window_menu: Menu,
    window_submenu1: Menu,
    window_menu_sep: MenuSeparator,
    window_menu_item1: MenuItem,
    window_menu_item2: MenuItem,
    window_menu_item3: MenuItem,

    // Timer / Notice
    timer: Timer,
    notice: Notice,
    
    // Control window
    control_window: Window,
    events_show: TextInput,
    run_button_test: Button,
    run_textedit_test: Button,
    run_combobox_test: Button,
    run_window_test: Button,
    run_menu_test: Button,
    run_thread_test: Button,
    focus_test: Button,
}

mod basic_app_ui {
    use super::TestApp;
    use crate::{NativeUi, ControlBase, ControlHandle, SystemError, bind_event_handler, WindowFlags};
    use std::rc::Rc;
    use std::ops::Deref;

    #[allow(dead_code)]
    pub struct TestAppUi {
        inner: TestApp
    }

    #[allow(unused_mut)]
    impl NativeUi<TestApp, TestAppUi> for TestApp {

        fn build_ui(mut data: TestApp) -> Result<Rc<TestAppUi>, SystemError> {
            use crate::Event as E;

            // Controls
            let window = ControlBase::build_hwnd()
              .class_name(data.window.class_name())
              .forced_flags(data.window.forced_flags())
              .flags(data.window.flags())
              .size((300, 120))
              .position((300, 300))
              .text("Tests")
              .build()?;
            data.window.handle = window.handle.clone();

            let control_window = ControlBase::build_hwnd()
              .class_name(data.control_window.class_name())
              .forced_flags(data.control_window.forced_flags())
              .flags(((WindowFlags::WINDOW | WindowFlags::VISIBLE).bits(), 0))
              .size((280, 300))
              .position((650, 300))
              .text("Controls Panel")
              .parent(&window)
              .build()?;
            data.control_window.handle = control_window.handle.clone();

            let test_button = ControlBase::build_hwnd()
              .class_name(data.test_button.class_name())
              .forced_flags(data.test_button.forced_flags())
              .flags(data.test_button.flags())
              .size((100, 40))
              .position((5, 5))
              .text("Test Button")
              .parent(&window)
              .build()?;
            data.test_button.handle = test_button.handle.clone();

            let test_input = ControlBase::build_hwnd()
              .class_name(data.test_input.class_name())
              .forced_flags(data.test_input.forced_flags())
              .flags(data.test_input.flags())
              .size((120, 25))
              .position((155, 15))
              .text("Test TextEdit")
              .parent(&window)
              .build()?;
            data.test_input.handle = test_input.handle.clone();

            let test_combobox = ControlBase::build_hwnd()
              .class_name(data.test_combobox.class_name())
              .forced_flags(data.test_combobox.forced_flags())
              .flags(data.test_combobox.flags())
              .size((120, 25))
              .position((5, 50))
              .parent(&window)
              .build()?;
            data.test_combobox.handle = test_combobox.handle.clone();
            data.test_combobox.set_collection(vec!["TEST1", "TEST2"]);

            let events_show = ControlBase::build_hwnd()
              .class_name(data.events_show.class_name())
              .forced_flags(data.events_show.forced_flags())
              .flags(data.events_show.flags())
              .size((255, 25))
              .position((5, 5))
              .text("")
              .parent(&control_window)
              .build()?;
            data.events_show.handle = events_show.handle.clone();

            let run_button_test = ControlBase::build_hwnd()
              .class_name(data.run_button_test.class_name())
              .forced_flags(data.run_button_test.forced_flags())
              .flags(data.run_button_test.flags())
              .size((125, 30))
              .position((5, 35))
              .text("Run button tests")
              .parent(&control_window)
              .build()?;
            data.run_button_test.handle = run_button_test.handle.clone();

            let run_textedit_test = ControlBase::build_hwnd()
              .class_name(data.run_textedit_test.class_name())
              .forced_flags(data.run_textedit_test.forced_flags())
              .flags(data.run_textedit_test.flags())
              .size((125, 30))
              .position((5, 65))
              .text("Run textedit tests")
              .parent(&control_window)
              .build()?;
            data.run_textedit_test.handle = run_textedit_test.handle.clone();

            let run_combobox_test = ControlBase::build_hwnd()
              .class_name(data.run_combobox_test.class_name())
              .forced_flags(data.run_combobox_test.forced_flags())
              .flags(data.run_combobox_test.flags())
              .size((125, 30))
              .position((135, 65))
              .text("Run combo tests")
              .parent(&control_window)
              .build()?;
            data.run_combobox_test.handle = run_combobox_test.handle.clone();

            let run_window_test = ControlBase::build_hwnd()
              .class_name(data.run_window_test.class_name())
              .forced_flags(data.run_window_test.forced_flags())
              .flags(data.run_window_test.flags())
              .size((125, 30))
              .position((5, 95))
              .text("Run window tests")
              .parent(&control_window)
              .build()?;
            data.run_window_test.handle = run_window_test.handle.clone();

            let run_menu_test = ControlBase::build_hwnd()
              .class_name(data.run_menu_test.class_name())
              .forced_flags(data.run_menu_test.forced_flags())
              .flags(data.run_menu_test.flags())
              .size((125, 30))
              .position((135, 95))
              .text("Run menu tests")
              .parent(&control_window)
              .build()?;
            data.run_menu_test.handle = run_menu_test.handle.clone();

            let run_thread_test = ControlBase::build_hwnd()
              .class_name(data.run_thread_test.class_name())
              .forced_flags(data.run_thread_test.forced_flags())
              .flags(data.run_thread_test.flags())
              .size((125, 30))
              .position((5, 125))
              .text("Run thread tests")
              .parent(&control_window)
              .build()?;
            data.run_thread_test.handle = run_thread_test.handle.clone();

            let focus_test = ControlBase::build_hwnd()
              .class_name(data.focus_test.class_name())
              .forced_flags(data.focus_test.forced_flags())
              .flags(data.focus_test.flags())
              .size((125, 30))
              .position((135, 35))
              .text("Focus text")
              .parent(&control_window)
              .build()?;
            data.focus_test.handle = focus_test.handle.clone();

            // Menus & Actions
            let window_menu = ControlBase::build_hmenu()
                .text("Test menu")
                .item(false)
                .parent(&window)
                .build()?;
            data.window_menu.handle = window_menu.handle.clone();

            let window_submenu1 = ControlBase::build_hmenu()
                .text("Test Submenu")
                .item(false)
                .parent(&window_menu)
                .build()?;
            data.window_submenu1.handle = window_submenu1.handle.clone();

            let window_menu_sep = ControlBase::build_hmenu()
                .separator(true)
                .parent(&window_menu)
                .build()?;
            data.window_menu_sep.handle = window_menu_sep.handle.clone();

            let window_menu_item1 = ControlBase::build_hmenu()
                .text("Test Item 1")
                .item(true)
                .parent(&window_menu)
                .build()?;
            data.window_menu_item1.handle = window_menu_item1.handle.clone();

            let window_menu_item2 = ControlBase::build_hmenu()
                .text("Test Item 2")
                .item(true)
                .parent(&window_submenu1)
                .build()?;
            data.window_menu_item2.handle = window_menu_item2.handle.clone();

            let window_menu_item3 = ControlBase::build_hmenu()
                .text("Test Item 3")
                .item(true)
                .parent(&window)
                .build()?;
            data.window_menu_item3.handle = window_menu_item3.handle.clone();

            // Timers
            let timer = ControlBase::build_timer()
                .interval(1000)
                .stopped(false)
                .parent(&window)
                .build()?;
            data.timer.handle = timer.handle;

            let notice = ControlBase::build_notice()
                .parent(&window)
                .build()?;
            data.notice.handle = notice.handle;

            // Wrap-up
            let ui = Rc::new(TestAppUi { inner: data });

            // Events
            let window_handles = [&ui.window.handle, &ui.control_window.handle];
            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, handle: ControlHandle| {
                    match evt {
                        E::OnButtonClick => 
                            if handle == evt_ui.run_button_test.handle {
                                super::test_button(&evt_ui.inner, evt);

                            } else if handle == evt_ui.run_textedit_test.handle {
                                super::test_textedit(&evt_ui.inner, evt);

                            } else if handle == evt_ui.run_combobox_test.handle {
                                super::test_combobox(&evt_ui.inner, evt);

                            } else if handle == evt_ui.run_window_test.handle {
                                super::test_window(&evt_ui.inner, evt);

                            } else if handle == evt_ui.run_menu_test.handle {
                                super::test_menu(&evt_ui.inner, evt);

                            } else if handle == evt_ui.test_button.handle {
                                super::test_events(&evt_ui.inner, evt);

                            } else if handle == evt_ui.focus_test.handle {
                                super::focus(&evt_ui.inner, evt);
                            },
                        E::OnButtonDoubleClick => 
                            if handle == evt_ui.test_button.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnTextInput =>
                            if handle == evt_ui.test_input.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnComboBoxClosed =>
                            if handle == evt_ui.test_combobox.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnComboBoxDropdown =>
                            if handle == evt_ui.test_combobox.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnComboBoxDoubleClick =>
                            if handle == evt_ui.test_combobox.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnComboxBoxSelection =>
                            if handle == evt_ui.test_combobox.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnMenuItemClick =>
                            if handle == evt_ui.window_menu_item1.handle {
                                super::test_events(&evt_ui.inner, evt);
                            } else if handle == evt_ui.window_menu_item2.handle {
                                super::test_events(&evt_ui.inner, evt);
                            } else if handle == evt_ui.window_menu_item3.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnTimerTick => {
                            if handle == evt_ui.timer.handle {
                                super::timer_tick(&evt_ui.inner, evt);
                            }
                        },
                        E::OnWindowClose => 
                            if handle == evt_ui.window.handle {
                                super::close(&evt_ui.inner, evt);
                            },
                        _ => {}
                    }
                };

                bind_event_handler(handle, handle_events);
            }

            return Ok(ui);
        }
    }

    
    impl Deref for TestAppUi {
        type Target = TestApp;

        fn deref(&self) -> &TestApp {
            &self.inner
        }
    }

}

fn focus(app: &TestApp, _e: Event) {
    app.test_input.set_focus();
}

fn test_button(app: &TestApp, _e: Event) {
    if !app.runs.borrow().button {
        assert_eq!(&app.test_button.text(), "Test Button");
        app.test_button.set_text("New Text");
        assert_eq!(&app.test_button.text(), "New Text");

        assert_eq!(app.test_button.position(), (5, 5));
        app.test_button.set_position(10, 10);
        assert_eq!(app.test_button.position(), (10, 10));

        assert_eq!(app.test_button.size(), (100, 40));
        app.test_button.set_size(130, 30);
        assert_eq!(app.test_button.size(), (130, 30));

        assert_eq!(app.test_button.visible(), true);
        app.test_button.set_visible(false);
        assert_eq!(app.test_button.visible(), false);
        app.test_button.set_visible(true);

        app.test_button.set_focus();
        assert_eq!(app.test_button.focus(), true);
        app.window.set_focus();
        assert_eq!(app.test_button.focus(), false);

        assert_eq!(app.test_button.enabled(), true);
        app.test_button.set_enabled(false);
        assert_eq!(app.test_button.enabled(), false);

        app.runs.borrow_mut().button = true;
    } else {
        app.test_button.set_text("Test Button");
        app.test_button.set_position(5, 5);
        app.test_button.set_size(100, 40);
        app.test_button.set_enabled(true);
        app.runs.borrow_mut().button = false;
    }
}

fn test_textedit(app: &TestApp, _e: Event) {
    if !app.runs.borrow().textedit {

        app.test_input.set_text("New Text");
        assert_eq!(&app.test_input.text(), "New Text");

        app.test_input.set_limit(32);
        assert_eq!(app.test_input.limit(), 32);

        assert_eq!(app.test_input.password_char(), None);
        app.test_input.set_password_char(Some('X'));
        assert_eq!(app.test_input.password_char(), Some('X'));

        app.test_input.set_modified(true);
        assert_eq!(app.test_input.modified(), true);

        app.test_input.set_selection(0..4);
        assert_eq!(app.test_input.selection(), 0..4);

        assert_eq!(app.test_input.len(), 8);

        assert_eq!(app.test_input.position(), (155, 15));
        app.test_input.set_position(150, 20);
        assert_eq!(app.test_input.position(), (150, 20));

        assert_eq!(app.test_input.size(), (120, 25));
        app.test_input.set_size(115, 30);
        assert_eq!(app.test_input.size(), (115, 30));

        assert_eq!(app.test_input.visible(), true);
        app.test_input.set_visible(false);
        assert_eq!(app.test_input.visible(), false);
        app.test_input.set_visible(true);

        app.test_input.set_focus();
        assert_eq!(app.test_input.focus(), true);
        app.window.set_focus();
        assert_eq!(app.test_input.focus(), false);

        assert_eq!(app.test_input.readonly(), false);
        app.test_input.set_readonly(true);
        assert_eq!(app.test_input.readonly(), true);

        assert_eq!(app.test_input.enabled(), true);
        app.test_input.set_enabled(false);
        assert_eq!(app.test_input.enabled(), false);

        app.runs.borrow_mut().textedit = true;
    } else {
        app.test_input.set_text("Test TextEdit");
        app.test_input.set_position(155, 15);
        app.test_input.set_size(120, 25);
        app.test_input.set_enabled(true);
        app.test_input.set_readonly(false);
        app.test_input.set_password_char(None);

        app.runs.borrow_mut().textedit = false;
    }
}

fn test_combobox(app: &TestApp, _e: Event) {
    if !app.runs.borrow().combo_box {
        {
            let col = app.test_combobox.collection();
            assert_eq!(&col as &[&'static str], &["TEST1", "TEST2"]);
        }

        {
            let mut col = app.test_combobox.collection_mut();
            col.push("TEST3");
        }

        app.test_combobox.sync();
        app.test_combobox.push("Hello!");

        app.test_combobox.set_selection(None);
        assert_eq!(app.test_combobox.selection(), None);
        assert_eq!(app.test_combobox.selection_string(), None);

        app.test_combobox.set_selection(Some(2));
        assert_eq!(app.test_combobox.selection(), Some(2));
        assert_eq!(app.test_combobox.selection_string(), Some("TEST3".to_string()));

        assert_eq!(app.test_combobox.set_selection_string("hel"), Some(3));
        assert_eq!(app.test_combobox.selection(), Some(3));
        assert_eq!(app.test_combobox.selection_string(), Some("Hello!".to_string()));

        app.test_combobox.sort();
        assert_eq!(app.test_combobox.set_selection_string("hel"), Some(0));

        app.test_combobox.insert(1, "BOO!");
        app.test_combobox.insert(std::usize::MAX, "Ahoy!!");
        assert_eq!(app.test_combobox.set_selection_string("BOO!"), Some(1));
        assert_eq!(app.test_combobox.set_selection_string("Ahoy!!"), Some(5));

        app.test_combobox.dropdown(true);

        app.runs.borrow_mut().combo_box = true;
    } else {
        app.test_combobox.set_collection(vec!["TEST1", "TEST2"]);
        app.runs.borrow_mut().combo_box = false;
    }
}

fn test_window(_app: &TestApp, _e: Event) {

}

fn test_menu(app: &TestApp, _e: Event) {
    if !app.runs.borrow().menu {
        assert_eq!(app.window_menu_item1.enabled(), true);
        app.window_menu_item1.set_enabled(false);
        assert_eq!(app.window_menu_item1.enabled(), false);

        assert_eq!(app.window_submenu1.enabled(), true);
        app.window_submenu1.set_enabled(false);
        assert_eq!(app.window_submenu1.enabled(), false);

        app.runs.borrow_mut().menu = true;
    } else {
        app.window_menu_item1.set_enabled(true);
        app.window_submenu1.set_enabled(true);
        app.runs.borrow_mut().menu = false;
    }
}

fn test_events(app: &TestApp, e: Event) {
    app.events_show.set_text(&format!("{:?}", e));
}

fn timer_tick(_app: &TestApp, _e: Event) {
    println!("TICK!");
}


fn close(_app: &TestApp, _e: Event) {
    stop_thread_dispatch();
}

#[test]
fn test_everything() {
    enable_visual_styles();
    
    let app = TestApp::build_ui(Default::default()).expect("Failed to build UI");

    app.window.set_focus();

    dispatch_thread_events();
}
