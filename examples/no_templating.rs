/**
    Simple example on how to use nwg without the template system.  
    Unless your UI is built dynamically, the usage of the macro templates is highly recommended
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Ui, Error, Event, simple_message, fatal_message, dispatch_events};

pub fn setup_ui(ui: &Ui<&'static str>) -> Result<(), Error> {

    let f1 = nwg::FontT { 
        family: "Arial", size: 27,
        weight: nwg::constants::FONT_WEIGHT_NORMAL,
        decoration: nwg::constants::FONT_DECO_NORMAL,
    };

    let f2 = nwg::FontT { 
        family: "Arial", size: 17,
        weight: nwg::constants::FONT_WEIGHT_NORMAL,
        decoration: nwg::constants::FONT_DECO_NORMAL,
    };

    let window = nwg::WindowT {
        title: "No template", 
        position: (100, 100), size: (280, 105), 
        resizable: false, visible: true, disabled: false, 
        exit_on_close: true
    };

    let label = nwg::LabelT { 
        text: "Your Name: ",
        position: (5,15), size: (80, 25), 
        visible: true, disabled: false, 
        align: nwg::constants::HTextAlign::Left,
        parent: "MainWindow", font: Some("TextFont")
    };

    let tedit = nwg::TextInputT::<_, &'static str, _> { 
        text: "",
        position: (85,13), size: (185,22), 
        visible: true, disabled: false, readonly: false, password: false,
        limit: 32_767, placeholder: None,
        parent: "MainWindow", font: Some("TextFont")
    };

    let hellbtn = nwg::ButtonT { 
        text: "Hello World!", 
        position: (5, 45), size: (270, 50), 
        visible: true, disabled: false, 
        parent: "MainWindow", font: Some("MainFont")
    };

    ui.pack_resource(&"MainFont", f1);
    ui.pack_resource(&"TextFont", f2); 

    ui.pack_control(&"MainWindow", window);
    ui.pack_control(&"Label1", label);
    ui.pack_control(&"YourName", tedit);
    ui.pack_control(&"HelloButton", hellbtn);

    ui.bind(&"HelloButton", &"SaySomething", Event::Click, |ui,_,_,_| {
        let your_name = nwg_get!(ui; ("YourName", nwg::TextInput));
        simple_message("Hello", &format!("Hello {}!", your_name.get_text()) );
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
