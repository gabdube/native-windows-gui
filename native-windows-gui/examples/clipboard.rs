/*!
    An example that shows how to handle custom clipboard operations

    Requires the following features: `cargo run --example clipboard --features "textbox listbox menu cursor clipboard"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct ClipboardCustom {
    #[nwg_control(size: (400, 300), position: (300, 300), title: "Clipboard Handling")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    grid: nwg::GridLayout,

    #[nwg_control(text:"Hello\r\nWorld\r\nClipboad\r\nExample", flags: "VISIBLE|AUTOVSCROLL|AUTOHSCROLL", focus: true,)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    sample_text: nwg::TextBox,

    #[nwg_control]
    #[nwg_events(MousePressRightUp: [ClipboardCustom::show_menu])]
    #[nwg_layout_item(layout: grid, col: 1, row: 0)]
    listbox: nwg::ListBox<String>,

    #[nwg_control(popup: true)]
    listbox_menu: nwg::Menu,

    #[nwg_control(parent: listbox_menu, text: "Paste Items")]
    #[nwg_events(OnMenuItemSelected: [ClipboardCustom::paste_items])]
    listbox_menu_paste: nwg::MenuItem,

    #[nwg_control(parent: listbox_menu, text: "Copy Items")]
    #[nwg_events(OnMenuItemSelected: [ClipboardCustom::copy_items])]
    listbox_menu_copy: nwg::MenuItem,
}

impl ClipboardCustom {

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.listbox_menu.popup(x, y)
    }

    fn paste_items(&self) {
        self.listbox.clear();

        if let Some(text) = nwg::Clipboard::data_text(&self.window) {
            for line in text.split("\r\n") {
                self.listbox.push(line.into());
            }
        }
    }

    fn copy_items(&self) {
        let mut copy_data = String::with_capacity(30);
        let col = self.listbox.collection();
        for item in col.iter() {
            copy_data.push_str(&item);
            copy_data.push_str("\r\n");
        }

        nwg::Clipboard::set_data_text(&self.window, &copy_data);
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = ClipboardCustom::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

