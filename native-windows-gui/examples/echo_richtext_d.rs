/*!
    Small example that shows how to append text to a rich text box with styling
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::fs;
use std::cell::RefCell;

#[derive(Default)]
pub struct StyleSection {
    start: u32,
    end: u32,
    char_format: Option<nwg::CharFormat>,
    para_format: Option<nwg::ParaFormat>,
}

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

    #[nwg_resource(family: "Segoe UI", size: 18)]
    font: nwg::Font,

    #[nwg_control(font: Some(&data.font), readonly: true, flags: "VSCROLL|HSCROLL|AUTOVSCROLL|VISIBLE|TAB_STOP|SAVE_SELECTION")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0, col_span: 7, row_span: 4)]
    rich_text_box: nwg::RichTextBox,

    #[nwg_control(focus: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 4, col_span: 7)]
    text_input: nwg::TextInput,

    #[nwg_control(text: "Clear")]
    #[nwg_layout_item(layout: grid, col: 7, row: 0, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::clear] )]
    b1: nwg::Button,
    #[nwg_control(text: "Underline")]
    #[nwg_layout_item(layout: grid, col: 7, row: 1, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::underline] )]
    b2: nwg::Button,
    #[nwg_control(text: "Bullet")]
    #[nwg_layout_item(layout: grid, col: 7, row: 2, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::bullet] )]
    b3: nwg::Button,
    #[nwg_control(text: "Header")]
    #[nwg_layout_item(layout: grid, col: 7, row: 3, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::header] )]
    b4: nwg::Button,
    #[nwg_control(text: "Submit")]
    #[nwg_layout_item(layout: grid, col: 7, row: 4, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::submit] )]
    b5: nwg::Button,

    style_sections: RefCell<Vec<StyleSection>>,
}

impl EchoApp {
    pub fn load_text(&self, data: &nwg::EventData) {
        let drop = data.on_file_drop();

        let mut text = String::with_capacity(1000);

        for file in drop.files() {
            text.push_str(&fs::read_to_string(file).unwrap_or("Invalid file".into()));
        }
        
        self.rich_text_box.appendln(&text);
        self.apply_styles();
    }

    pub fn init_text(&self) {
        self.rich_text_box.set_text_unix2dos("Each button on the right does one of the following:\n");
        self.rich_text_box.append("\r\nClears the rich text box, removing all text and associated formatting.\r\n");
        self.text_input.set_text("Echoes the text in the input box with underline styling.");
        self.underline();
        self.text_input.set_text("Echoes the text in the input box as a bullet point.");
        self.bullet();
        self.text_input.set_text("Echoes the text in the input box with header styling.");
        self.header();
        self.text_input.set_text("Echoes the text in the input box with no styling.");
        self.submit();
    }

    pub fn clear(&self) {
        self.rich_text_box.clear();
        self.style_sections.borrow_mut().clear();
    }

    pub fn underline(&self) {
        let mut style_section = StyleSection { start: self.rich_text_box.len(), ..Default::default()};
        style_section.char_format = Some(nwg::CharFormat {
            effects: Some(nwg::CharEffects::UNDERLINE),
            ..Default::default()
        });
        self.rich_text_box.appendln(&self.text_input.text());
        style_section.end = self.rich_text_box.len();
        self.style_sections.borrow_mut().push(style_section);
        self.text_input.set_text("");
        self.apply_styles();
    }

    pub fn bullet(&self) {
        let mut style_section = StyleSection { start: self.rich_text_box.len(), ..Default::default()};
        style_section.para_format = Some(nwg::ParaFormat {
            start_indent: Some(300),
            right_indent: Some(300),
            line_spacing: Some(nwg::ParaLineSpacing::Double),
            numbering: Some(nwg::ParaNumbering::Bullet),
            numbering_tab: Some(200),
            ..Default::default()
        });
        self.rich_text_box.appendln(&self.text_input.text());
        style_section.end = self.rich_text_box.len();
        self.style_sections.borrow_mut().push(style_section);
        self.text_input.set_text("");
        self.apply_styles();
    }

    pub fn header(&self) {
        let mut style_section = StyleSection { start: self.rich_text_box.len(), ..Default::default()};
        style_section.char_format = Some(nwg::CharFormat {
            effects: Some(nwg::CharEffects::BOLD | nwg::CharEffects::ITALIC),
            height: Some(350),
            text_color: Some([150, 50, 50]),
            ..Default::default()
        });
        style_section.para_format = Some(nwg::ParaFormat {
            alignment: Some(nwg::ParaAlignment::Center),
            ..Default::default()
        });
        self.rich_text_box.appendln(&self.text_input.text());
        style_section.end = self.rich_text_box.len();
        self.style_sections.borrow_mut().push(style_section);
        self.text_input.set_text("");
        self.apply_styles();
    }

    pub fn apply_styles(&self){
        for style_section in self.style_sections.borrow().iter() {   
            self.rich_text_box.set_selection(style_section.start..style_section.end);
            if let Some(p) = &style_section.para_format {
                self.rich_text_box.set_para_format(p);
            }
            if let Some(c) = &style_section.char_format {
                self.rich_text_box.set_char_format(c);
            }
        }
    }

    pub fn submit(&self) {
        self.rich_text_box.appendln(&self.text_input.text());
        self.text_input.set_text("");
        self.apply_styles();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Courier New").expect("Failed to set default font");

    let _app = EchoApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
