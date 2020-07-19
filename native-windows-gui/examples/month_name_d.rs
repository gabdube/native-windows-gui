/*!
    An application that list the month name of the selected locale. If you need inspiration see
    https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes

    Requires the following features: `cargo run --example month_name_d --features "winnls textbox"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct CalendarNames {
    #[nwg_control(size: (300, 230), position: (600, 300), title: "Localization example")]
    #[nwg_events( OnWindowClose: [CalendarNames::exit], OnInit: [CalendarNames::init] )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "Locale:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    locale_label: nwg::Label,

    #[nwg_control(focus: true)]
    #[nwg_layout_item(layout: layout, col: 1, row: 0, col_span: 2)]
    locale_input: nwg::TextInput,

    #[nwg_control(text: "Load")]
    #[nwg_layout_item(layout: layout, col: 3, row: 0)]
    #[nwg_events( OnButtonClick: [CalendarNames::load] )]
    local_load: nwg::Button,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, col: 0, row: 1, col_span: 4, row_span: 4)]
    months: nwg::TextBox,
}

impl CalendarNames {
    
    fn init(&self) {
        let locale = nwg::Locale::user();
        self.locale_input.set_text(locale.name());
        self.load_months(&locale);
    }

    fn load(&self) {
        match nwg::Locale::new(self.locale_input.text()) {
            Ok(loc) => {
                self.load_months(&loc);
            },
            Err(_) => {
                nwg::error_message("Error", "Failed to load locale");
            }
        }
    }

    fn load_months(&self, locale: &nwg::Locale) {
        let mut months_string = String::new();

        months_string.push_str(&locale.display_name());
        months_string.push_str("\r\n\r\n");

        for i in 1..=12 {
            let month_name = locale.month_name(i);
            months_string.push_str(&month_name);
            months_string.push_str("\r\n");
        }

        self.months.set_text(&months_string);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = CalendarNames::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
