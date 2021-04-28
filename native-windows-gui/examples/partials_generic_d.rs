/*!
    An application that load different interfaces using the partial feature.
    All partials are represented as the same generic struct.

    Requires the following features: `cargo run --example partials_generic_d --features "listbox frame combobox"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::fmt::Display;
use std::cell::RefCell;

use nwd::{NwgUi, NwgPartial};
use nwg::NativeUi;


#[derive(NwgUi)]
pub struct PartialGenericDemo<T1: Display + Default + 'static, T2, T3>
    where T2: Display + Default + 'static,
          T3: Display + Default + 'static {
    #[nwg_control(size: (500, 200), position: (300, 300), title: "Many generic UI", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [PartialGenericDemo::exit])]
    window: nwg::Window,

    #[nwg_control(collection: vec![data.f1_ui.title, data.f2_ui.title, data.f3_ui.title], size: (180, 180), position: (10, 10))]
    #[nwg_events(OnListBoxSelect: [PartialGenericDemo::change_interface])]
    menu: nwg::ListBox<&'static str>,

    #[nwg_control(size: (290, 180), position: (200, 10))]
    frame1: nwg::Frame,

    #[nwg_control(size: (290, 180), position: (200, 10), flags: "BORDER")]
    frame2: nwg::Frame,

    #[nwg_control(size: (290, 180), position: (200, 10), flags: "BORDER")]
    frame3: nwg::Frame,

    #[nwg_partial(parent: frame1)]
    #[nwg_events((save_btn, OnButtonClick): [PartialGenericDemo::show(SELF, CTRL)])]
    f1_ui: GenericFrameUi<T1>,

    #[nwg_partial(parent: frame2)]
    #[nwg_events((save_btn, OnButtonClick): [PartialGenericDemo::show(SELF, CTRL)])]
    f2_ui: GenericFrameUi<T2>,

    #[nwg_partial(parent: frame3)]
    #[nwg_events((save_btn, OnButtonClick): [PartialGenericDemo::show(SELF, CTRL)])]
    f3_ui: GenericFrameUi<T3>,
}

impl<T1, T2, T3> PartialGenericDemo<T1, T2, T3>
    where T1: Display + Default,
          T2: Display + Default,
          T3: Display + Default {
    fn change_interface(&self) {
        let frames = [&self.frame1, &self.frame2, &self.frame3];
        frames.iter().for_each(|f| f.set_visible(false));

        let selected_frame = match self.menu.selection() {
            Some(x) => frames[x],
            None => frames[0],
        };
        selected_frame.set_visible(true);
    }

    fn show<T: Display + Default>(&self, frame: &GenericFrameUi<T>) {
        let message = match frame.combobox.selection() {
            Some(v) => format!("'{}' is our choice from '{}' frame", frame.combobox.collection()[v], frame.title),
            None => "Please choose something".to_owned(),
        };
        nwg::simple_message("Show message", &message);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

#[derive(NwgPartial)]
pub struct GenericFrameUi<T: Display + Default + 'static> {
    title: &'static str,

    #[nwg_control(size: (270, 40), position: (10, 10), text: data.text)]
    l: nwg::Label,
    text: &'static str,

    #[nwg_control(collection: data.combo_items.borrow_mut().take().unwrap_or_default(), selected_index: Some(0), size: (270, 40), position: (10, 60))]
    combobox: nwg::ComboBox<T>,
    combo_items: RefCell<Option<Vec<T>>>,

    #[nwg_control(text: "Show", size: (270, 40), position: (10, 110))]
    save_btn: nwg::Button,
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let demo = PartialGenericDemo {
        window: Default::default(),
        menu: Default::default(),
        frame1: Default::default(),
        frame2: Default::default(),
        frame3: Default::default(),
        f1_ui: GenericFrameUi {
            title: "Numbers",
            text: "i32 numbers",
            combo_items: Some(vec![1, 2, 3]).into(),
            l: Default::default(),
            combobox: Default::default(),
            save_btn: Default::default(),
        },
        f2_ui: GenericFrameUi {
            title: "Strings",
            text: "Static strings",
            combo_items: Some(vec!["String 1", "String 2", "String 3"]).into(),
            l: Default::default(),
            combobox: Default::default(),
            save_btn: Default::default(),
        },
        f3_ui: GenericFrameUi {
            title: "Booleans",
            text: "Bool values",
            combo_items: Some(vec![true, false, false]).into(),
            l: Default::default(),
            combobox: Default::default(),
            save_btn: Default::default(),
        },
    };

    let _ui = PartialGenericDemo::build_ui(demo).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
