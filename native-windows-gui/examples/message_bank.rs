/*!
    An application that saves messages into buttons. 
    Demonstrate the dynamic functions of NWG.
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;
use std::cell::RefCell;


#[derive(Default)]
pub struct MessageBank {
    window: nwg::Window,
    layout: nwg::GridLayout,

    add_message_btn: nwg::Button,
    message_title: nwg::TextInput,
    message_content: nwg::TextInput,

    buttons: RefCell<Vec<nwg::Button>>,
    handlers: RefCell<Vec<nwg::EventHandler>>,
}

impl MessageBank {

    fn add_message(&self) {
        let title = self.message_title.text();
        let content = self.message_content.text();

        let mut new_button = Default::default();
        nwg::Button::builder()
            .text(&title)
            .parent(&self.window)
            .build(&mut new_button)
            .expect("Failed to build button");

        let mut buttons = self.buttons.borrow_mut();
        let mut handlers = self.handlers.borrow_mut();

        let blen = buttons.len() as u32;
        let (x, y) = (blen % 6, blen / 6);
        self.layout.add_child(x, y+1, &new_button);

        // You can share controls handle with events handlers
        let new_button_handle = new_button.handle;
        let handler = nwg::bind_event_handler(&new_button.handle, &self.window.handle, move |evt, _evt_data, handle| {
            match evt {
                nwg::Event::OnButtonClick => {
                    if handle == new_button_handle {
                        nwg::simple_message(&title, &content);
                    }
                },
                _ => {}
            }
        });

        buttons.push(new_button);
        handlers.push(handler);
    }

    fn exit(&self) {
        let handlers = self.handlers.borrow();
        for handler in handlers.iter() {
            nwg::unbind_event_handler(&handler);
        }

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
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct MessageBankUi {
        inner: Rc<MessageBank>,
        default_handler: RefCell<Vec<nwg::EventHandler>>
    }

    impl nwg::NativeUi<MessageBankUi> for MessageBank {
        fn build_ui(mut data: MessageBank) -> Result<MessageBankUi, nwg::NwgError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::MAIN_WINDOW | nwg::WindowFlags::VISIBLE)
                .size((400, 300))
                .position((800, 300))
                .title("My message bank")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .text("Hello World!")
                .focus(true)
                .parent(&data.window)
                .build(&mut data.message_content)?;

            nwg::Button::builder()
                .text("Save")
                .parent(&data.window)
                .build(&mut data.add_message_btn)?;

            nwg::TextInput::builder()
                .text("Title")
                .parent(&data.window)
                .build(&mut data.message_title)?;

            // Wrap-up
            let ui = MessageBankUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let window_handles = [&ui.window.handle];

            for handle in window_handles.iter() {
                let evt_ui = Rc::downgrade(&ui.inner);
                let handle_events = move |evt, _evt_data, handle| {
                    if let Some(evt_ui) = evt_ui.upgrade() {
                        match evt {
                            E::OnButtonClick => {
                                if &handle == &evt_ui.add_message_btn { MessageBank::add_message(&evt_ui); }
                            },
                            E::OnWindowClose => {
                                if &handle == &evt_ui.window { MessageBank::exit(&evt_ui); }
                            },
                            _ => {}
                        }
                    }
                };

                ui.default_handler.borrow_mut().push(
                    nwg::full_bind_event_handler(handle, handle_events)
                );
            }

            // Layout
            nwg::GridLayout::builder()
              .parent(&ui.window)
              .max_row(Some(6))
              .child(0, 0, &ui.add_message_btn)
              .child_item(nwg::GridLayoutItem::new(&ui.message_title, 1, 0, 2, 1))
              .child_item(nwg::GridLayoutItem::new(&ui.message_content, 3, 0, 3, 1))
              .build(&ui.layout)?;
            
            return Ok(ui);
        }
    }

    impl Drop for MessageBankUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
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
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _ui = MessageBank::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
