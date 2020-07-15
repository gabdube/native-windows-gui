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
    #[nwg_events( OnWindowClose: [DataViewApp::exit], OnInit: [DataViewApp::load_data], MousePressLeftDown: [DataViewApp::test] )]
    window: nwg::Window,
    
    #[nwg_resource(family: "Arial", size: 19)]
    arial: nwg::Font,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(item_count: 10, size: (500, 350), list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT, 
    )]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 4, row: 0, row_span: 6)]
    data_view: nwg::ListView,

    #[nwg_control(text: "View:", font: Some(&data.arial))]
    #[nwg_layout_item(layout: layout, col: 4, row: 0)]
    label: nwg::Label,

    #[nwg_control(collection: vec!["Simple", "Details", "Icon", "Icon small"], selected_index: Some(1), font: Some(&data.arial))]
    #[nwg_layout_item(layout: layout, col: 4, row: 1)]
    #[nwg_events( OnComboxBoxSelection: [DataViewApp::update_view] )]
    view_style: nwg::ComboBox<&'static str>
}

impl DataViewApp {
    
    fn load_data(&self) {
        let dv = &self.data_view;

        dv.insert_column("Name");
        dv.insert_column("Genus");

        // Passing a str to this method will automatically push the item at the end of the list in the first column
        dv.insert_item("Cat");
        dv.insert_item(nwg::InsertListViewItem { 
            index: Some(0),
            column_index: 1,
            text: "Felis".into()
        });

        // To insert a new row, use the index 0.
        dv.insert_item(nwg::InsertListViewItem {
            index: Some(0),
            column_index: 0,
            text: "Moose".into(),
        });

        dv.insert_item(nwg::InsertListViewItem {
            index: Some(0),
            column_index: 1,
            text: "Alces".into(),
        });

        // Insert multiple item on a single row. 
        dv.insert_items_row(None, &["Dog", "Canis"]);

        // Insert many item at one
        dv.insert_items(&["Duck", "Horse", "Boomalope"]);
        dv.insert_items(&[
            nwg::InsertListViewItem { index: Some(3), column_index: 1, text: "Anas".into() },
            nwg::InsertListViewItem { index: Some(4), column_index: 1, text: "Equus".into() },
        ]);
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

    fn test(&self) {
        let dv = &self.data_view;

        dv.set_text_color(120, 120, 120);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = DataViewApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
