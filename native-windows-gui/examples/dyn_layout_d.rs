
use native_windows_gui as nwg;
use native_windows_derive as nwd;

use nwd::{NwgUi, NwgPartial};
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct ConfigDlg {
    #[nwg_control(size: (500, 400), position: (300, 300), title: "DynLayout")]
    #[nwg_events(OnInit: [ConfigDlg::init], OnResize: [ConfigDlg::size], OnWindowClose: [ConfigDlg::exit])]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::DynLayout,

    #[nwg_control(position: (10, 30), size: (220, 330), collection: vec!["People"])]
    list: nwg::ListBox<&'static str>,

    #[nwg_control(text: "Cancel", position: (10, 350), size: (100, 25))]
    cancel_btn: nwg::Button,

    #[nwg_control(text: "Ok", position: (120, 350), size: (100, 25))]
    ok_btn: nwg::Button,

    #[nwg_control(text: "Config", position: (380, 350), size: (100, 25))]
    config_btn: nwg::Button,

    #[nwg_control(position: (240, 30), size: (240, 300))]
    frame: nwg::Frame,

    #[nwg_partial(parent: frame)]
    #[nwg_events((save_btn, OnButtonClick): [ConfigDlg::save])]
    controls: Controls,
}

impl ConfigDlg {
    fn init(&self) {
        self.frame.set_visible(true);

        self.layout.add_child((0, 0), (50, 100), &self.list);
        self.layout.add_child((0, 100), (0, 0), &self.ok_btn);
        self.layout.add_child((0, 100), (0, 0), &self.cancel_btn);
        self.layout.add_child((100, 100), (0, 0), &self.config_btn);

        self.layout.add_child((50, 0), (50, 100), &self.frame);

        self.controls.init(&self.frame);

        self.layout.fit();
    }

    fn size(&self) {
        self.layout.fit();
    }

    fn save(&self) {
        nwg::simple_message("Saved!", "Data saved!");
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default, NwgPartial)]
pub struct Controls {
    #[nwg_layout]
    layout: nwg::DynLayout,

    #[nwg_control(text: "Name:", h_align: HTextAlign::Right, position: (10, 10), size: (100, 20))]
    label1: nwg::Label,

    #[nwg_control(text: "Age:", h_align: HTextAlign::Right, position: (10, 40), size: (100, 20))]
    label2: nwg::Label,

    #[nwg_control(text: "Job:", h_align: HTextAlign::Right, position: (10, 70), size: (100, 20))]
    label3: nwg::Label,

    #[nwg_control(text: "John Doe", position: (120, 10), size: (100, 20))]
    #[nwg_events(OnChar: [print_char(EVT_DATA)])]
    name_input: nwg::TextInput,

    #[nwg_control(text: "75", flags: "NUMBER|VISIBLE", position: (120, 40), size: (100, 20))]
    age_input: nwg::TextInput,

    #[nwg_control(text: "Programmer", position: (120, 70), size: (100, 25))]
    job_input: nwg::TextInput,

    #[nwg_control(text: "Save", position: (10, 250), size: (100, 25))]
    save_btn: nwg::Button,
}

impl Controls {
    fn init(&self, frame: &nwg::Frame) {
        self.layout.parent(frame);

        self.layout.add_child((0, 0), (0, 0), &self.label1);
        self.layout.add_child((0, 0), (0, 0), &self.label2);
        self.layout.add_child((0, 0), (0, 0), &self.label3);

        self.layout.add_child((0, 0), (100, 0), &self.name_input);
        self.layout.add_child((0, 0), (100, 0), &self.age_input);
        self.layout.add_child((0, 0), (100, 0), &self.job_input);

        self.layout.add_child((0, 100), (0, 0), &self.save_btn);
    }
}

fn print_char(data: &nwg::EventData) {
    println!("{:?}", data.on_char());
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    //nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("MS Shell Dlg")
        .size(15)
        .build(&mut font)
        .expect("Failed to build font");
    nwg::Font::set_global_default(Some(font));

    let _ui = ConfigDlg::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
