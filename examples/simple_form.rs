extern crate native_windows_gui as nwg;

use nwg::Ui;
use nwg::controls::{Label, Window, TextInput, GroupBox, RadioButton, ComboBox, CheckBox, Button};
use nwg::actions::Action;
use nwg::constants::{HTextAlign, VTextAlign};

fn col(c: &[&'static str]) -> Vec<String> {
    let mut string_col: Vec<String> = Vec::with_capacity(c.len());
    for i in c.iter() {
        string_col.push(String::from(*i));
    }

    string_col
}

fn setup_controls(ui: &mut Ui<&'static str>) {
    let main_window = Window {
        caption: "Application Form".to_string(),
        size: (350, 350),
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

    let gender_label = Label {
        text: "Gender:".to_string(),
        size: (60, 20),
        position: (30, 92),
        parent: "MainWindow",
        text_align: HTextAlign::Left
    };

    let gender_choice = ComboBox {
        size: (240, 20),
        position: (95, 90),
        parent: "MainWindow",
        collection: col(&["Prefer not to disclose", "Male", "Female", "Combat Helicopter"]),
        sorted: false,
    };

    let exp_box = GroupBox {
        text: "Angular 2.0 Experience".to_string(),
        size: (330, 60),
        position: (10, 135),
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
        text: "> 5 years".to_string(),
        size: (80, 20),
        position: (230, 25),
        parent: "ExpBox",
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };

    let other_exp_box = GroupBox {
        text: "Other experiences".to_string(),
        size: (330, 90),
        position: (10, 210),
        parent: "MainWindow",
        text_align: HTextAlign::Left
    };

    let exp_access = CheckBox {
        text: "Microsoft Access".to_string(),
        size: (140, 20),
        position: (10, 25),
        parent: "OtherExpBox",
        tristate: false,
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };

    let exp_vb6 = CheckBox {
        text: "Visual Basic 6.0".to_string(),
        size: (130, 20),
        position: (170, 25),
        parent: "OtherExpBox",
        tristate: false,
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };

    let exp_agile = CheckBox {
        text: "Agile scrum".to_string(),
        size: (105, 20),
        position: (10, 55),
        parent: "OtherExpBox",
        tristate: false,
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };

    let cancel_button = Button {
        text: "Cancel".to_string(),
        size: (90, 30),
        position: (140, 310),
        parent: "MainWindow",
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };

    let submit_button = Button {
        text: "Submit".to_string(),
        size: (90, 30),
        position: (250, 310),
        parent: "MainWindow",
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
    ui.new_control("GenderLabel", gender_label).unwrap();
    ui.new_control("GenderChoice", gender_choice).unwrap();
    ui.new_control("OtherExpBox", other_exp_box).unwrap();
    ui.new_control("ExpAccess", exp_access).unwrap();
    ui.new_control("ExpVb6", exp_vb6).unwrap();
    ui.new_control("ExpAgile", exp_agile).unwrap();
    ui.new_control("Cancel", cancel_button).unwrap();
    ui.new_control("Submit", submit_button).unwrap();

    ui.exec("MainWindow", Action::SetVisibility(true)).unwrap();
}

fn main() {
    let mut ui: Ui<&'static str> = Ui::new();

    setup_controls(&mut ui);

    nwg::dispatch_events();
}