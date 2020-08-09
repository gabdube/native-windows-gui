/*!
    Shows how to add controls dynamically into a flexbox layout
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::{NativeUi, stretch};
use stretch::geometry::Size;
use stretch::style::*;

use std::cell::RefCell;


#[derive(Default, NwgUi)]
pub struct FlexboxDynamic {
    #[nwg_control(size: (300, 500), position: (400, 200), title: "Flexbox example")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [FlexboxDynamic::setup] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    buttons: RefCell<Vec<nwg::Button>>,
}

impl FlexboxDynamic {

    fn setup(&self) {
        let mut buttons = self.buttons.borrow_mut();
        for i in 0.. 20 {
            buttons.push(nwg::Button::default());

            let button_index = buttons.len() - 1;

            nwg::Button::builder()
                .text(&format!("Button {}", i+1))
                .parent(&self.window)
                .build(&mut buttons[button_index]).expect("Failed to create button");

            
            let style = Style {
                size: Size { width: Dimension::Auto, height: Dimension::Points(100.0) },
                justify_content: JustifyContent::Center,
                ..Default::default()
            };

            self.layout.add_child(&buttons[button_index], style).expect("Failed to add button to layout");
        }
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = FlexboxDynamic::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
