use nwd::NwgPartial;
use std::{cell::Cell, rc::Rc};
use super::gui_shared::set_cursor;


#[derive(Default)]
#[derive(NwgPartial)]
pub struct ObjectInspector {
    // Saved width to keep then restore when the main window is too small
    pub(super) user_width: Cell<u32>,

    // Saved control list height
    control_list_height: Rc<Cell<u32>>,

    #[nwg_resource(source_system: Some(nwg::OemCursor::SizeWE))]
    cursor_size_we: nwg::Cursor,

    #[nwg_resource(source_system: Some(nwg::OemCursor::Normal))]
    cursor_default: nwg::Cursor,

    #[nwg_control(size: (275, 0))]
    #[nwg_events(
        OnResize: [ObjectInspector::resize],
        OnMouseMove: [ObjectInspector::update_cursor],
        OnMouseLeft: [ObjectInspector::clear_cursor],
    )]
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
        let control_list_height = self.control_list_height.get();
        let width_with_offset = w - ((left_offset as u32) * 2);

        let mut top_offset = 5;

        self.controls_label.set_position(left_offset, top_offset);
        self.controls_label.set_size(width_with_offset, label_height);
        top_offset += label_height as i32;

        self.control_list.set_position(left_offset, top_offset);
        self.control_list.set_size(width_with_offset, control_list_height);
        top_offset += (5 + control_list_height) as i32;

        self.properties_label.set_position(left_offset, top_offset);
        self.properties_label.set_size(width_with_offset, label_height);
        top_offset += label_height as i32;

        self.properties_list.set_position(left_offset, top_offset);
        self.properties_list.set_size(width_with_offset, h - (top_offset as u32) - 10);
    }

    fn update_cursor(&self) {
        let (x, _y) = nwg::GlobalCursor::local_position(&self.container_frame, None);
        
        /*
        // Safe "recommended" (by the windows docs) way to set the cursor. Flickers a bit.
        if x < 4 {
            nwg::GlobalCursor::set(&self.size_we);
        }
        */

        // This tells winapi to send a `Event::OnMouseLeave` when the cursor will leave the control
        nwg::GlobalCursor::track_mouse_leaving(&self.container_frame);

        // The ~cool~ way to set a cursor in winapi that never flickers
        // Sadly (for me, the dev of NWG) it's impossible to safely wrap because this
        // sets the cursor ON ALL INSTANCES OF THE SAME CONTROL. Goddamit Windows.
        if x < 5 {
            set_cursor(&self.container_frame.handle, &self.cursor_size_we);
        } else {
            set_cursor(&self.container_frame.handle, &self.cursor_default);
        }
    }

    /// This gets triggered by the `track_mouse_leaving` call in `update_cursor`
    /// Sets the cursor back to normal so that the other frame works correctly
    fn clear_cursor(&self) {
        set_cursor(&self.container_frame.handle, &self.cursor_default);
    }

}
