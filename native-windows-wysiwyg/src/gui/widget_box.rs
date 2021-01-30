use nwd::NwgPartial;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};


#[derive(Default)]
#[derive(NwgPartial)]
pub struct WidgetBox {
    #[nwg_layout(auto_size: false, auto_spacing: None, flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Percent(1.0) })]
    pub(super) widgets_tree: nwg::TreeView,
}

impl WidgetBox {

    pub(super) fn init(&self) {
        self.load_widgets();
    }

    pub(super) fn load_widgets(&self) {
        let last = nwg::TreeInsert::Last;
        let tree = &self.widgets_tree;

        let controls = tree.insert_item("Controls", None, last);
        
        let top_level = tree.insert_item("Top level", Some(&controls), last);
        tree.insert_item("Window", Some(&top_level), last);
        tree.insert_item("Message window", Some(&top_level), last);
        tree.insert_item("Extern canvas (window)", Some(&top_level), last);

        let buttons = tree.insert_item("Buttons", Some(&controls), last);
        tree.insert_item("Button", Some(&buttons), last);
        tree.insert_item("Check box", Some(&buttons), last);
        tree.insert_item("Radio button", Some(&buttons), last);

        let display = tree.insert_item("Display", Some(&controls), last);
        tree.insert_item("Label", Some(&display), last);
        tree.insert_item("Rich label", Some(&display), last);
        tree.insert_item("Image frame", Some(&display), last);
        tree.insert_item("Status bar", Some(&display), last);
        tree.insert_item("Progress bar", Some(&display), last);

        let inputs = tree.insert_item("Inputs", Some(&controls), last);
        tree.insert_item("Rich text box", Some(&inputs), last);
        tree.insert_item("Text box", Some(&inputs), last);
        tree.insert_item("Text input", Some(&inputs), last);
        tree.insert_item("Number select", Some(&inputs), last);
        tree.insert_item("Date picker", Some(&inputs), last);
        
        let collections = tree.insert_item("Collections", Some(&controls), last);
        tree.insert_item("Combobox", Some(&collections), last);
        tree.insert_item("List box", Some(&collections), last);
        tree.insert_item("List view", Some(&collections), last);
        tree.insert_item("Tree view", Some(&collections), last);

        let containers = tree.insert_item("Containers", Some(&controls), last);
        tree.insert_item("Frame", Some(&containers), last);
        tree.insert_item("Tab container", Some(&containers), last);
        tree.insert_item("Tab", Some(&containers), last);
        tree.insert_item("Extern canvas (child)", Some(&containers), last);
    
        let triggers = tree.insert_item("Triggers", Some(&controls), last);
        tree.insert_item("Notice", Some(&triggers), last);
        tree.insert_item("Timer", Some(&triggers), last);
        tree.insert_item("Tray notification", Some(&triggers), last);
        tree.insert_item("Tooltip", Some(&triggers), last);

        let other = tree.insert_item("Other", Some(&controls), last);
        tree.insert_item("Track bar", Some(&other), last);

        let _custom = tree.insert_item("Partials", Some(&controls), last);

        for item in tree.iter() {
            tree.set_expand_state(&item, nwg::ExpandState::Expand);
        }

        tree.ensure_visible(&controls);
    }


}
