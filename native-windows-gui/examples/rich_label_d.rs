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
    #[nwg_control(size: (590, 430), position: (300, 300), title: "Rich TextBox")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [RichText::init_text], OnMinMaxInfo: [RichText::set_resize(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    font: nwg::Font,

    #[nwg_control(parent: window, popup: true)]
    text_menu: nwg::Menu,

    #[nwg_control(parent: text_menu, text: "Copy")]
    #[nwg_events(OnMenuItemSelected: [RichText::copy_text])]
    copy_text_item: nwg::MenuItem,

    #[nwg_control(font: Some(&data.font), flags: "VISIBLE|MULTI_LINE")]
    #[nwg_events(MousePressRightUp: [RichText::show_menu])]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    rich_text_box: nwg::RichLabel
}

impl RichText {

    fn init_text(&self) {
        let text = concat!(
            "Russian political jokes\r\n",  //0..24  //0..24

            // 25..187
            "Russian political jokes are a part of Russian humour and can be grouped into the major time periods: Imperial Russia, Soviet Union and finally post-Soviet Russia.\r\n",
            
            "Imperial Russia\r\n", // 187..203

            // 203..411
            "In Imperial Russia, most political jokes were of the polite variety that circulated in educated society. Few of the political jokes of the time are recorded, but some were printed in a 1904 German anthology.\r\n",

            // 411..660
            "A man was reported to have said: \"Nikolay is a moron!\" and was arrested by a policeman. \"No, sir, I meant not our respected Emperor, but another Nikolay!\" - \"Don't try to trick me: if you say \"moron\", you are obviously referring to our tsar!\r\n",

            // 660..861
            "A respected merchant, Sevenassov (Semizhopov in the original Russian), wants to change his surname, and asks the Tsar for permission. The Tsar gives his decision in writing: \"Permitted to subtract two asses\"\r\n",

            // 861..873
            "Soviet Union\r\n", 

            // 873..1106
            "Every nation enjoys political jokes, but in the Soviet Union telling political jokes could be regarded as type of extreme sport: according to Article 58 (RSFSR Penal Code), \"anti-Soviet propaganda\" was a potentially capital offense.\r\n",
        );

        let rich = &self.rich_text_box;
        rich.set_text(text);

        rich.set_char_format(0..24, &nwg::CharFormat {
            height: Some(500),
            text_color: Some([50, 50, 150]),
            font_face_name: Some("Comic Sans MS".to_string()),
            ..Default::default()
        });

        rich.set_para_format(0..24,&nwg::ParaFormat {
            alignment: Some(nwg::ParaAlignment::Center),
            ..Default::default()
        });

        rich.set_para_format(25..187, &nwg::ParaFormat {
            space_before: Some(200),
            space_after: Some(200),
            ..Default::default()
        });

        rich.set_char_format(187..203, &nwg::CharFormat {
            effects: Some(nwg::CharEffects::BOLD),
            height: Some(350),
            text_color: Some([50, 50, 50]),
            ..Default::default()
        });

        rich.set_para_format(187..203, &nwg::ParaFormat {
            alignment: Some(nwg::ParaAlignment::Center),
            ..Default::default()
        });

        rich.set_para_format(203..411, &nwg::ParaFormat {
            space_before: Some(200),
            space_after: Some(200),
            ..Default::default()
        });

        rich.set_para_format(411..412, &nwg::ParaFormat {
            space_after: Some(100),
            ..Default::default()
        });

        rich.set_para_format(411..861, &nwg::ParaFormat {
            start_indent: Some(300),
            right_indent: Some(300),
            line_spacing: Some(nwg::ParaLineSpacing::Single),
            numbering: Some(nwg::ParaNumbering::Seq('1')),
            numbering_style: Some(nwg::ParaNumberingStyle::Period),
            numbering_tab: Some(300),
            ..Default::default()
        });

        rich.set_char_format(861..873, &nwg::CharFormat {
            effects: Some(nwg::CharEffects::BOLD | nwg::CharEffects::ITALIC),
            height: Some(350),
            text_color: Some([150, 50, 50]),
            ..Default::default()
        });

        rich.set_para_format(861..873, &nwg::ParaFormat {
            alignment: Some(nwg::ParaAlignment::Center),
            ..Default::default()
        });

        rich.set_para_format(873..1106, &nwg::ParaFormat {
            space_before: Some(200),
            space_after: Some(200),
            ..Default::default()
        });
    }

    fn set_resize(&self, data: &nwg::EventData) {
        let data = data.on_min_max();
        data.set_min_size(200, 200);
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.text_menu.popup(x, y);
    }

    fn copy_text(&self) {
        let current_selection = self.rich_text_box.selection();
        let current_text = self.rich_text_box.text();
        let selected_text = &current_text[current_selection];
        nwg::Clipboard::set_data_text(&self.window, &selected_text);
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = RichText::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
