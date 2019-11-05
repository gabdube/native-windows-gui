use crate::*;
use std::cell::RefCell;

#[derive(Default)]
#[allow(dead_code)]
pub struct TestRun {
    window: bool,
    button: bool,
    check: bool,
    combo: bool,
    date: bool,
}


#[derive(Default)]
pub struct ControlsTest {
    // data
    pub runs: RefCell<TestRun>,

    // Control window
    pub window: Window,
    pub window_icon: Image,
    pub controls_holder: TabsContainer,
    pub basics_control_tab: Tab,
    pub test_button: Button,
    pub test_checkbox1: CheckBox,
    pub test_checkbox2: CheckBox,
    pub test_combo: ComboBox<&'static str>,
    pub test_date: DatePicker,

    // Control panel
    pub panel: Window,
    pub run_window_test: Button,
    pub run_button_test: Button,
    pub run_check_box_test: Button,
    pub run_combo_test: Button,
    pub run_date_test: Button
}

mod partial_controls_test_ui {
    use super::*;
    use crate::{PartialUi, SystemError, ControlHandle};

    impl PartialUi<ControlsTest> for ControlsTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut ControlsTest, _parent: Option<W>) -> Result<(), SystemError> {
            
            //
            // Resources
            //
            data.window_icon = Image::icon("./test_rc/cog.ico", None, false)?;

            //
            //  Controls holder
            //

            Window::builder()
                .flags(WindowFlags::MAIN_WINDOW)
                .size((500, 370))
                .position((100, 100))
                .title("Controls")
                .icon(Some(&data.window_icon))
                .build(&mut data.window)?;

            TabsContainer::builder()
                .parent(&data.window)
                .build(&mut data.controls_holder)?;

            Tab::builder()
                .text("Basic")
                .parent(&data.controls_holder)
                .build(&mut data.basics_control_tab)?;

            Button::builder()
                .text("A simple button")
                .position((10, 10))
                .size((130, 30))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_button)?;

            CheckBox::builder()
                .text("I like bacon")
                .position((10, 50))
                .size((130, 30))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_checkbox1)?;

            CheckBox::builder()
                .flags(CheckBoxFlags::VISIBLE | CheckBoxFlags::TRISTATE)
                .text("Tree state")
                .position((10, 80))
                .size((130, 30))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_checkbox2)?;

            ComboBox::builder()
                .position((10, 120))
                .size((130, 30))
                .collection(vec!["Chocolate", "Strawberry", "Blueberry"])
                .selected_index(Some(0))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_combo)?;

            DatePicker::builder()
                .position((10, 160))
                .size((130, 30))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_date)?;

            //
            // Run tests
            //

            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((300, 360))
                .position((650, 100))
                .title("Action panel")
                .icon(Some(&data.window_icon))
                .build(&mut data.panel)?;
            
            Button::builder()
                .text("Run window test")
                .parent(&data.panel)
                .build(&mut data.run_window_test)?;

            Button::builder()
                .text("Run button test")
                .parent(&data.panel)
                .build(&mut data.run_button_test)?;

            Button::builder()
                .text("Run checkbox test")
                .parent(&data.panel)
                .build(&mut data.run_check_box_test)?;

            Button::builder()
                .text("Run combo test")
                .parent(&data.panel)
                .build(&mut data.run_combo_test)?;

            Button::builder()
                .text("Run date test")
                .parent(&data.panel)
                .build(&mut data.run_date_test)?;

            // Layout
            VBoxLayout::builder()
                .parent(&data.window)
                .child(0, &data.controls_holder)
                .build();

            GridLayout::builder()
                .parent(&data.panel)
                .child(0, 0, &data.run_window_test)
                .child(1, 0, &data.run_button_test)
                .child(0, 1, &data.run_check_box_test)
                .child(1, 1, &data.run_combo_test)
                .child(0, 2, &data.run_date_test)
                .max_row(Some(8))
                .build();
            
            Ok(())
        }

        fn process_event(&self, evt: Event, handle: ControlHandle) {
            use crate::Event as E;

            match evt {
                E::OnButtonClick =>
                    if handle == self.run_window_test.handle {
                        run_window_tests(self, evt);
                    } else if handle == self.run_button_test.handle {
                        run_button_tests(self, evt);
                    } else if handle == self.run_check_box_test.handle {
                        run_check_box_tests(self, evt);
                    } else if handle == self.run_combo_test.handle {
                        run_combo_tests(self, evt);
                    },
                _ => {}
            }
        }


        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle, &self.panel.handle]
        }

    }
}

fn run_window_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().window {

        assert_eq!(&app.window.text(), "Controls");
        app.window.set_text("Controls New title");
        assert_eq!(&app.window.text(), "Controls New title");

        assert_eq!(app.window.visible(), true);
        app.window.set_visible(false);
        assert_eq!(app.window.visible(), false);
        app.window.set_visible(true);

        assert_eq!(app.window.enabled(), true);
        app.window.set_enabled(false);
        assert_eq!(app.window.enabled(), false);
        app.window.set_enabled(true);

        app.window.set_position(100, 100);
        assert_eq!(app.window.position(), (100, 100));
        
        app.window.set_size(500, 370);
        assert_eq!(app.window.size(), (500, 370));

        app.runs.borrow_mut().window = true;
    } else {
        app.window.set_text("Controls");
        app.runs.borrow_mut().window = false;
    }
}

fn run_button_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().button {
        assert_eq!(&app.test_button.text(), "A simple button");
        app.test_button.set_text("New Text");
        assert_eq!(&app.test_button.text(), "New Text");

        assert_eq!(app.test_button.position(), (10, 10));
        app.test_button.set_position(5, 5);
        assert_eq!(app.test_button.position(), (5, 5));

        assert_eq!(app.test_button.size(), (130, 30));
        app.test_button.set_size(120, 35);
        assert_eq!(app.test_button.size(), (120, 35));

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
        app.test_button.set_text("A simple button");
        app.test_button.set_position(10, 10);
        app.test_button.set_size(130, 30);
        app.test_button.set_enabled(true);
        app.runs.borrow_mut().button = false;
    }
}

fn run_check_box_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().check {

        assert_eq!(app.test_checkbox2.tristate(), true);
        assert_eq!(app.test_checkbox1.tristate(), false);

        app.test_checkbox1.set_tristate(true);
        assert_eq!(app.test_checkbox1.tristate(), true);

        app.test_checkbox1.set_check_state(CheckBoxState::Checked);
        assert_eq!(app.test_checkbox1.check_state(), CheckBoxState::Checked);

        app.test_checkbox1.set_check_state(CheckBoxState::Unchecked);
        assert_eq!(app.test_checkbox1.check_state(), CheckBoxState::Unchecked);

        app.test_checkbox1.set_check_state(CheckBoxState::Indeterminate);
        assert_eq!(app.test_checkbox1.check_state(), CheckBoxState::Indeterminate);

        app.runs.borrow_mut().check = true;
    } else {
        app.test_checkbox1.set_tristate(false);
        app.runs.borrow_mut().check = false;
    }
}

fn run_combo_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().combo {
        {
            let col = app.test_combo.collection();
            assert_eq!(&col as &[&'static str], &["Chocolate", "Strawberry", "Blueberry"]);
        }

        {
            let mut col = app.test_combo.collection_mut();
            col.push("Hello");
        }

        app.test_combo.sync();
        app.test_combo.push("World!");
        assert_eq!(app.test_combo.len(), 5);

        app.test_combo.set_selection(None);
        assert_eq!(app.test_combo.selection(), None);
        assert_eq!(app.test_combo.selection_string(), None);

        app.test_combo.set_selection(Some(2));
        assert_eq!(app.test_combo.selection(), Some(2));
        assert_eq!(app.test_combo.selection_string(), Some("Blueberry".to_string()));

        assert_eq!(app.test_combo.set_selection_string("hel"), Some(3));
        assert_eq!(app.test_combo.selection(), Some(3));
        assert_eq!(app.test_combo.selection_string(), Some("Hello".to_string()));

        app.test_combo.sort();
        assert_eq!(app.test_combo.set_selection_string("Blue"), Some(0));

        app.test_combo.insert(1, "BOO!");
        app.test_combo.insert(std::usize::MAX, "Ahoy!!");
        assert_eq!(app.test_combo.set_selection_string("BOO!"), Some(1));
        assert_eq!(app.test_combo.set_selection_string("Ahoy!!"), Some(6));

        app.test_combo.remove(0);

        app.test_combo.dropdown(true);

        app.runs.borrow_mut().combo = true;
    } else {
        app.test_combo.set_collection(vec!["Chocolate", "Strawberry", "Blueberry"]);
        app.runs.borrow_mut().combo = false;
    }
}
