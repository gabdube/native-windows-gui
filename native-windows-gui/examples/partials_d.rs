/*!
    An application that load different interfaces using the partial feature.
    Partials can be used to split large GUI application into smaller bits.

    Requires the following features: `cargo run --example partials_d --features "listbox frame combobox"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::{NwgUi, NwgPartial};
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct PartialDemo {
    #[nwg_control(size: (500, 400), position: (300, 300), title: "Many UI")]
    #[nwg_events( OnWindowClose: [PartialDemo::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(collection: vec!["People", "Animals", "Food"])]
    #[nwg_layout_item(layout: layout, cell: 0)]
    #[nwg_events( OnListBoxSelect: [PartialDemo::change_interface] )]
    menu: nwg::ListBox<&'static str>,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, cell: 1, cell_span: 3)]
    frame1: nwg::Frame,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, cell: 1, cell_span: 3)]
    frame2: nwg::Frame,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, cell: 1, cell_span: 3)]
    frame3: nwg::Frame,

    #[nwg_partial(parent: frame1)]
    people_ui: PeopleUi,

    #[nwg_partial(parent: frame2)]
    animal_ui: AnimalUi,

    #[nwg_partial(parent: frame3)]
    food_ui: FoodUi,
}

impl PartialDemo {

    fn change_interface(&self) {
        self.frame1.set_visible(false);
        self.frame2.set_visible(false);
        self.frame3.set_visible(false);

        match self.menu.selection() {
            None | Some(0) => self.frame1.set_visible(true),
            Some(1) => self.frame2.set_visible(true),
            Some(2) => self.frame3.set_visible(true),
            Some(_) => unreachable!()
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default, NwgPartial)]
pub struct PeopleUi {

    #[nwg_layout(max_size: [1000, 150], min_size: [100, 120])]
    layout: nwg::GridLayout,

    #[nwg_control(text: "Name:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    label1: nwg::Label,

    #[nwg_control(text: "Age:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 1)]
    label2: nwg::Label,

    #[nwg_control(text: "Job:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 2)]
    label3: nwg::Label,

    #[nwg_control(text: "John Doe")]
    #[nwg_layout_item(layout: layout, col: 1, row: 0)]
    name_input: nwg::TextInput,

    #[nwg_control(text: "75", flags: "NUMBER|VISIBLE")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    age_input: nwg::TextInput,

    #[nwg_control(text: "Programmer")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    job_input: nwg::TextInput,
}

#[derive(Default, NwgPartial)]
pub struct AnimalUi {
    #[nwg_layout(max_size: [1000, 150], min_size: [100, 120])]
    layout: nwg::GridLayout,

    #[nwg_control(text: "Name:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    label1: nwg::Label,

    #[nwg_control(text: "Race:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 1)]
    label2: nwg::Label,

    #[nwg_control(text: "Is fluffy:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 2)]
    label3: nwg::Label,

    #[nwg_control(text: "Mittens")]
    #[nwg_layout_item(layout: layout, col: 1, row: 0)]
    name_input: nwg::TextInput,

    #[nwg_control(collection: vec!["Cat", "Dog", "Pidgeon", "Monkey"], selected_index: Some(0))]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    race_input: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "", check_state: CheckBoxState::Checked)]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    is_soft_input: nwg::CheckBox
}

#[derive(Default, NwgPartial)]
pub struct FoodUi {
    
    #[nwg_layout(max_size: [1000, 90], min_size: [100, 80])]
    layout: nwg::GridLayout,

    #[nwg_control(text: "Name:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    label1: nwg::Label,

    #[nwg_control(text: "Tasty:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: layout, col: 0, row: 1)]
    label2: nwg::Label,

    #[nwg_control(text: "Banana")]
    #[nwg_layout_item(layout: layout, col: 1, row: 0)]
    name_input: nwg::TextInput,

    #[nwg_control(text: "", check_state: CheckBoxState::Checked)]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    tasty_input: nwg::CheckBox,
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _ui = PartialDemo::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
