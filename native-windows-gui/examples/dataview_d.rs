/*!
    An application that show how to use the ListView control.

    Requires the following features: `cargo run --example dataview_d --features "list-view combobox image-list"`
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct DataViewApp {
    #[nwg_control(size: (500, 350), position: (300, 300), title: "DataView - Animals list")]
    #[nwg_events( OnWindowClose: [DataViewApp::exit], OnInit: [DataViewApp::load_data])]
    window: nwg::Window,
    
    #[nwg_resource(family: "Arial", size: 19)]
    arial: nwg::Font,

    #[nwg_resource(initial: 5)]
    view_icons: nwg::ImageList,

    #[nwg_resource(initial: 5, size: (16, 16))]
    view_icons_small: nwg::ImageList,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(item_count: 10, size: (500, 350), list_style: nwg::ListViewStyle::Detailed, focus: true,
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
        let icons = &self.view_icons;
        let icons_small = &self.view_icons_small;

        // Load the listview images
        icons.add_icon_from_filename("./test_rc/cog.ico").unwrap();
        icons.add_icon_from_filename("./test_rc/love.ico").unwrap();
        icons_small.add_icon_from_filename("./test_rc/cog.ico").unwrap();
        icons_small.add_icon_from_filename("./test_rc/love.ico").unwrap();

        // Setting up the listview data
        dv.set_image_list(Some(icons), nwg::ListViewImageListType::Normal);
        dv.set_image_list(Some(icons_small), nwg::ListViewImageListType::Small);

        dv.insert_column("Name");
        dv.insert_column(nwg::InsertListViewColumn{
            index: Some(1),
            fmt: Some(nwg::ListViewColumnFlags::RIGHT),
            width: Some(20),
            text: Some("test".into())
        });
        dv.set_headers_enabled(true);

        // Passing a str to this method will automatically push the item at the end of the list in the first column
        dv.insert_item("Cat");
        dv.insert_item(nwg::InsertListViewItem { 
            index: Some(0),
            column_index: 1,
            text: Some("Felis".into()),
            image: None
        });

        // To insert a new row, use the index 0.
        dv.insert_item(nwg::InsertListViewItem {
            index: Some(0),
            column_index: 0,
            text: Some("Moose".into()),
            image: Some(1),
        });

        dv.insert_item(nwg::InsertListViewItem {
            index: Some(0),
            column_index: 1,
            text: Some("Alces".into()),
            image: None,
        });

        // Insert multiple item on a single row. 
        dv.insert_items_row(None, &["Dog", "Canis"]);

        // Insert many item at one
        dv.insert_items(&["Duck", "Horse", "Boomalope"]);
        dv.insert_items(&[
            nwg::InsertListViewItem { index: Some(3), column_index: 1, text: Some("Anas".into()), image: None },
            nwg::InsertListViewItem { index: Some(4), column_index: 1, text: Some("Equus".into()), image: None },
        ]);

        // Update items
        dv.update_item(2, nwg::InsertListViewItem { image: Some(1), ..Default::default() });
        dv.update_item(4, nwg::InsertListViewItem { image: Some(1), ..Default::default() });
    }

    fn update_view(&self) {
        let value = self.view_style.selection_string();
        
        let style = match value.as_ref().map(|v| v as &str) {
            Some("Icon") => nwg::ListViewStyle::Icon,
            Some("Icon small") => nwg::ListViewStyle::SmallIcon,
            Some("Details") => nwg::ListViewStyle::Detailed,
            None | Some(_) => nwg::ListViewStyle::Simple,
        };

        self.data_view.set_list_style(style);
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
