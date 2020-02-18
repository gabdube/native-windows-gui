/*!
    A application that customize a window to display a splash screen.

    Requires the following features: `cargo run --example splash_screen_d --features "image-decoder"`
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;


#[derive(Default, NwgUi)]
pub struct SplashScreen {
    #[nwg_control(size: (500, 215), position: (700, 300), flags: "POPUP", ex_flags: WindowExFlags::TOPMOST)]
    #[nwg_events( OnInit: [SplashScreen::init] )]
    window: nwg::Window,

    #[nwg_resource]
    decoder: nwg::ImageDecoder,
    splash: RefCell<nwg::Bitmap>,

    #[nwg_control(size: (500, 215))]
    #[nwg_events(OnImageFrameClick: [SplashScreen::exit])]
    image_frame: nwg::ImageFrame
}

impl SplashScreen {
    
    fn init(&self) {
        let splash = self.decoder
            .from_filename("./test_rc/splash.png").unwrap()
            .frame(0).unwrap()
            .as_bitmap().unwrap();

        self.image_frame.set_bitmap(Some(&splash));

        *self.splash.borrow_mut() = splash;
        
        self.window.set_visible(true);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = SplashScreen::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
