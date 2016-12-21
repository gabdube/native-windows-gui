extern crate native_windows_gui as nwg;

use nwg::{Ui, Event, WindowT, MenuT, MenuItemT, ButtonT, FontT, ListBoxT, CheckBoxT, 
  RadioButtonT, LabelT, dispatch_events, exit as nwg_exit};
use nwg::constants::{FONT_WEIGHT_BLACK, FONT_DECO_ITALIC, FONT_DECO_NORMAL, FONT_WEIGHT_NORMAL,
  CheckState, HTextAlign};

pub fn default_window() -> WindowT<&'static str> {
    WindowT { 
        title: "NWG Showcase",
        position: (100, 100), size: (500, 400),
        resizable: true, visible: true,
        disabled: false, exit_on_close: true 
    }
}


fn setup_controls(app: &Ui<&'static str>) {

    app.pack_control(&"MainWindow", default_window());
    app.pack_control(&"FileMenu", MenuT{ text: "&File", parent: "MainWindow" });
    app.pack_control(&"QuitItem", MenuItemT{ text: "&Quit", parent: "FileMenu" });

    app.pack_control(&"TestButton", ButtonT{
        text: "A button",
        position:(10, 10), size: (100, 30),
        visible: true, disabled: false,
        parent: "MainWindow",
        font: Some("Font2") 
    });

    app.pack_control(&"TestCheckBox1", CheckBoxT{
        text: "A checkbox",
        position:(120, 10), size: (110, 30),
        visible: true, disabled: false,
        parent: "MainWindow",
        checkstate: CheckState::Checked,
        tristate: false,
        font: Some("Font1") 
    });

    app.pack_control(&"TestCheckBox2", CheckBoxT{
        text: "A trisate checkbox",
        position:(240, 10), size: (150, 30),
        visible: true, disabled: false,
        parent: "MainWindow",
        checkstate: CheckState::Indeterminate,
        tristate: true,
        font: Some("Font1") 
    });

    app.pack_control(&"TestRad1", RadioButtonT{
        text: "A radiobutton",
        position:(120, 50), size: (110, 30),
        visible: true, disabled: false,
        parent: "MainWindow",
        checkstate: CheckState::Checked,
        font: Some("Font1") 
    });

    app.pack_control(&"TestRad2", RadioButtonT{
        text: "A radiobutton",
        position:(240, 50), size: (110, 30),
        visible: true, disabled: false,
        parent: "MainWindow",
        checkstate: CheckState::Unchecked,
        font: Some("Font1") 
    });

    app.pack_control(&"TestList", ListBoxT{
        collection: vec!["A Listbox", "Jimmy", "Sam", "Coconut", "Waldo", "David", "John"],
        position:(10, 50), size: (100, 60),
        visible: true, disabled: false, readonly: false, multi_select: false,
        parent: "MainWindow",
        font: None 
    });

    app.pack_control(&"", LabelT{
       text: "Current time: 00:00",
        position:(120, 90), size: (150, 100),
        visible: true, disabled: false,
        align: HTextAlign::Left,
        parent: "MainWindow",
        font: Some("Font1")
    });
}


fn setup_callbacks(app: &Ui<&'static str>) {

    app.bind(&"TestButton", &"...", Event::Click, |app,_,_,_|{
        println!("{:?}", app.get::<nwg::CheckBox>(&"TestCheckBox2").unwrap().get_checkstate());
    });

    app.bind(&"QuitItem", &"Quit", Event::Click, |_,_,_,_|{
        nwg_exit()
    });

}

pub fn main() {
    let app: Ui<&'static str> = Ui::new().expect("Failed to initialize the Ui");
    
    // Always create the resources first because they will be used in the controls.
    app.pack_resource(&"Font1", FontT{ family: "Calibri", size: 20, weight: FONT_WEIGHT_NORMAL, decoration: FONT_DECO_NORMAL });
    app.pack_resource(&"Font2", FontT{ family: "Calibri", size: 20, weight: FONT_WEIGHT_BLACK, decoration: FONT_DECO_ITALIC });

    /// Pack the control in the application
    setup_controls(&app);

    /// Setup the callbacks
    setup_callbacks(&app);

    // Execute the commands
    app.commit().expect("Commit failed");

    // Dispatch the events until the user quits
    dispatch_events();
}