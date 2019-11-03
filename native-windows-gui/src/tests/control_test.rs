use crate::*;

#[derive(Default)]
pub struct ControlsTest {
    pub window: Window,
}

#[derive(Default)]
pub struct ControlsTestPanel {
    pub window: Window,
}

mod partial_controls_test_ui {
    use super::*;
    use crate::{PartialUi, SystemError, ControlHandle};

    impl PartialUi<ControlsTest> for ControlsTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut ControlsTest, _parent: Option<W>) -> Result<(), SystemError> {
            Window::builder()
                .flags(WindowFlags::MAIN_WINDOW)
                .size((500, 370))
                .position((100, 100))
                .title("Controls")
                .build(&mut data.window)?;
            
            Ok(())
        }
    }
}

mod partial_controls_test_panel_ui {
    use super::*;
    use crate::{PartialUi, SystemError, ControlHandle};

    impl PartialUi<ControlsTestPanel> for ControlsTestPanel {

        fn build_partial<W: Into<ControlHandle>>(data: &mut ControlsTestPanel, _parent: Option<W>) -> Result<(), SystemError> {
            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((280, 360))
                .position((650, 100))
                .title("Action panel")
                .build(&mut data.window)?;
            
            Ok(())
        }
    }
}
