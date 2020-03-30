/*!
    An application that show how to use the ListView control.

    Requires the following features: `cargo run --example dataview_d --features "list-view"`
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

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    data_view: nwg::ListView
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
