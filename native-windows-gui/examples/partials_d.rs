/*!
    An application that load different interfaces using the partial feature.
    Partials can be used to split large GUI application into smaller bits.

    Requires the following features: `cargo run --example partials_d --features "listbox frame combobox checkbox"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::{NwgUi};
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct PartialDemo {
    #[nwg_control(size: (500, 400), position: (300, 300), title: "Many UI")]
    #[nwg_events( OnWindowClose: [PartialDemo::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, layout_type: BoxLayoutType::Horizontal)]
    layout: nwg::BoxLayout,

    #[nwg_control(collection: vec!["People", "Animals", "Food"])]
    #[nwg_layout_item(layout: layout, cell: 0)]
    #[nwg_events( OnListBoxSelect: [PartialDemo::change_interface] )]
    menu: nwg::ListBox<&'static str>,

    #[nwg_control()]
    #[nwg_layout_item(layout: layout, cell: 1, cell_span: 3)]
    frame1: nwg::Frame,

    #[nwg_control()]
    #[nwg_layout_item(layout: layout, cell: 1, cell_span: 3)]
    frame2: nwg::Frame,

    #[nwg_control()]
    #[nwg_layout_item(layout: layout, cell: 1, cell_span: 3)]
    frame3: nwg::Frame,

    people_ui: PeopleUi,
    animal_ui: AnimalUi,
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

#[derive(Default)]
pub struct PeopleUi {
    layout: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,
    label3: nwg::Label,

    name_input: nwg::TextInput,
    age_input: nwg::TextInput,
    job_input: nwg::TextInput,
}

#[derive(Default)]
pub struct AnimalUi {
    layout: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,
    label3: nwg::Label,

    name_input: nwg::TextInput,
    race_input: nwg::ComboBox<&'static str>,
    is_soft_input: nwg::CheckBox
}

#[derive(Default)]
pub struct FoodUi {
    layout: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,

    name_input: nwg::TextInput,
    tasty_input: nwg::CheckBox,
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _ui = PartialDemo::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
