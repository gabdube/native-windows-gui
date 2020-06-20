/*!
    Small example that shows how to accept file drop in an application

    Requires the following features: `cargo run --example drop_files_d --features "textbox"`
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

use std::fs;


#[derive(Default, NwgUi)]
pub struct DropApp {
    #[nwg_control(size: (360, 360), position: (300, 300), title: "Drag & Drop", accept_files: true)]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnFileDrop: [DropApp::load_text(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: grid)]
    text: nwg::TextBox,
}

impl DropApp {
    pub fn load_text(&self, data: &nwg::EventData) {
        let drop = data.on_file_drop();

        let mut text = String::with_capacity(1000);

        for file in drop.files() {
            text.push_str(&fs::read_to_string(file).unwrap_or("Invalid file".into()));
        }

        self.text.set_text(&text);
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Comic Sans MS").expect("Failed to set default font");

    let _app = DropApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
