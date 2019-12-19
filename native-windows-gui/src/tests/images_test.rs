use crate::*;

#[derive(Default)]
pub struct ImagesTest {
    pub window: Window,
    jpeg_test: Jpeg,
}

mod partial_image_test_ui {
    use super::*;
    use crate::{PartialUi, NwgError, ControlHandle};

    impl PartialUi<ImagesTest> for ImagesTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut ImagesTest, _parent: Option<W>) -> Result<(), NwgError> {
            
            Jpeg::builder()
              .source_file(Some("./test_rc/cat.jpg"))
              .build(&mut data.jpeg_test)?;
            
            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((300, 300))
                .position((450, 100))
                .title("Images")
                .build(&mut data.window)?;

            Ok(())
        }

        fn process_event<'a>(&self, _evt: Event, mut _evt_data: &EventData, _handle: ControlHandle) {
            use crate::Event as E;
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle]
        }

    }
}
