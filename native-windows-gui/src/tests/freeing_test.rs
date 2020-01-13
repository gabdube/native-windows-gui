use crate::*;
use winapi::um::winuser::WM_LBUTTONUP;
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Default)]
struct FreeingData {
    raw_handler_bound: bool,
    raw_callback_id: usize,
    raw_handler: Option<RawEventHandler>,
    rc: Rc<()>
}

#[derive(Default)]
pub struct FreeingTest {
    data: RefCell<FreeingData>,
    pub window: Window,
    layout: GridLayout,
    bind_handler_btn: Button,
    custom_bind_button: Button,

    bind_handler_btn2: Button,
    custom_bind_button2: Button,
}

impl FreeingTest {

    fn bind_raw_handler(&self) {
        let mut data = self.data.borrow_mut();
        if data.raw_handler_bound {
            self.bind_handler_btn.set_text("Bind raw handler");
            data.raw_handler_bound = false;
            unbind_raw_event_handler(&data.raw_handler.take().unwrap());
        } else {
            self.bind_handler_btn.set_text("Unbind raw handler");
            data.raw_handler_bound = true;
            data.raw_callback_id += 1;

            let send_rc = data.rc.clone();
            let handler = bind_raw_event_handler(&self.custom_bind_button.handle, data.raw_callback_id, move |_hwnd, msg, _w, _l| {
                if msg == WM_LBUTTONUP {
                    simple_message("Raw handler", &format!("Button WM_LBUTTONUP hook. Rc has {:?} strong count", Rc::strong_count(&send_rc)) );
                }
                None
            });

            data.raw_handler = Some(handler);
        }
    }

}

mod partial_freeing_test_ui {
    use super::*;
    use crate::{PartialUi, NwgError, ControlHandle};

    impl PartialUi<FreeingTest> for FreeingTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut FreeingTest, _parent: Option<W>) -> Result<(), NwgError> {
            
            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((400, 150))
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

            Button::builder()
                .text("Bind handler")
                .parent(&data.window)
                .build(&mut data.bind_handler_btn2)?;

            Button::builder()
                .text("Do something else!")
                .parent(&data.window)
                .build(&mut data.custom_bind_button2)?;

            GridLayout::builder()
                .parent(&data.window)
                .max_column(Some(2))
                .max_row(Some(2))
                .child(0, 0, &data.bind_handler_btn)
                .child(1, 0, &data.custom_bind_button)
                .child(0, 1, &data.bind_handler_btn2)
                .child(1, 1, &data.custom_bind_button2)
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
