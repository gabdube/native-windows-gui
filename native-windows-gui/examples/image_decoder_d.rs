/*!
    A application that uses the `image-decoder` feature to load resources and display them.

    Requires the following features: `cargo run --example image_decoder_d --features "image-decoder file-dialog"`
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::{env};


#[derive(Default, NwgUi)]
pub struct ImageDecoderApp {
    #[nwg_control(size: (400, 300), position: (400, 150), title: "Image decoder")]
    #[nwg_events( OnWindowClose: [ImageDecoderApp::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, max_row: Some(5), max_column: Some(5) )]
    main_layout: nwg::GridLayout,

    #[nwg_resource]
    decoder: nwg::ImageDecoder,

    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "Png(*.png)|Jpeg(*.jpg;*.jpeg)|DDS(*.dds)|TIFF(*.tiff)|BMP(*.bmp)")]
    dialog: nwg::FileDialog,

    #[nwg_control(text: "Open")]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 0)]
    #[nwg_events(OnButtonClick: [ImageDecoderApp::open_file])]
    open_btn: nwg::Button,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 0, col_span: 4)]
    file_name: nwg::TextInput,

    #[nwg_control]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 1, col_span: 5, row_span: 4)]
    img: nwg::ImageFrame,
}

impl ImageDecoderApp {

    fn open_file(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog.set_default_folder(d).expect("Failed to set default folder.");
            }
        }
        
        if self.dialog.run() {
            self.file_name.set_text("");
            if let Ok(directory) = self.dialog.get_selected_item() {
                self.file_name.set_text(&directory);
                self.read_file();
            }
        }
    }

    fn read_file(&self) {
        let image = self.decoder.from_filename(&self.file_name.text());
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = ImageDecoderApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
