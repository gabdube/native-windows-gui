use nwd::NwgPartial;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};


#[derive(Default)]
#[derive(NwgPartial)]
pub struct ObjectInspector {

    #[nwg_layout(flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    //
    // Current controls tree
    //
    #[nwg_control(text: "Control Tree", background_color: Some([255,255,255]), v_align: nwg::VTextAlign::Top)]
    #[nwg_layout_item(layout: layout, flex_shrink: 0.0, size: Size { width: Percent(1.0), height: Points(15.0) })]
    controls_label: nwg::Label,

    #[nwg_control(
        list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::AUTO_COLUMN_SIZE | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Percent(1.0) })]
    pub control_list: nwg::ListView,

    //
    // Selected control properties
    //
    #[nwg_control(text: "Control Properties", background_color: Some([255,255,255]), v_align: nwg::VTextAlign::Top)]
    #[nwg_layout_item(layout: layout, flex_shrink: 0.0, size: Size { width: Percent(1.0), height: Points(15.0) })]
    properties_label: nwg::Label,

    #[nwg_control(
        list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::AUTO_COLUMN_SIZE | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Percent(1.0) })]
    pub properties_list: nwg::ListView,
}

impl ObjectInspector {

    pub(super) fn init(&self) {
        let ctrl = &self.control_list;
        ctrl.set_headers_enabled(true);
        ctrl.insert_column("Name");
        ctrl.insert_column("Type");
    
        let prop = &self.properties_list;
        prop.set_headers_enabled(true);
        prop.insert_column("Name");
        prop.insert_column("Value");

    }

    pub fn enable_ui(&self, enable: bool) {
        // Listview needs to be enabled in order for the background color to be changed
        match enable {
            false => {
                self.control_list.set_background_color(220, 220, 220);
                self.properties_list.set_background_color(220, 220, 220);
                self.control_list.set_enabled(enable);
                self.properties_list.set_enabled(enable);
            },
            true => {
                self.control_list.set_enabled(enable);
                self.properties_list.set_enabled(enable);
                self.control_list.set_background_color(255, 255, 255);
                self.properties_list.set_background_color(255, 255, 255);
            }
        }
    }

}
