/*!
    An application that show how to use the ListView control.

    Requires the following features: `cargo run --example dataview_d --features "list-view combobox"`
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct DataViewApp {
    #[nwg_control(size: (500, 350), position: (300, 300), title: "DataView")]
    #[nwg_events( OnWindowClose: [DataViewApp::exit] )]
    window: nwg::Window,

    #[nwg_resource(family: "Arial", size: 19)]
    arial: nwg::Font,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 4, row: 0, row_span: 6)]
    data_view: nwg::ListView,

    #[nwg_control(text: "View", font: Some(&data.arial))]
    #[nwg_layout_item(layout: layout, col: 4, row: 0)]
    label: nwg::Label,

    #[nwg_control(collection: vec!["Details", "Icon", "List", "Tile"], selected_index: Some(0), font: Some(&data.arial))]
    #[nwg_layout_item(layout: layout, col: 4, row: 1)]
    view_style: nwg::ComboBox<&'static str>
}

impl DataViewApp {
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = DataViewApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
