/**
    Simple example on how to use nwg without the template system.
    Unless your UI is built dynamically, the usage of the macro templates is highly recommended
*/

extern crate native_windows_gui as nwg;

use nwg::{Ui, Error, simple_message, fatal_message, dispatch_events};
use nwg::events as nwge;

pub fn setup_ui(ui: &Ui<&'static str>) -> Result<(), Error> {

    // nwg_font!(family="Arial"; size=27)
    let f1 = nwg::FontT {
        family: "Arial", size: 27,
        weight: nwg::constants::FONT_WEIGHT_NORMAL,
        decoration: nwg::constants::FONT_DECO_NORMAL,
    };

    // nwg_font!(family="Arial"; size=17)
    let f2 = nwg::FontT {
        family: "Arial", size: 17,
        weight: nwg::constants::FONT_WEIGHT_NORMAL,
        decoration: nwg::constants::FONT_DECO_NORMAL,
    };

    // nwg_window!( title="Template Example"; size=(280, 105))
    let window = nwg::WindowT {
        title: "No template",
        position: (100, 100), size: (280, 105),
        resizable: false, visible: true, disabled: false,
        exit_on_close: true
    };

    // nwg_label!( parent="MainWindow"; [...] font=Some("TextFont") )
    let label = nwg::LabelT {
        text: "Your Name: ",
        position: (5,15), size: (80, 25),
        visible: true, disabled: false,
        align: nwg::constants::HTextAlign::Left,
        parent: "MainWindow", font: Some("TextFont")
    };

    // nwg_textinput!( parent="MainWindow"; [..] font=Some("TextFont") )
    let tedit = nwg::TextInputT::<_, &'static str, _> {
        text: "",
        position: (85,13), size: (185,22),
        visible: true, disabled: false, readonly: false, password: false,
        limit: 32_767, placeholder: None,
        parent: "MainWindow", font: Some("TextFont")
    };

    // nwg_button!( parent="MainWindow"; [..] font=Some("MainFont") )
    let hellbtn = nwg::ButtonT {
        text: "Hello World!",
        position: (5, 45), size: (270, 50),
        visible: true, disabled: false,
        parent: "MainWindow", font: Some("MainFont")
    };

    // resources: 
    ui.pack_resource(&"MainFont", f1);
    ui.pack_resource(&"TextFont", f2);

    // controls:
    ui.pack_control(&"MainWindow", window);
    ui.pack_control(&"Label1", label);
    ui.pack_control(&"YourName", tedit);
    ui.pack_control(&"HelloButton", hellbtn);

    // events:
    ui.bind(&"HelloButton", &"SaySomething", nwge::button::Click, |ui,_,_,_| {
        if let Ok(your_name) = ui.get::<nwg::TextInput>(&"YourName") {
            simple_message("Hello", &format!("Hello {}!", your_name.get_text()) );
        } else {
            panic!()
        }
    });

    ui.commit()
}

fn main() {
    let app: Ui<&'static str>;

    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    if let Err(e) = setup_ui(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    dispatch_events();
}
