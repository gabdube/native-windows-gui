/*!
    An application that load new fonts from file
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct CustomFontApp {
    #[nwg_control(size: (300, 200), position: (300, 300), title: "Custom Fonts")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1, min_size: [200, 100])]
    grid: nwg::GridLayout,

    #[nwg_resource(family: "Fredoka One", size: 28)]
    font1: nwg::Font,

    #[nwg_resource(family: "MS Sans Serif", size: 22)]
    font2: nwg::Font,

    #[nwg_resource(family: "Indie Flower", size: 30)]
    font3: nwg::Font,

    #[nwg_control(text: "Hello World", font: Some(&data.font1))]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    label1: nwg::Label,

    #[nwg_control(text: "The quick brown fox", font: Some(&data.font2))]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    label2: nwg::Label,

    #[nwg_control(text: "WATCH THE SKY", font: Some(&data.font3))]
    #[nwg_layout_item(layout: grid, row: 2, col: 0)]
    label3: nwg::Label,
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    // Font data loaded from memory must be mutable. 
    // You should use embedded resources or load them directly from disk if possible
    let font_data = include_bytes!("../test_rc/FredokaOne-Regular.ttf");
    let mut font_data_mut = font_data.to_vec(); 
    let mem_font = nwg::Font::add_memory_font(&mut font_data_mut).expect("Failed to load font from memory");

    // Loading a font from a file is that simple
    nwg::Font::add_font("./test_rc/IndieFlower-Regular.ttf");

    let _app = CustomFontApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();

    nwg::Font::remove_memory_font(mem_font);
    nwg::Font::remove_font("./test_rc/IndieFlower-Regular.ttf");
}
