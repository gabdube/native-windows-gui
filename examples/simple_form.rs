extern crate native_windows_gui as nwg;

use nwg::Ui;
use nwg::controls::{Label, Window, TextInput, GroupBox, RadioButton};
use nwg::actions::Action;
use nwg::constants::{HTextAlign, VTextAlign};

fn setup_controls(ui: &mut Ui<&'static str>) {
    let main_window = Window {
        caption: "Application Form".to_string(),
        size: (350, 500),
        position: (100, 100),
        visible: false,
        resizable: false,
        exit_on_close: true
    };

    let name_label = Label {
        text: "Name:".to_string(),
        size: (50, 20),
        position: (40, 10),
        parent: "MainWindow",
        text_align: HTextAlign::Left
    };

    let name_input = TextInput {
        text: "".to_string(),
        size: (240, 20),
        position: (95, 10),
        parent: "MainWindow",
        placeholder: Some("FirstName LastName".to_string()),
        text_align: HTextAlign::Left,
        password: false,
        readonly: false
    };

    let job_label = Label {
        text: "Profession:".to_string(),
        size: (80, 20),
        position: (10, 50),
        parent: "MainWindow",
        text_align: HTextAlign::Left
    };

    let job_input = TextInput {
        text: "".to_string(),
        size: (240, 20),
        position: (95, 50),
        parent: "MainWindow",
        placeholder: None,
        text_align: HTextAlign::Left,
        password: false,
        readonly: false
    };

    let exp_box = GroupBox {
        text: "Angular 2.0 Experience".to_string(),
        size: (330, 60),
        position: (10, 85),
        parent: "MainWindow",
        text_align: HTextAlign::Left
    };

    let exp_less = RadioButton {
        text: "< 1 year".to_string(),
        size: (70, 20),
        position: (10, 25),
        parent: "ExpBox",
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };

    let exp_some = RadioButton {
        text: "1 to 3 years".to_string(),
        size: (100, 20),
        position: (110, 25),
        parent: "ExpBox",
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };

    let exp_alot = RadioButton {
        text: "> 3 years".to_string(),
        size: (80, 20),
        position: (230, 25),
        parent: "ExpBox",
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };


    ui.new_control("MainWindow", main_window).unwrap();
    ui.new_control("NameLabel", name_label).unwrap();
    ui.new_control("NameInput", name_input).unwrap();
    ui.new_control("JobLabel", job_label).unwrap();
    ui.new_control("JobInput", job_input).unwrap();
    ui.new_control("ExpBox", exp_box).unwrap();
    ui.new_control("ExpLess", exp_less).unwrap();
    ui.new_control("ExpSome", exp_some).unwrap();
    ui.new_control("ExpAlot", exp_alot).unwrap();

    ui.exec("MainWindow", Action::SetVisibility(true)).unwrap();
}

fn main() {
    let mut ui: Ui<&'static str> = Ui::new();

    setup_controls(&mut ui);

    nwg::dispatch_events();
}