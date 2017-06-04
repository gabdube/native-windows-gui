/**
    Simple example on how to use the nwg template system.
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Ui, simple_message, fatal_message, dispatch_events};
use nwg::events as nwge;

nwg_template!(
    head: setup_ui<&'static str>,
    controls: [
        ("MainWindow", nwg_window!( title="Template Example"; size=(280, 105) )),
        ("Label1", nwg_label!( parent="MainWindow"; text="Your Name: "; position=(5,15); size=(80, 25); font=Some("TextFont") )),
        ("YourName", nwg_textinput!( parent="MainWindow"; position=(85,13); size=(185,22); font=Some("TextFont") )),
        ("HelloButton", nwg_button!( parent="MainWindow"; text="Hello World!"; position=(5, 45); size=(270, 50); font=Some("MainFont") ))
    ];
    events: [
        ("HelloButton", "SaySomething", nwge::button::Click, |ui,_,_,_| {
            let your_name = nwg_get!(ui; ("YourName", nwg::TextInput));
            simple_message("Hello", &format!("Hello {}!", your_name.get_text()) );
        })
    ];
    resources: [
        ("MainFont", nwg_font!(family="Arial"; size=27)),
        ("TextFont", nwg_font!(family="Arial"; size=17))
    ];
    values: []
);

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
