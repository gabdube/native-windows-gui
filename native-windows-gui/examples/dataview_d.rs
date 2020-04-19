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
    #[nwg_control(size: (500, 350), position: (300, 300), title: "DataView - Animals list")]
    #[nwg_events( OnWindowClose: [DataViewApp::exit], OnInit: [DataViewApp::load_data] )]
    window: nwg::Window,

    #[nwg_resource(family: "Arial", size: 19)]
    arial: nwg::Font,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(item_count: 10, list_style: nwg::ListViewStyle::Detailed, size: (500, 350))]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 4, row: 0, row_span: 6)]
    data_view: nwg::ListView,

    #[nwg_control(text: "View:", font: Some(&data.arial))]
    #[nwg_layout_item(layout: layout, col: 4, row: 0)]
    label: nwg::Label,

    #[nwg_control(collection: vec!["Simple", "Details", "Icon", "Icon small"], selected_index: Some(0), font: Some(&data.arial))]
    #[nwg_layout_item(layout: layout, col: 4, row: 1)]
    #[nwg_events( OnComboxBoxSelection: [DataViewApp::update_view] )]
    view_style: nwg::ComboBox<&'static str>
}

impl DataViewApp {
    
    fn load_data(&self) {
        let dv = &self.data_view;

        dv.insert_column("Name");
        dv.insert_column("Genus");

        dv.insert_item("Cat");
        dv.insert_item("Dog");
        dv.insert_item("Moose");
        dv.insert_items(&["Duck", "Horse", "Boomalope"]);
    }

    fn update_view(&self) {
        let value = self.view_style.selection_string();
        let view = &self.data_view;

        let style = match value.as_ref().map(|v| v as &str) {
            Some("Icon") => nwg::ListViewStyle::Icon,
            Some("Icon small") => nwg::ListViewStyle::SmallIcon,
            Some("Details") => nwg::ListViewStyle::Detailed,
            None | Some(_) => nwg::ListViewStyle::Simple,
        };

        view.set_list_style(style);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = DataViewApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
