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
    #[nwg_events( OnWindowClose: [DataViewApp::exit], OnInit: [DataViewApp::load_data] )]
    window: nwg::Window,

    #[nwg_resource(family: "Arial", size: 19)]
    arial: nwg::Font,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(item_count: 10)]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 4, row: 0, row_span: 6)]
    data_view: nwg::ListView,

    #[nwg_control(text: "View:", font: Some(&data.arial))]
    #[nwg_layout_item(layout: layout, col: 4, row: 0)]
    label: nwg::Label,

    #[nwg_control(collection: vec!["Simple", "Details", "Icon", "Icon small", "Tile"], selected_index: Some(0), font: Some(&data.arial))]
    #[nwg_layout_item(layout: layout, col: 4, row: 1)]
    #[nwg_events( OnComboxBoxSelection: [DataViewApp::update_view] )]
    view_style: nwg::ComboBox<&'static str>
}

impl DataViewApp {
    
    fn load_data(&self) {
        self.data_view.insert_item(nwg::InsertListViewItem { 
            text: "Item 1".to_string(),
            ..Default::default()
        })
    }

    fn update_view(&self) {
        let value = self.view_style.selection_string();
        let view = &self.data_view;

        /*match value.as_ref().map(|v| v as &str) {
            Some("Icon") => view.set_list_type(nwg::ListViewFlags::ICON_LIST),
            Some("Icon small") => view.set_list_type(nwg::ListViewFlags::SMALL_ICON_LIST),
            Some("Details") => view.set_list_type(nwg::ListViewFlags::DETAILED_LIST),
            Some("Tile") => view.set_list_type(nwg::ListViewFlags::TILE_LIST),
            None | Some(_) => view.set_list_type(nwg::ListViewFlags::SIMPLE_LIST),
        }*/
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
