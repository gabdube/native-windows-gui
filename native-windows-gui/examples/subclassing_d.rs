/*!
    Example on how to use custom types directly with native windows derive
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

type UserButton = nwg::Button;

#[derive(Default, NwgUi)]
pub struct SubclassApp {
    #[nwg_control(size: (300, 300), position: (700, 300), title: "Subclass example")]
    #[nwg_events( OnWindowClose: [SubclassApp::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::BoxLayout,

    #[nwg_control(text: "Simple button")]
    #[nwg_layout_item(layout: layout, cell: 0)]
    button1: nwg::Button,

    #[nwg_control(ty: Button, text: "User type button")]
    #[nwg_layout_item(layout: layout, cell: 1)]
    button2: UserButton,

    #[nwg_control(text: "Subclassed button")]
    #[nwg_layout_item(layout: layout, cell: 2)]
    button3: nwg::Button,

    #[nwg_control(text: "Custom builder button")]
    #[nwg_layout_item(layout: layout, cell: 3)]
    button4: nwg::Button,
}

impl SubclassApp {

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = SubclassApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
