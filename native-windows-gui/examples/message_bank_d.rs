/*!
    An application that saves messages into buttons. 
    Demonstrate the dynamic functions of NWG.

    `cargo run --example message_bank_d`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;


#[derive(Default, NwgUi)]
pub struct MessageBank {
    
    #[nwg_control(size:(400, 300), position:(800, 300), title: "My message bank")]
    #[nwg_events( OnWindowClose: [MessageBank::exit] )]
    window: nwg::Window,
    
    #[nwg_layout(parent: window, max_row: Some(6), spacing: 3)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "Save", focus: true)]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    #[nwg_events( OnButtonClick: [MessageBank::add_message] )]
    add_message_btn: nwg::Button,

    #[nwg_control(text:"Title")]
    #[nwg_layout_item(layout: layout, col: 1, row: 0, col_span: 2)]
    message_title: nwg::TextInput,

    #[nwg_control(text:"Hello World!")]
    #[nwg_layout_item(layout: layout, col: 3, row: 0, col_span: 3)]
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

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _ui = MessageBank::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}


