use crate::*;

#[derive(Default)]
pub struct CanvasTest {
    pub window: CanvasWindow,
}

mod partial_canvas_test_ui {
    use super::*;
    use crate::{PartialUi, SystemError, ControlHandle};

    impl PartialUi<CanvasTest> for CanvasTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut CanvasTest, _parent: Option<W>) -> Result<(), SystemError> {
            CanvasWindow::builder()
                .flags(CanvasWindowFlags::POPUP)
                .size((300, 300))
                .position((250, 100))
                .title("Canvas")
                .build(&mut data.window)?;

            Ok(())
        }

        fn process_event<'a>(&self, _evt: Event, mut _evt_data: EventData<'a>, _handle: ControlHandle) {
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle]
        }

    }

}
