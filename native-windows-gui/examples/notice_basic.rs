/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;


#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 300), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnInit: [BasicApp::setup], OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(size: (280, 280), position: (10, 10), focus: true, flags: "VISIBLE|AUTOVSCROLL")]
    text_box: nwg::TextBox,

    #[nwg_control]
    #[nwg_events(OnNotice: [BasicApp::update_text])]
    update_text: nwg::Notice,

    text_receiver: RefCell<Option<Receiver<String>>>,
}

impl BasicApp {

  fn setup(&self) {
    let (sender, receiver) = channel();

    // Creates a sender to trigger the `OnNotice` event
    let notice_sender = self.update_text.sender();

    thread::spawn(move || {
      let mut counter = 0;

      loop {
        counter += 1;
        sender.send(format!("ID: {}\r\n", counter)).unwrap();

        // Calling the notice function will trigger the OnNotice event on the gui thread
        notice_sender.notice();

        thread::sleep(::std::time::Duration::from_millis(500));
      }
    });

    *self.text_receiver.borrow_mut() = Some(receiver);
  }

  fn update_text(&self) {
    let mut receiver_ref = self.text_receiver.borrow_mut();
    let receiver = receiver_ref.as_mut().unwrap();
    while let Ok(data) = receiver.try_recv() {
      let mut new_text = self.text_box.text();
      new_text.push_str(&data);
      self.text_box.set_text(&new_text);
      self.text_box.scroll_lastline();
    }
  }

  fn say_goodbye(&self) {
      nwg::stop_thread_dispatch();
  }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
