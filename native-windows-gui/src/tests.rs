use crate::*;
use std::cell::RefCell;

#[derive(Default)]
pub struct TestRun {
    button: bool,
    textedit: bool,
    combo_box: bool,
    menu: bool,
    label: bool,
    status: bool,
    window: bool,
    image: bool,
    date_picker: bool,
    progress_bar: bool,
    check_box: bool,
}


#[derive(Default)]
pub struct PartialApp1 {
    window: Window,
    test: Button
}

#[derive(Default)]
pub struct PartialApp2 {
    test: Button
}

#[derive(Default)]
pub struct TestApp {
    // App data
    runs: RefCell<TestRun>,

    // Partial
    p1: PartialApp1,
    p2: PartialApp2,

    // Resources
    font: Font,
    ico1: Image,
    ferris: Image,

    // Controls
    window: Window,
    window_status: StatusBar,
    test_button: Button,
    test_input: TextInput, 
    test_combobox: ComboBox<&'static str>,
    test_label: Label,
    test_image: ImageFrame,
    test_checkbox: CheckBox,
    test_radio1: RadioButton,
    test_radio2: RadioButton,

    #[cfg(feature = "datetime-picker")]
    dtpick: DatePicker,

    #[cfg(feature = "progress-bar")]
    pbar: ProgressBar,

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

    // File dialog
    #[cfg(feature = "file-dialog")]
    open_file: FileDialog,
    open_file_button: Button,
    
    // Control window
    control_window: Window,
    events_show: TextInput,
    run_button_test: Button,
    run_textedit_test: Button,
    run_combobox_test: Button,
    run_window_test: Button,
    run_menu_test: Button,
    run_thread_test: Button,
    run_label_test: Button,
    run_status_test: Button,
    run_datepicker_test: Button,
    run_progressbar_test: Button,
    run_check_test: Button,
    focus_test: Button,
}

#[allow(unused_imports)]
mod partial_app_1_ui {
    use super::*;
    use crate::{PartialUi, ControlBase, SystemError, WindowFlags};

    #[allow(unused_mut)]
    impl PartialUi<PartialApp1> for PartialApp1 {

        fn build_partial(data: &mut PartialApp1, parent: Option<&ControlBase>) -> Result<(), SystemError> {

            let window = ControlBase::build_hwnd()
              .class_name(data.window.class_name())
              .forced_flags(data.window.forced_flags())
              .flags(((WindowFlags::WINDOW | WindowFlags::VISIBLE).bits(), 0))
              .size((300, 300))
              .position((1200, 300))
              .text("Partial 1 Window")
              .build()?;
            data.window.handle = window.handle.clone();

            let test = ControlBase::build_hwnd()
              .class_name(data.test.class_name())
              .forced_flags(data.test.forced_flags())
              .flags(data.test.flags())
              .size((290, 290))
              .position((5, 5))
              .text("Partial Button")
              .parent(Some(&window).or(parent.clone()) )
              .build()?;
            data.test.handle = test.handle.clone();

            Ok(())
        }

    }

}


#[allow(unused_imports)]
mod partial_app_2_ui {
    use super::*;
    use crate::{PartialUi, ControlBase, SystemError, WindowFlags};

    #[allow(unused_mut)]
    impl PartialUi<PartialApp2> for PartialApp2 {

        fn build_partial(data: &mut PartialApp2, parent: Option<&ControlBase>) -> Result<(), SystemError> {
            
            let test = ControlBase::build_hwnd()
              .class_name(data.test.class_name())
              .forced_flags(data.test.forced_flags())
              .flags(data.test.flags())
              .size((150, 30))
              .position((5, 80))
              .text("Partial Button")
              .parent(None.or(parent.clone()))
              .build()?;
            data.test.handle = test.handle.clone();
        
            Ok(())
        }

    }

}


#[allow(unused_imports)]
mod test_app_ui {
    use super::*;
    use crate::{NativeUi, ControlBase, ControlHandle, SystemError, bind_event_handler, WindowFlags};
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct TestAppUi {
        inner: TestApp
    }

    #[allow(unused_mut)]
    impl NativeUi<TestApp, TestAppUi> for TestApp {

        fn build_ui(mut data: TestApp) -> Result<Rc<TestAppUi>, SystemError> {
            use crate::Event as E;

            #[cfg(feature = "file-dialog")]
            fn setup_file_dialog(app: &mut TestApp) -> Result<(), SystemError> {
                app.open_file = FileDialog::builder()
                  .action(FileDialogAction::Open)
                  .multiselect(false)
                  .title("Select a file")
                  .build()?;

                  Ok(())
            }

            #[cfg(feature = "datetime-picker")]
            fn setup_datetime_picker(app: &mut TestApp, window: &ControlBase) -> Result<(), SystemError> {
               let dtpick = ControlBase::build_hwnd()
                .class_name(app.dtpick.class_name())
                .forced_flags(app.dtpick.forced_flags())
                .flags(app.dtpick.flags())
                .size((125, 30))
                .position((5, 195))
                .parent(Some(window))
                .build()?;

                app.dtpick.handle = dtpick.handle.clone();

                Ok(())
            }

            #[cfg(feature = "progress-bar")]
            fn setup_progress_bar(app: &mut TestApp, window: &ControlBase) -> Result<(), SystemError> {
                let pbar = ControlBase::build_hwnd()
                .class_name(app.pbar.class_name())
                .forced_flags(app.pbar.forced_flags())
                .flags(app.pbar.flags())
                .size((200, 30))
                .position((5, 235))
                .parent(Some(window))
                .build()?;

                app.pbar.handle = pbar.handle.clone();

                Ok(())
            }

            #[cfg(not(feature = "file-dialog"))]
            fn setup_file_dialog(_app: &mut TestApp) -> Result<(), SystemError> {
                Ok(())
            }

            #[cfg(not(feature = "datetime-picker"))]
            fn setup_datetime_picker(_app: &mut TestApp, _window: &ControlBase) -> Result<(), SystemError> {
                Ok(())
            }

            #[cfg(not(feature = "progress-bar"))]
            fn setup_progress_bar(_app: &mut TestApp, _window: &ControlBase) -> Result<(), SystemError> {
                Ok(())
            }


            // Font
            let font = Font::builder()
              .size(24)
              .weight(700)
              .family(Some("Arial"))
              .build()?;
            data.font = font;

            // Image
            data.ico1 = Image::icon("./test_rc/cog.ico", None, false)?;
            data.ferris = Image::bitmap("./test_rc/ferris.bmp", None, false)?;

            // Controls
            let window = ControlBase::build_hwnd()
              .class_name(data.window.class_name())
              .forced_flags(data.window.forced_flags())
              .flags(data.window.flags())
              .size((350, 370))
              .position((300, 250))
              .text("Tests")
              .build()?;
            data.window.handle = window.handle.clone();
            data.window.set_icon(Some(&data.ico1));

            let window_status = ControlBase::build_hwnd()
              .class_name(data.window_status.class_name())
              .forced_flags(data.window_status.forced_flags())
              .flags(data.window_status.flags())
              .parent(Some(&window))
              .build()?;
            data.window_status.handle = window_status.handle;
            data.window_status.set_text(0, "TEST status");
            data.window_status.hook_parent_resize();

            let control_window = ControlBase::build_hwnd()
              .class_name(data.control_window.class_name())
              .forced_flags(data.control_window.forced_flags())
              .flags(((WindowFlags::WINDOW | WindowFlags::VISIBLE).bits(), 0))
              .size((280, 300))
              .position((680, 300))
              .text("Controls Panel")
              .parent(Some(&window))
              .build()?;
            data.control_window.handle = control_window.handle.clone();
            data.control_window.set_icon(Some(&data.ico1));

            let test_button = ControlBase::build_hwnd()
              .class_name(data.test_button.class_name())
              .forced_flags(data.test_button.forced_flags())
              .flags(data.test_button.flags())
              .size((100, 40))
              .position((5, 5))
              .text("Test Button")
              .parent(Some(&window))
              .build()?;
            data.test_button.handle = test_button.handle.clone();

            let test_input = ControlBase::build_hwnd()
              .class_name(data.test_input.class_name())
              .forced_flags(data.test_input.forced_flags())
              .flags(data.test_input.flags())
              .size((120, 25))
              .position((155, 15))
              .text("Test TextEdit")
              .parent(Some(&window))
              .build()?;
            data.test_input.handle = test_input.handle.clone();

            let test_combobox = ControlBase::build_hwnd()
              .class_name(data.test_combobox.class_name())
              .forced_flags(data.test_combobox.forced_flags())
              .flags(data.test_combobox.flags())
              .size((150, 25))
              .position((5, 50))
              .parent(Some(&window))
              .build()?;
            data.test_combobox.handle = test_combobox.handle.clone();
            data.test_combobox.set_collection(vec!["TEST1", "TEST2"]);

            let test_label = ControlBase::build_hwnd()
              .class_name(data.test_label.class_name())
              .forced_flags(data.test_label.forced_flags())
              .flags(data.test_label.flags())
              .text("Test label")
              .size((150, 25))
              .position((5, 115))
              .parent(Some(&window))
              .build()?;
            data.test_label.handle = test_label.handle.clone();
            data.test_label.set_font(Some(&data.font));

            let test_image = ControlBase::build_hwnd()
              .class_name(data.test_image.class_name())
              .forced_flags(data.test_image.forced_flags())
              .flags(data.test_image.flags())
              .size((150, 99))
              .position((170, 65))
              .parent(Some(&window))
              .build()?;
            data.test_image.handle = test_image.handle.clone();
            data.test_image.set_image(Some(&data.ferris));

            let test_checkbox = ControlBase::build_hwnd()
              .class_name(data.test_checkbox.class_name())
              .forced_flags(data.test_checkbox.forced_flags())
              .flags(data.test_checkbox.flags())
              .size((130, 25))
              .position((215, 230))
              .text("Test CheckBox")
              .parent(Some(&window))
              .build()?;
            data.test_checkbox.handle = test_checkbox.handle.clone();

            let test_radio1 = ControlBase::build_hwnd()
              .class_name(data.test_radio1.class_name())
              .forced_flags(data.test_radio1.forced_flags())
              .flags(data.test_radio1.flags())
              .size((130, 25))
              .position((215, 255))
              .text("Test Radio 1")
              .parent(Some(&window))
              .build()?;
            data.test_radio1.handle = test_radio1.handle.clone();

            let test_radio2 = ControlBase::build_hwnd()
              .class_name(data.test_radio2.class_name())
              .forced_flags(data.test_radio2.forced_flags())
              .flags(data.test_radio2.flags())
              .size((130, 25))
              .position((215, 280))
              .text("Test Radio 2")
              .parent(Some(&window))
              .build()?;
            data.test_radio2.handle = test_radio2.handle.clone();

            let open_file_button = ControlBase::build_hwnd()
                    .class_name(data.open_file_button.class_name())
                    .forced_flags(data.open_file_button.forced_flags())
                    .flags(data.open_file_button.flags())
                    .size((125, 30))
                    .position((5, 155))
                    .text("Choose a file")
                    .parent(Some(&window))
                    .build()?;
            data.open_file_button.handle = open_file_button.handle.clone();
            data.open_file_button.set_enabled(cfg!(feature = "file-dialog"));

            setup_datetime_picker(&mut data, &window)?;
            setup_progress_bar(&mut data, &window)?;

            let events_show = ControlBase::build_hwnd()
              .class_name(data.events_show.class_name())
              .forced_flags(data.events_show.forced_flags())
              .flags(data.events_show.flags())
              .size((255, 25))
              .position((5, 5))
              .text("")
              .parent(Some(&control_window))
              .build()?;
            data.events_show.handle = events_show.handle.clone();

            let run_button_test = ControlBase::build_hwnd()
              .class_name(data.run_button_test.class_name())
              .forced_flags(data.run_button_test.forced_flags())
              .flags(data.run_button_test.flags())
              .size((125, 30))
              .position((5, 35))
              .text("Run button tests")
              .parent(Some(&control_window))
              .build()?;
            data.run_button_test.handle = run_button_test.handle.clone();

            let run_textedit_test = ControlBase::build_hwnd()
              .class_name(data.run_textedit_test.class_name())
              .forced_flags(data.run_textedit_test.forced_flags())
              .flags(data.run_textedit_test.flags())
              .size((125, 30))
              .position((5, 65))
              .text("Run textedit tests")
              .parent(Some(&control_window))
              .build()?;
            data.run_textedit_test.handle = run_textedit_test.handle.clone();

            let run_combobox_test = ControlBase::build_hwnd()
              .class_name(data.run_combobox_test.class_name())
              .forced_flags(data.run_combobox_test.forced_flags())
              .flags(data.run_combobox_test.flags())
              .size((125, 30))
              .position((135, 65))
              .text("Run combo tests")
              .parent(Some(&control_window))
              .build()?;
            data.run_combobox_test.handle = run_combobox_test.handle.clone();

            let run_window_test = ControlBase::build_hwnd()
              .class_name(data.run_window_test.class_name())
              .forced_flags(data.run_window_test.forced_flags())
              .flags(data.run_window_test.flags())
              .size((125, 30))
              .position((5, 95))
              .text("Run window tests")
              .parent(Some(&control_window))
              .build()?;
            data.run_window_test.handle = run_window_test.handle.clone();

            let run_menu_test = ControlBase::build_hwnd()
              .class_name(data.run_menu_test.class_name())
              .forced_flags(data.run_menu_test.forced_flags())
              .flags(data.run_menu_test.flags())
              .size((125, 30))
              .position((135, 95))
              .text("Run menu tests")
              .parent(Some(&control_window))
              .build()?;
            data.run_menu_test.handle = run_menu_test.handle.clone();

            let run_thread_test = ControlBase::build_hwnd()
              .class_name(data.run_thread_test.class_name())
              .forced_flags(data.run_thread_test.forced_flags())
              .flags(data.run_thread_test.flags())
              .size((125, 30))
              .position((5, 125))
              .text("Run thread tests")
              .parent(Some(&control_window))
              .build()?;
            data.run_thread_test.handle = run_thread_test.handle.clone();

            let run_label_test = ControlBase::build_hwnd()
              .class_name(data.run_label_test.class_name())
              .forced_flags(data.run_label_test.forced_flags())
              .flags(data.run_label_test.flags())
              .size((125, 30))
              .position((135, 125))
              .text("Run label tests")
              .parent(Some(&control_window))
              .build()?;
            data.run_label_test.handle = run_label_test.handle.clone();

            let run_status_test = ControlBase::build_hwnd()
              .class_name(data.run_status_test.class_name())
              .forced_flags(data.run_status_test.forced_flags())
              .flags(data.run_status_test.flags())
              .size((125, 30))
              .position((5, 155))
              .text("Run status tests")
              .parent(Some(&control_window))
              .build()?;
              
            data.run_status_test.handle = run_status_test.handle.clone();

            let run_check_test = ControlBase::build_hwnd()
              .class_name(data.run_check_test.class_name())
              .forced_flags(data.run_check_test.forced_flags())
              .flags(data.run_check_test.flags())
              .size((125, 30))
              .position((135, 185))
              .text("Run check tests")
              .parent(Some(&control_window))
              .build()?;
              
            data.run_check_test.handle = run_check_test.handle.clone();

            let run_datepicker_test = ControlBase::build_hwnd()
              .class_name(data.run_datepicker_test.class_name())
              .forced_flags(data.run_datepicker_test.forced_flags())
              .flags(data.run_datepicker_test.flags())
              .size((125, 30))
              .position((135, 155))
              .text("Run date tests")
              .parent(Some(&control_window))
              .build()?;
              
            data.run_datepicker_test.handle = run_datepicker_test.handle.clone();
            data.run_datepicker_test.set_enabled(cfg!(feature = "file-dialog"));

            let run_progressbar_test = ControlBase::build_hwnd()
              .class_name(data.run_progressbar_test.class_name())
              .forced_flags(data.run_progressbar_test.forced_flags())
              .flags(data.run_progressbar_test.flags())
              .size((125, 30))
              .position((5, 185))
              .text("Run progbar tests")
              .parent(Some(&control_window))
              .build()?;
              
            data.run_progressbar_test.handle = run_progressbar_test.handle.clone();
            data.run_progressbar_test.set_enabled(cfg!(feature = "progress-bar"));

            let focus_test = ControlBase::build_hwnd()
              .class_name(data.focus_test.class_name())
              .forced_flags(data.focus_test.forced_flags())
              .flags(data.focus_test.flags())
              .size((125, 30))
              .position((135, 35))
              .text("Focus text")
              .parent(Some(&control_window))
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

            // Control for partial ui
            PartialApp1::build_partial(&mut data.p1, Some(&window))?;
            PartialApp2::build_partial(&mut data.p2, Some(&window))?;

            // File dialogs
            setup_file_dialog(&mut data)?;

            // Timers
            let timer = ControlBase::build_timer()
                .interval(10000)
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
            let window_handles = [&ui.window.handle, &ui.control_window.handle, &ui.p1.window.handle];
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

                            } else if handle == evt_ui.run_thread_test.handle {
                                super::test_thread(&evt_ui.inner, evt);

                            } else if handle == evt_ui.run_label_test.handle {
                                super::test_label(&evt_ui.inner, evt);

                            } else if handle == evt_ui.test_button.handle {
                                super::test_events(&evt_ui.inner, evt);

                            } else if handle == evt_ui.run_status_test.handle {
                                super::test_status(&evt_ui.inner, evt);

                            } else if handle == evt_ui.run_check_test.handle {
                                super::test_check(&evt_ui.inner, evt);

                            } else if handle == evt_ui.focus_test.handle {
                                super::focus(&evt_ui.inner, evt);

                            } else if handle == evt_ui.p1.test.handle {
                                super::test_partial(&evt_ui.inner, evt);

                            } else if handle == evt_ui.p2.test.handle {
                                super::test_partial(&evt_ui.inner, evt);

                            } else if handle == evt_ui.open_file_button.handle {
                                super::test_file_dialog(&evt_ui.inner, evt);
                            } else {
                                super::dispatch_date_time_tests(&handle, &evt_ui.inner);
                                super::dispatch_progress_bar_tests(&handle, &evt_ui.inner);
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
                        E::OnComboxBoxSelection =>
                            if handle == evt_ui.test_combobox.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnLabelClick =>
                            if handle == evt_ui.test_label.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnLabelDoubleClick =>
                            if handle == evt_ui.test_label.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnImageFrameClick => 
                            if handle == evt_ui.test_image.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnImageFrameDoubleClick =>
                            if handle == evt_ui.test_image.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnDatePickerDropdown =>
                            super::dispatch_dtp_event(&evt_ui.inner, evt, handle),
                        E::OnDatePickerClosed =>
                            super::dispatch_dtp_event(&evt_ui.inner, evt, handle),
                        E::OnDatePickerChanged =>
                            super::dispatch_dtp_event(&evt_ui.inner, evt, handle),
                        E::OnMenuItemClick =>
                            if handle == evt_ui.window_menu_item1.handle {
                                super::test_events(&evt_ui.inner, evt);
                            } else if handle == evt_ui.window_menu_item2.handle {
                                super::test_events(&evt_ui.inner, evt);
                            } else if handle == evt_ui.window_menu_item3.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnTimerTick => 
                            if handle == evt_ui.timer.handle {
                                super::test_events(&evt_ui.inner, evt);
                            },
                        E::OnNotice => 
                            if handle == evt_ui.notice.handle {
                                super::test_events(&evt_ui.inner, evt);
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
        app.test_input.set_position(150, 5);
        assert_eq!(app.test_input.position(), (150, 5));

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

fn test_window(app: &TestApp, _e: Event) {
    if !app.runs.borrow().window {
        app.runs.borrow_mut().window = true;

        assert_eq!(app.window.icon().as_ref(), Some(&app.ico1));
    } else {
        app.runs.borrow_mut().window = false;
    }
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

fn test_thread(app: &TestApp, _e: Event) {
    use std::thread;
    use std::time::Duration;

    let notice_sender = app.notice.sender();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        notice_sender.notice();
    });
}

fn test_label(app: &TestApp, _e: Event) {
    if !app.runs.borrow().label {
        app.runs.borrow_mut().label = true;
    } else {
        app.runs.borrow_mut().label = false;
    }
}

fn test_status(app: &TestApp, _e: Event) {
    if !app.runs.borrow().status {

        app.window_status.set_text(0, "Status changed!");
        assert_eq!(&app.window_status.text(0), "Status changed!");

        app.window_status.set_font(Some(&app.font));
        assert_eq!(app.window_status.font().as_ref(), Some(&app.font));

        app.window_status.set_min_height(55);

        app.runs.borrow_mut().status = true;
    } else {

        app.window_status.set_font(None);
        app.window_status.set_min_height(25);

        app.runs.borrow_mut().status = false;
    }
}

fn test_check(app: &TestApp, _e: Event) {
    if !app.runs.borrow().check_box {

        app.test_checkbox.set_tristate(true);
        assert!(app.test_checkbox.tristate());

        app.test_checkbox.set_check_state(CheckBoxState::Checked);
        assert_eq!(app.test_checkbox.check_state(), CheckBoxState::Checked);

        app.test_radio2.set_check_state(RadioButtonState::Checked);
        assert_eq!(app.test_radio2.check_state(), RadioButtonState::Checked);

        app.test_radio1.set_check_state(RadioButtonState::Unchecked);
        assert_eq!(app.test_radio1.check_state(), RadioButtonState::Unchecked);

        app.runs.borrow_mut().check_box = true;
    } else {
        app.test_checkbox.set_tristate(false);
        app.runs.borrow_mut().check_box = false;
    }
}

#[cfg(feature = "file-dialog")]
fn test_file_dialog(app: &TestApp, _e: Event) {
    if app.open_file.run() {
        if let Ok(file_name) = app.open_file.get_selected_item() {
            let text = format!("{:?}", file_name);
            simple_message("Selected file", &text);
        }
    }

    app.open_file.set_multiselect(true).expect("FAIL");
    assert_eq!(app.open_file.multiselect(), true);

    app.open_file.set_title("Test 123");

    if app.open_file.run() {
        if let Ok(file_names) = app.open_file.get_selected_items() {
            let text = format!("{:?}", file_names);
            simple_message("Selected files", &text);
        }
    }
}

#[cfg(not(feature = "file-dialog"))]
fn test_file_dialog(_app: &TestApp, _e: Event) {
}

#[cfg(feature = "datetime-picker")]
fn dispatch_dtp_event(app: &TestApp, evt: Event, handle: ControlHandle) {
    if handle == app.dtpick.handle {
        test_events(app, evt);
    }
} 

#[cfg(not(feature = "datetime-picker"))]
fn dispatch_dtp_event(_app: &TestApp, _evt: Event, handle: ControlHandle) {
} 

#[cfg(feature = "datetime-picker")]
fn dispatch_date_time_tests(handle: &ControlHandle, app: &TestApp) {
    if handle != &app.run_datepicker_test.handle {
        return;
    }

    if !app.runs.borrow().date_picker {

        let v = DatePickerValue { year: 2000, month: 10, day: 5 };
        app.dtpick.set_value(Some(v));
        assert_eq!(app.dtpick.value(), Some(v));
        assert_eq!(app.dtpick.checked(), true);

        app.dtpick.set_value(None);
        assert_eq!(app.dtpick.value(), None);
        assert_eq!(app.dtpick.checked(), false);

        app.dtpick.set_format("dddd MMM dd', 'yyyy");
        app.dtpick.set_size(250, 30);

        let up = DatePickerValue { year: 2000, month: 1, day: 1 };
        let down = DatePickerValue { year: 2001, month: 1, day: 1 };
        app.dtpick.set_range(&[up, down]);
        assert_eq!(app.dtpick.range(), [up, down]);

        app.runs.borrow_mut().date_picker = true;
    } else {
        app.runs.borrow_mut().date_picker = false;
    }
}

#[cfg(not(feature = "datetime-picker"))]
fn dispatch_date_time_tests(_handle: &ControlHandle, _app: &TestApp) {
}

#[cfg(feature = "progress-bar")]
fn dispatch_progress_bar_tests(handle: &ControlHandle, app: &TestApp) {
    if handle != &app.run_progressbar_test.handle {
        return;
    }

    if !app.runs.borrow().progress_bar {
        app.pbar.set_range(0..1000);

        let r = app.pbar.range();
        assert!(r.start == 0 && r.end == 1000);

        app.pbar.set_pos(500);
        assert!(app.pbar.pos() == 500);

        app.pbar.set_step(100);
        assert!(app.pbar.step() == 100);

        app.pbar.set_state(ProgressBarState::Paused);
        assert!(app.pbar.state() == ProgressBarState::Paused);

        app.pbar.advance();
        assert!(app.pbar.pos() == 600);

        app.pbar.advance_delta(50);
        assert!(app.pbar.pos() == 650);

        app.runs.borrow_mut().progress_bar = true;
    } else {
        app.runs.borrow_mut().progress_bar = false;
    }
}

#[cfg(not(feature = "progress-bar"))]
fn dispatch_progress_bar_tests(_handle: &ControlHandle, _app: &TestApp) {
}

fn test_events(app: &TestApp, e: Event) {
    app.events_show.set_text(&format!("{:?}", e));
}

fn test_partial(_app: &TestApp, _e: Event) {
    simple_message("HELLO!", "Hello from partial");
}

fn close(_app: &TestApp, _e: Event) {
    stop_thread_dispatch();
}

#[test]
fn test_everything() {
    enable_visual_styles();
    init_common_controls();
    
    let app = TestApp::build_ui(Default::default()).expect("Failed to build UI");

    app.window.set_focus();

    dispatch_thread_events();
}
