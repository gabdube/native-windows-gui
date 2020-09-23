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
    #[nwg_control(size: (450, 430), position: (300, 300), title: "Rich TextBox")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [RichText::init_text], OnMinMaxInfo: [RichText::set_resize(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    font: nwg::Font,

    #[nwg_control(font: Some(&data.font), focus: true, flags: "VSCROLL|AUTOVSCROLL|VISIBLE|TAB_STOP|SAVE_SELECTION")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    rich_text_box: nwg::RichTextBox
}

impl RichText {

    fn init_text(&self) {
        let text = concat!(
            "Russian political jokes\r\n",  //0..24

            // 25..188
            "Russian political jokes are a part of Russian humour and can be grouped into the major time periods: Imperial Russia, Soviet Union and finally post-Soviet Russia.\r\n",
            
            "Imperial Russia\r\n", // 187..203

            // 203..411
            "In Imperial Russia, most political jokes were of the polite variety that circulated in educated society. Few of the political jokes of the time are recorded, but some were printed in a 1904 German anthology.\r\n",

            "Soviet Union\r\n",  // 411..423

            // 423..658
            "Every nation enjoys political jokes, but in the Soviet Union telling political jokes could be regarded as type of extreme sport: according to Article 58 (RSFSR Penal Code), \"anti-Soviet propaganda\" was a potentially capital offense.\r\n",
        );

        let rich = &self.rich_text_box;
        rich.set_text(text);

        rich.set_selection(0..24);
        rich.set_char_format(&nwg::CharFormat {
            effects: Some(nwg::CharEffects::BOLD),
            height: Some(500),
            text_color: Some([50, 50, 150]),
            font_face_name: Some("Comic Sans MS".to_string()),
            y_offset: Some(150),
            ..Default::default()
        });

        rich.set_selection(187..203);
        rich.set_char_format(&nwg::CharFormat {
            effects: Some(nwg::CharEffects::BOLD),
            height: Some(350),
            text_color: Some([50, 50, 50]),
            y_offset: Some(100),
            ..Default::default()
        });

        rich.set_selection(411..423);
        rich.set_char_format(&nwg::CharFormat {
            effects: Some(nwg::CharEffects::BOLD | nwg::CharEffects::ITALIC),
            height: Some(350),
            text_color: Some([150, 50, 50]),
            y_offset: Some(100),
            ..Default::default()
        });

        rich.set_selection(5000..5001);
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
