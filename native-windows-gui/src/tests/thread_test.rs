use crate::*;
use std::cell::RefCell;

#[derive(Default)]
pub struct ThreadResources {

}

#[derive(Default)]
pub struct ThreadTest {
    resources: RefCell<ThreadResources>,
    pub window: Window,
}

mod partial_canvas_test_ui {
    use super::*;
    use crate::{PartialUi, SystemError, ControlHandle};

    impl PartialUi<ThreadTest> for ThreadTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut ThreadTest, _parent: Option<W>) -> Result<(), SystemError> {
            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((300, 300))
                .position((250, 100))
                .title("Threads")
                .build(&mut data.window)?;

            Ok(())
        }

        fn process_event<'a>(&self, evt: Event, mut _evt_data: &EventData, _handle: ControlHandle) {
            use crate::Event as E;

            match evt {
                _ => {}
            }
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle]
        }

    }

}
