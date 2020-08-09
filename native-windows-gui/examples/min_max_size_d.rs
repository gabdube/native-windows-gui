/*!
    Shows you how to set the maximum/minimum size of a window
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct ResizeApp {
    #[nwg_control(size: (500, 310), position: (300, 300), title: "Resize example")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnMinMaxInfo: [ResizeApp::resize(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 5)]
    grid: nwg::GridLayout,

    // Maximized size
    #[nwg_control(text: "Maximized size:")]
    #[nwg_layout_item(layout: grid, row: 1)]
    label1: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 1, col: 1)]
    edit_maxed_size_width: nwg::TextInput,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 1, col: 2)]
    edit_maxed_size_height: nwg::TextInput,

    // Maximized pos
    #[nwg_control(text: "Maximized position:")]
    #[nwg_layout_item(layout: grid, row: 2)]
    label2: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 2, col: 1)]
    edit_maxed_pos_x: nwg::TextInput,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 2, col: 2)]
    edit_maxed_pos_y: nwg::TextInput,

    // Max size
    #[nwg_control(text: "Max size:")]
    #[nwg_layout_item(layout: grid, row: 3)]
    label3: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 3, col: 1)]
    edit_max_size_width: nwg::TextInput,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 3, col: 2)]
    edit_max_size_height: nwg::TextInput,

    // Min size
    #[nwg_control(text: "Min size:")]
    #[nwg_layout_item(layout: grid, row: 4)]
    label4: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 4, col: 1)]
    edit_min_size_width: nwg::TextInput,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, row: 4, col: 2)]
    edit_min_size_height: nwg::TextInput,

    // Column label
    #[nwg_control(text: "Width", h_align: nwg::HTextAlign::Center)]
    #[nwg_layout_item(layout: grid, row: 0, col: 1)]
    label5: nwg::Label,

    #[nwg_control(text: "Height", h_align: nwg::HTextAlign::Center)]
    #[nwg_layout_item(layout: grid, row: 0, col: 2)]
    label6: nwg::Label

}

impl ResizeApp {

    fn resize(&self, data: &nwg::EventData) {
        let data = data.on_min_max();
        let [old_maximized_width, old_maximized_height] = data.maximized_size();
        let [old_maximized_x, old_maximized_y] = data.maximized_pos();
        let [old_max_width, old_max_height] = data.max_size();
        let [old_min_width, old_min_height] = data.min_size();


        // Maximized size
        let maximized_width = self.edit_maxed_size_width.text().parse::<i32>().unwrap_or(9999);
        let maximized_height = self.edit_maxed_size_height.text().parse::<i32>().unwrap_or(9999);

        if maximized_width == 9999  { self.edit_maxed_size_width.set_text(&format!("{}", old_maximized_width)); }
        if maximized_height == 9999 { self.edit_maxed_size_height.set_text(&format!("{}", old_maximized_height)); }
        if maximized_width != 9999 && maximized_height != 9999 {
            data.set_maximized_size(maximized_width, maximized_height);
        }

        // Maximized position
        let maximized_x = self.edit_maxed_pos_x.text().parse::<i32>().unwrap_or(9999);
        let maximized_y = self.edit_maxed_pos_y.text().parse::<i32>().unwrap_or(9999);

        if maximized_x == 9999 { self.edit_maxed_pos_x.set_text(&format!("{}", old_maximized_x)); }
        if maximized_y == 9999 { self.edit_maxed_pos_y.set_text(&format!("{}", old_maximized_y)); }
        if maximized_x != 9999 && maximized_y != 9999 {
            data.set_maximized_pos(maximized_x, maximized_y);
        }

        // Max size
        let max_width = self.edit_max_size_width.text().parse::<i32>().unwrap_or(9999);
        let max_height = self.edit_max_size_height.text().parse::<i32>().unwrap_or(9999);

        if max_width == 9999  { self.edit_max_size_width.set_text(&format!("{}", old_max_width)); }
        if max_height == 9999 { self.edit_max_size_height.set_text(&format!("{}", old_max_height)); }
        if max_width != 9999 && max_height != 9999 {
            data.set_max_size(max_width, max_height);
        }

        // Min size
        let min_width = self.edit_min_size_width.text().parse::<i32>().unwrap_or(9999);
        let min_height = self.edit_min_size_height.text().parse::<i32>().unwrap_or(9999);

        if min_width == 9999  { self.edit_min_size_width.set_text(&format!("{}", old_min_width)); }
        if min_height == 9999 { self.edit_min_size_height.set_text(&format!("{}", old_min_height)); }
        if min_width != 9999 && min_height != 9999 {
            data.set_min_size(min_width, min_height);
        }

    }


}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = ResizeApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
