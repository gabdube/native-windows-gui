/*!
    Small example that shows how to scroll and append text to a text box
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::fs;

#[derive(Default, NwgUi)]
pub struct EchoApp {
    #[nwg_control(size: (1000, 420), position: (300, 300), title: "Echo", accept_files: true)]
    #[nwg_events( 
        OnInit: [EchoApp::init_text],
        OnWindowClose: [nwg::stop_thread_dispatch()], 
        OnFileDrop: [EchoApp::load_text(SELF, EVT_DATA)], 
        OnKeyEnter: [EchoApp::submit], 
    )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text:"Loading...\r\n", readonly: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 7, row_span: 4)]
    text: nwg::TextBox,

    #[nwg_control(focus: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 4, col_span: 7)]
    text_input: nwg::TextInput,


    #[nwg_control(text: "Clear")]
    #[nwg_layout_item(layout: grid, col: 7, row: 0)]
    #[nwg_events( OnButtonClick: [EchoApp::clear] )]
    b1: nwg::Button,
    #[nwg_control(text: "Scroll Top")]
    #[nwg_layout_item(layout: grid, col: 7, row: 1)]
    #[nwg_events( OnButtonClick: [EchoApp::scroll_to_top] )]
    b2: nwg::Button,
    #[nwg_control(text: "Scroll Mid")]
    #[nwg_layout_item(layout: grid, col: 7, row: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::scroll_to_mid] )]
    b3: nwg::Button,
    #[nwg_control(text: "Scroll Bot")]
    #[nwg_layout_item(layout: grid, col: 7, row: 3)]
    #[nwg_events( OnButtonClick: [EchoApp::scroll_to_bot] )]
    b4: nwg::Button,
    #[nwg_control(text: "Submit")]
    #[nwg_layout_item(layout: grid, col: 7, row: 4)]
    #[nwg_events( OnButtonClick: [EchoApp::submit] )]
    b5: nwg::Button,
}

impl EchoApp {
    pub fn load_text(&self, data: &nwg::EventData) {
        let drop = data.on_file_drop();

        let mut text = String::with_capacity(1000);

        for file in drop.files() {
            text.push_str(&fs::read_to_string(file).unwrap_or("Invalid file".into()));
        }
        
        self.text.appendln(&text);
    }

    pub fn init_text(&self) {
        self.text.set_text_unix2dos("This text box will echo any text submitted below.\n");
        self.text.append("Printing lines 2-256 to demo scrolling: ");
        for i in 2..257 { 
            self.text.appendln(&format!("{}", i));
        }
    }

    pub fn clear(&self) {
        self.text.clear();
    }

    pub fn scroll_to_top(&self) {
        self.text.scroll(self.text.linecount() * -1);
    }

    pub fn scroll_to_mid(&self) {
        self.scroll_to_top();
        self.text.scroll(self.text.linecount() / 2);
    }

    pub fn scroll_to_bot(&self) {
        self.text.scroll_lastline();
    }

    pub fn submit(&self) {
        self.text.appendln(&self.text_input.text());
        self.text_input.set_text("");
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Courier New").expect("Failed to set default font");

    let _app = EchoApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
