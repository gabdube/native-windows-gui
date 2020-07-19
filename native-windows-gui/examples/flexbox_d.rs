/*!
    A very simple application that show how to use a flexbox layout.

    Requires the following features: `cargo run --example flexbox_d --features "flexbox"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

// Stretch style
use nwg::stretch::{geometry::{Size, Rect}, style::{Dimension as D, FlexDirection, AlignSelf}};
const FIFTY_PC: D = D::Percent(0.5);
const PT_10: D = D::Points(10.0);
const PT_5: D = D::Points(5.0);
const PADDING: Rect<D> = Rect{ start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };
const MARGIN: Rect<D> = Rect{ start: PT_5, end: PT_5, top: PT_5, bottom: PT_5 };


#[derive(Default, NwgUi)]
pub struct FlexBoxApp {
    #[nwg_control(size: (500, 300), position: (300, 300), title: "Flexbox example")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: "Btn 1")]
    #[nwg_layout_item(layout: layout, margin: MARGIN,
        max_size: Size { width: D::Points(200.0), height: D::Undefined },
        size: Size { width: FIFTY_PC, height: D::Auto }
    )]
    button1: nwg::Button,

    #[nwg_control(text: "Btn 2")]
    #[nwg_layout_item(layout: layout,
        margin: MARGIN,
        align_self: AlignSelf::FlexEnd,
        size: Size { width: D::Percent(0.25), height: FIFTY_PC }
    )]
    button2: nwg::Button,

    #[nwg_control(text: "Btn 3")]
    #[nwg_layout_item(layout: layout,
        margin: MARGIN,
        flex_grow: 2.0,
        size: Size { width: D::Auto, height: D::Auto }
    )]
    button3: nwg::Button
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _ui = FlexBoxApp::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
