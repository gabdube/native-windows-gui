/*!
    A very simple application that show your name in a message box.
    Use a manifest file to load control style and requires admin priviledge to start.

    For more info about resources see https://docs.microsoft.com/en-us/windows/win32/menurc/resource-definition-statements
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;


#[derive(Default, NwgUi)]
pub struct EmbedApp {
    #[nwg_control(size: (300, 145), position: (300, 300), flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [EmbedApp::say_goodbye], OnInit: [EmbedApp::init] )]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    /// It's possible to load embed resources automatically
    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("ICE"))]
    ice_cursor: nwg::Cursor,

    #[nwg_control(size: (280, 25), position: (10, 10))]
    name_edit: nwg::TextInput,

    #[nwg_control(position: (130, 110), size: (35, 35))]
    embed_bitmap: nwg::ImageFrame,

    #[nwg_control(size: (280, 60), position: (10, 40))]
    #[nwg_events( OnButtonClick: [EmbedApp::say_hello], OnMouseMove: [EmbedApp::set_cursor], OnMousePress: [EmbedApp::set_cursor] )]
    hello_button: nwg::Button,

    mem_font: RefCell<Option<nwg::MemFont>>,
}

impl EmbedApp {

    fn init(&self) {
        let em = &self.embed;
        self.name_edit.set_text(&em.string(0).unwrap());
        self.hello_button.set_text(&em.string(1).unwrap());

        self.window.set_text(&em.string(2).unwrap());
        self.window.set_icon(em.icon_str("TEST", None).as_ref());

        self.embed_bitmap.set_bitmap(em.bitmap_str("BALL", None).as_ref());

        // Load a custom font from embed resource
        let mem_font = unsafe {
            let rc = self.embed.raw_str("INDIE", nwg::RawResourceType::Other("FONTFILE")).unwrap();
            nwg::Font::add_memory_font(rc.as_mut_slice()).unwrap()
        };

        let mut font = Default::default();
        nwg::Font::builder()
            .family("Indie Flower")
            .size(30)
            .build(&mut font)
            .expect("Failed to build font");
        self.hello_button.set_font(Some(&font));

        *self.mem_font.borrow_mut() = Some(mem_font);
    }

    fn say_hello(&self) {
        nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }

    fn set_cursor(&self) {
        nwg::GlobalCursor::set(&self.ice_cursor);
    }
    
    fn say_goodbye(&self) {
        nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();

        let mut font = self.mem_font.borrow_mut();
        if let Some(font) = font.take() {
            nwg::Font::remove_memory_font(font);
        }
    }
    

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = EmbedApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
