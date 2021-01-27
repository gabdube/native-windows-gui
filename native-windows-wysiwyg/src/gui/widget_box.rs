use nwd::NwgPartial;
use std::cell::Cell;
use super::gui_shared::set_cursor;

#[derive(Default)]
#[derive(NwgPartial)]
pub struct WidgetBox {
    // Saved width to keep then restore when the main window is too small
    pub(super) user_width: Cell<u32>,

    #[nwg_resource(source_system: Some(nwg::OemCursor::SizeWE))]
    cursor_size_we: nwg::Cursor,

    #[nwg_resource(source_system: Some(nwg::OemCursor::Normal))]
    cursor_default: nwg::Cursor,

    #[nwg_control(size: (275, 0))]
    #[nwg_events(
        OnMouseMove: [WidgetBox::update_cursor],
        OnMouseLeft: [WidgetBox::clear_cursor],
    )]
    pub(super) container_frame: nwg::Frame,

    #[nwg_layout(parent: container_frame, spacing: 0, margin: [0,5,0,0])]
    layout: nwg::GridLayout,

    #[nwg_control(parent: container_frame)]
    #[nwg_layout_item(layout: layout, col:0, row:0)]
    pub(super) widgets_tree: nwg::TreeView,
}

impl WidgetBox {

    pub(super) fn init(&self) {
        self.user_width.set(275);
        self.load_widgets();
    }

    pub(super) fn load_widgets(&self) {
        let tree = &self.widgets_tree;

        let controls = tree.insert_item("Controls", None, nwg::TreeInsert::Last);
        
        let top_level = tree.insert_item("Top level", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Window", Some(&top_level), nwg::TreeInsert::Last);
        tree.insert_item("Message window", Some(&top_level), nwg::TreeInsert::Last);
        tree.insert_item("Extern canvas (window)", Some(&top_level), nwg::TreeInsert::Last);

        let buttons = tree.insert_item("Buttons", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Button", Some(&buttons), nwg::TreeInsert::Last);
        tree.insert_item("Check box", Some(&buttons), nwg::TreeInsert::Last);
        tree.insert_item("Radio button", Some(&buttons), nwg::TreeInsert::Last);

        let display = tree.insert_item("Display", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Label", Some(&display), nwg::TreeInsert::Last);
        tree.insert_item("Rich label", Some(&display), nwg::TreeInsert::Last);
        tree.insert_item("Image frame", Some(&display), nwg::TreeInsert::Last);
        tree.insert_item("Status bar", Some(&display), nwg::TreeInsert::Last);
        tree.insert_item("Progress bar", Some(&display), nwg::TreeInsert::Last);

        let inputs = tree.insert_item("Inputs", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Rich text box", Some(&inputs), nwg::TreeInsert::Last);
        tree.insert_item("Text box", Some(&inputs), nwg::TreeInsert::Last);
        tree.insert_item("Text input", Some(&inputs), nwg::TreeInsert::Last);
        tree.insert_item("Number select", Some(&inputs), nwg::TreeInsert::Last);
        tree.insert_item("Date picker", Some(&inputs), nwg::TreeInsert::Last);
        
        let collections = tree.insert_item("Collections", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Combobox", Some(&collections), nwg::TreeInsert::Last);
        tree.insert_item("List box", Some(&collections), nwg::TreeInsert::Last);
        tree.insert_item("List view", Some(&collections), nwg::TreeInsert::Last);
        tree.insert_item("Tree view", Some(&collections), nwg::TreeInsert::Last);

        let containers = tree.insert_item("Containers", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Frame", Some(&containers), nwg::TreeInsert::Last);
        tree.insert_item("Tab container", Some(&containers), nwg::TreeInsert::Last);
        tree.insert_item("Tab", Some(&containers), nwg::TreeInsert::Last);
        tree.insert_item("Extern canvas (child)", Some(&containers), nwg::TreeInsert::Last);
    
        let triggers = tree.insert_item("Triggers", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Notice", Some(&triggers), nwg::TreeInsert::Last);
        tree.insert_item("Timer", Some(&triggers), nwg::TreeInsert::Last);
        tree.insert_item("Tray notification", Some(&triggers), nwg::TreeInsert::Last);
        tree.insert_item("Tooltip", Some(&triggers), nwg::TreeInsert::Last);

        let other = tree.insert_item("Other", Some(&controls), nwg::TreeInsert::Last);
        tree.insert_item("Track bar", Some(&other), nwg::TreeInsert::Last);

        let _custom = tree.insert_item("Partials", Some(&controls), nwg::TreeInsert::Last);

        for item in tree.iter() {
            tree.set_expand_state(&item, nwg::ExpandState::Expand);
        }

        tree.ensure_visible(&controls);
        tree.set_enabled(false);
    }


    fn update_cursor(&self) {
        let (x, _y) = nwg::GlobalCursor::local_position(&self.container_frame, None);
        let (w, _h) = self.container_frame.size();
        
        /*
        // Safe "recommended" (by the windows docs) way to set the cursor. Flickers a bit.
        if x < 4 {
            nwg::GlobalCursor::set(&self.size_we);
        }
        */

        // This tells winapi to send a `Event::OnMouseLeave` when the cursor will leave the control
        nwg::GlobalCursor::track_mouse_leaving(&self.container_frame);

        if x > (w as i32 - 5) {
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
