/*!
    Small example that shows how to accept file drop in an application

    Requires the following features: `cargo run --example drop_files_d --features "textbox"`
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct RichText {
    #[nwg_control(size: (360, 360), position: (300, 300), title: "Rich TextBox")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [RichText::init_text], OnMinMaxInfo: [RichText::set_resize(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    font: nwg::Font,

    #[nwg_control(font: Some(&data.font))]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    rich_text_box: nwg::RichTextBox
}

impl RichText {

    fn init_text(&self) {
        let text = concat!(
            "HELLO\r\n",
            "WORLD!"
        );

        self.rich_text_box.set_text(text);
    }

    fn set_resize(&self, data: &nwg::EventData) {
        let data = data.on_min_max();
        data.set_min_size(200, 200);
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = RichText::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
