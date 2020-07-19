/*!
    An example that shows how multithreading works in NWG. Upon clicking on "Open Dialog" button, the application starts
    a new GUI thread. Once the user selects a value in the new dialog, the dialog notice the main thread that it should read the data.

    Requires the following features: `cargo run --example dialog_multithreading_d --features "notice"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::{thread, cell::RefCell};


/// The dialog UI
#[derive(Default, NwgUi)]
pub struct ThreadingDialog {
    data: RefCell<String>,

    #[nwg_control(size: (300, 115), position: (650, 300), title: "A dialog", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [ThreadingDialog::close] )]
    window: nwg::Window,

    #[nwg_control(text: "YES", position: (10, 10), size: (130, 95))]
    #[nwg_events( OnButtonClick: [ThreadingDialog::choose(SELF, CTRL)] )]
    choice_yes: nwg::Button,

    #[nwg_control(text: "NO", position: (160, 10), size: (130, 95), focus: true)]
    #[nwg_events( OnButtonClick: [ThreadingDialog::choose(SELF, CTRL)] )]
    choice_no: nwg::Button,
}

impl ThreadingDialog {

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn choose(&self, btn: &nwg::Button) {
        let mut data = self.data.borrow_mut();
        if btn == &self.choice_no {
            *data = "No!".to_string();
        } else if btn == &self.choice_yes {
            *data = "Yes!".to_string();
        }

        self.window.close();
    }

}


/// The Main UI
#[derive(Default, NwgUi)]
pub struct ThreadingApp {
    dialog_data: RefCell<Option<thread::JoinHandle<String>>>,

    #[nwg_control(size: (300, 115), position: (300, 300), title: "Multithreading example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [ThreadingApp::exit] )]
    window: nwg::Window,

    #[nwg_control]
    #[nwg_events( OnNotice: [ThreadingApp::read_dialog_output] )]
    dialog_notice: nwg::Notice,

    #[nwg_control(size: (280, 25), position: (10, 10), readonly: true)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Open Dialog", size: (280, 60), position: (10, 40), focus: true)]
    #[nwg_events( OnButtonClick: [ThreadingApp::open_dialog] )]
    button: nwg::Button,
}

impl ThreadingApp {

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn open_dialog(&self) {
        let mut data = self.dialog_data.borrow_mut();
        if data.is_some() {
            nwg::error_message("Error", "The dialog is already running!");
            return;
        }

        let notice = self.dialog_notice.sender();

        *data = Some(thread::spawn(move || {
            let app = ThreadingDialog::build_ui(Default::default()).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            
            notice.notice();
            { 
                let data = app.data.borrow();
                data.clone()
            }
        }))
    }

    fn read_dialog_output(&self) {
        let mut data = self.dialog_data.borrow_mut();
        match data.take() {
            Some(handle) => {
                self.name_edit.set_text(&handle.join().unwrap());
                self.button.set_focus();
            },
            None => {}
        }
    }

}

fn main() {
    // nwg::init can be done on any thread.
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = ThreadingApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();

}
