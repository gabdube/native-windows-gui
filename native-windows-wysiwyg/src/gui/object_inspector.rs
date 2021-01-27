use nwd::NwgPartial;
use std::cell::Cell;


#[derive(Default)]
#[derive(NwgPartial)]
pub struct ObjectInspector {
    // Saved width to keep then restore when the main window is too small
    pub(super) user_width: Cell<u32>,

    // Saved control list height
    control_list_height: Cell<u32>,

    #[nwg_control(size: (275, 0))]
    #[nwg_events(OnResize: [ObjectInspector::resize])]
    pub(super) container_frame: nwg::Frame,

    #[nwg_control(parent: container_frame, text: "Controls", v_align: nwg::VTextAlign::Top)]
    controls_label: nwg::Label,

    #[nwg_control(
        parent: container_frame,
        list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::AUTO_COLUMN_SIZE | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    control_list: nwg::ListView,

    #[nwg_control(parent: container_frame, text: "Properties", v_align: nwg::VTextAlign::Top)]
    properties_label: nwg::Label,

    #[nwg_control(
        parent: container_frame,
        list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::AUTO_COLUMN_SIZE | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    properties_list: nwg::ListView,
}

impl ObjectInspector {

    pub(super) fn init(&self) {
        self.user_width.set(275);
        self.control_list_height.set(300);

        let ctrl = &self.control_list;
        ctrl.set_headers_enabled(true);
        ctrl.insert_column("Name");
        ctrl.insert_column("Type");
        ctrl.set_enabled(false);

        let prop = &self.properties_list;
        prop.set_headers_enabled(true);
        prop.insert_column("Name");
        prop.insert_column("Value");
        prop.set_enabled(false);
    }

    fn resize(&self) {
        let (w, h) = self.container_frame.size();
        let left_offset = 5i32;
        let label_height = 25;
        let width_with_offset = w - ((left_offset as u32) * 2);

        let mut top_offset = 5;

        self.controls_label.set_position(left_offset, top_offset);
        self.controls_label.set_size(width_with_offset, label_height);
        top_offset += label_height as i32;

        self.control_list.set_position(left_offset, top_offset);
        self.control_list.set_size(width_with_offset, self.control_list_height.get());
        top_offset += (5 + self.control_list_height.get()) as i32;

        self.properties_label.set_position(left_offset, top_offset);
        self.properties_label.set_size(width_with_offset, label_height);
        top_offset += label_height as i32;

        self.properties_list.set_position(left_offset, top_offset);
        self.properties_list.set_size(width_with_offset, h - (top_offset as u32) - 10);
    }

}
