use nwd::NwgPartial;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};
use super::controls::{LabeledField, LeftButtonList};

const LABEL_WIDTH: f32 = 130.0;

#[derive(Default)]
#[derive(NwgPartial)]
pub struct ProjectSettingsUi {

    #[nwg_layout(
        auto_spacing: Some(0),
        flex_direction: FlexDirection::Column,
        min_size: Size { width: Points(300.0), height: Points(300.0) },
    )]
    layout: nwg::FlexboxLayout,

    #[nwg_control]
    tt: nwg::Tooltip,
    
    #[nwg_control]
    pub on_settings_saved: nwg::CustomEvent,

    #[nwg_control]
    pub on_settings_refresh: nwg::CustomEvent,

    #[nwg_control(text: "Crate Name:", disabled: true, label_width: LABEL_WIDTH, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    pub crate_name: LabeledField,

    #[nwg_control(text: "NWG version:", label_width: LABEL_WIDTH, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    pub nwg_version: LabeledField,

    #[nwg_control(text: "NWD version:", label_width: LABEL_WIDTH, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    pub nwd_version: LabeledField,

    #[nwg_control(text: "Resource file:", label_width: LABEL_WIDTH, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    pub res_file: LabeledField,

    #[nwg_control(text: "Resources path:", label_width: LABEL_WIDTH, background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(45.0) })]
    pub res_path: LabeledField,

    #[nwg_control(buttons: vec!["Update", "Refresh"], width: 100.0, background_color: Some([255,255,255]))]
    #[nwg_events(
        (buttons[0], OnButtonClick): [ProjectSettingsUi::save_settings],
        (buttons[1], OnButtonClick): [ProjectSettingsUi::refesh_settings]
    )]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Points(55.0) })]
    pub save_btn: LeftButtonList

}

impl ProjectSettingsUi {

    pub(super) fn init(&self) {
        let tt = &self.tt;

        let tt0 = "The crate name";
        tt.register(&self.crate_name.label, tt0);
        tt.register(&self.crate_name.input, tt0);

        let tt1 = "Relative path to the windows resource file (*.rc) for the project. Can be left empty";
        tt.register(&self.res_file.label, tt1);
        tt.register(&self.res_file.input, tt1);

        let tt2 = "Relative path to a folder containing the resources used by the GUI. Can be left empty";
        tt.register(&self.res_path.label, tt2);
        tt.register(&self.res_path.input, tt2);

        let tt3 = "Current native-windows-gui version defined in Cargo.toml";
        tt.register(&self.nwg_version.label, tt3);
        tt.register(&self.nwg_version.input, tt3);

        let tt4 = "Current native-windows-derive version defined in Cargo.toml";
        tt.register(&self.nwd_version.label, tt4);
        tt.register(&self.nwd_version.input, tt4);
    }

    pub fn save_settings(&self) {
        self.on_settings_saved.trigger();
    }

    pub fn refesh_settings(&self) {
        self.on_settings_refresh.trigger();
    }

    pub fn enable_ui(&self, enable: bool) {
        self.nwg_version.set_enabled(enable);
        self.nwd_version.set_enabled(enable);
        self.res_file.set_enabled(enable);
        self.res_path.set_enabled(enable);
        self.save_btn.set_enabled(enable);
    }

    pub fn reload(&self, project: &crate::Project) {
        self.crate_name.set_text(&project.name());
        self.nwg_version.set_text(&project.nwg_version());
        self.nwd_version.set_text(&project.nwd_version());
    }

    pub fn clear(&self) {
        self.crate_name.set_text("");
        self.nwg_version.set_text("");
        self.nwd_version.set_text("");
        self.res_file.set_text("");
        self.res_path.set_text("");
    }

}
