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

    #[nwg_control(font: Some(&data.font), focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    rich_text_box: nwg::RichTextBox
}

impl RichText {

    fn init_text(&self) {
        let text = concat!(
            "HELLO\r\n",
            "WORLD!\r\n"
        );

        let rich = &self.rich_text_box;
        rich.set_text(text);

        rich.set_selection(0..5);
        rich.set_char_format(&nwg::CharFormat {
            height: Some(500),
            text_color: Some([255, 0, 0]),
            underline_type: Some(nwg::UnderlineType::Solid),
            ..Default::default()
        });

        rich.set_selection(6..12);
        rich.set_char_format(&nwg::CharFormat {
            effets: Some(nwg::CharEffets::ITALIC | nwg::CharEffets::BOLD),
            height: Some(500),
            text_color: Some([0, 0, 255]),
            font_face_name: Some("Comic Sans MS".to_string()),
            ..Default::default()
        });
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
