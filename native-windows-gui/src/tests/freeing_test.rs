use crate::*;
use std::cell::RefCell;


#[derive(Default)]
struct FreeingData {
    raw_handler_bound: bool,
}

#[derive(Default)]
pub struct FreeingTest {
    data: RefCell<FreeingData>,
    pub window: Window,
    layout: GridLayout,
    bind_handler_btn: Button,
    custom_bind_button: Button,
}

impl FreeingTest {

    fn bind_raw_handler(&self) {

    }

}

mod partial_freeing_test_ui {
    use super::*;
    use crate::{PartialUi, NwgError, ControlHandle};

    impl PartialUi<FreeingTest> for FreeingTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut FreeingTest, _parent: Option<W>) -> Result<(), NwgError> {
            
            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((300, 300))
                .position((450, 100))
                .title("Freeing stuff")
                .build(&mut data.window)?;

            Button::builder()
                .text("Bind raw handler")
                .parent(&data.window)
                .build(&mut data.bind_handler_btn)?;

            Button::builder()
                .text("Do something!")
                .parent(&data.window)
                .build(&mut data.custom_bind_button)?;

            GridLayout::builder()
                .parent(&data.window)
                .max_column(Some(2))
                .max_row(Some(3))
                .child(0, 0, &data.bind_handler_btn)
                .child(1, 0, &data.custom_bind_button)
                .build(&data.layout);

            Ok(())
        }

        fn process_event<'a>(&self, evt: Event, mut _evt_data: &EventData, handle: ControlHandle) {
            use crate::Event as E;

            match evt {
                E::OnButtonClick => 
                    if &handle == &self.bind_handler_btn {
                        FreeingTest::bind_raw_handler(self)
                    }
                _ => {}
            }
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle]
        }

    }
}
