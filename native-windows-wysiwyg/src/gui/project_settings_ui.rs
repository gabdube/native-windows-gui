use nwd::NwgPartial;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};
use super::controls::{LabeledField, LeftButton};

#[derive(Default)]
#[derive(NwgPartial)]
pub struct ProjectSettingsUi {

    #[nwg_layout(auto_spacing: Some(0), flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    #[nwg_control]
    pub on_settings_saved: nwg::CustomEvent,

    #[nwg_control(text: "Crate Name:", disabled: true, label_width: 130.0, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    crate_name: LabeledField,

    #[nwg_control(text: "Gui module:", label_width: 130.0, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    gui_module: LabeledField,

    #[nwg_control(text: "Resource file:", label_width: 130.0, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    res_file: LabeledField,

    #[nwg_control(text: "Resources path:", label_width: 130.0, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    res_path: LabeledField,

    #[nwg_control(text: "Save", width: 100.0, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(55.0) })]
    save_btn: LeftButton

}

impl ProjectSettingsUi {

    pub(super) fn init(&self) {
        
    }

}
