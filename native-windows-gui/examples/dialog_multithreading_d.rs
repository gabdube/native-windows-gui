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
pub struct YesNoDialog {
    data: RefCell<Option<String>>,

    #[nwg_control(size: (300, 115), position: (650, 300), title: "A dialog", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [YesNoDialog::close] )]
    window: nwg::Window,

    #[nwg_control(text: "YES", position: (10, 10), size: (130, 95))]
    #[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
    choice_yes: nwg::Button,

    #[nwg_control(text: "NO", position: (160, 10), size: (130, 95), focus: true)]
    #[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
    choice_no: nwg::Button,
}

impl YesNoDialog {

    /// Create the dialog UI on a new thread. The dialog result will be returned by the thread handle.
    /// To alert the main GUI that the dialog completed, this function takes a notice sender object.
    fn popup(sender: nwg::NoticeSender) -> thread::JoinHandle<String> {
        thread::spawn(move || {
            // Create the UI just like in the main function
            let app = YesNoDialog::build_ui(Default::default()).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            
            // Notice the main thread that the dialog completed
            sender.notice();

            // Return the dialog data
            app.data.take().unwrap_or("Cancelled!".to_owned())
        })
    }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn choose(&self, btn: &nwg::Button) {
        let mut data = self.data.borrow_mut();
        if btn == &self.choice_no {
            *data = Some("No!".to_string());
        } else if btn == &self.choice_yes {
            *data = Some("Yes!".to_string());
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
        // Disable the button to stop the user from spawning multiple dialogs
        self.button.set_enabled(false);

        *self.dialog_data.borrow_mut() = Some(YesNoDialog::popup(self.dialog_notice.sender()));
    }

    fn read_dialog_output(&self) {
        self.button.set_enabled(true);

        let data = self.dialog_data.borrow_mut().take();
        match data {
            Some(handle) => {
                let dialog_result = handle.join().unwrap();
                self.name_edit.set_text(&dialog_result);
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
