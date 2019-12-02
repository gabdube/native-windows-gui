/*!
    A very simple application that show your name in a message box.
    See `basic_d` for the derive version
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct MessageBank {
    window: nwg::Window,
    layout: nwg::GridLayout,
    add_message_btn: nwg::Button,
    message_title: nwg::TextBox,
    message_content: nwg::TextBox,
}

impl MessageBank {

    fn add_message(&self) {
        let title = self.message_title.text();
        let content = self.message_content.text();
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

//
// ALL of this stuff is handled by native-windows-derive
//
mod message_bank_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct MessageBankUi {
        inner: MessageBank
    }

    impl nwg::NativeUi<MessageBank, MessageBankUi> for MessageBank {
        fn build_ui(mut data: MessageBank) -> Result<Rc<MessageBankUi>, nwg::SystemError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((400, 300))
                .position((800, 300))
                .title("My message bank")
                .build(&mut data.window)?;

            nwg::Button::builder()
                .text("Save")
                .parent(&data.window)
                .build(&mut data.add_message_btn)?;

            nwg::TextBox::builder()
                .text("Message Title")
                .parent(&data.window)
                .build(&mut data.message_title)?;

            nwg::TextBox::builder()
                .text("Message Content")
                .parent(&data.window)
                .build(&mut data.message_content)?;

            // Wrap-up
            let ui = Rc::new(MessageBankUi { inner: data });

            // Events
            let window_handles = [&ui.window.handle];

            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, _evt_data, handle| {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &evt_ui.add_message_btn { MessageBank::add_message(&evt_ui.inner); }
                        },
                        E::OnWindowClose => {
                            if &handle == &evt_ui.window { MessageBank::exit(&evt_ui.inner); }
                        },
                        _ => {}
                    }
                };

                nwg::bind_event_handler(handle, handle_events);
            }

            // Layout
            nwg::GridLayout::builder()
              .parent(&ui.window)
              .max_row(Some(6))
              .child(0, 0, &ui.add_message_btn)
              .child_item(nwg::GridLayoutItem::new(&ui.message_title, 1, 0, 2, 1))
              .child_item(nwg::GridLayoutItem::new(&ui.message_content, 3, 0, 3, 1))
              .build(&ui.layout);
            
            return Ok(ui);
        }
    }


    impl Deref for MessageBankUi {
        type Target = MessageBank;

        fn deref(&self) -> &MessageBank {
            &self.inner
        }
    }

}



fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _ui = MessageBank::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
