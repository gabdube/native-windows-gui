/**
    How to use the nwg template system.
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Event, Ui, simple_message, fatal_message, dispatch_events};

nwg_template!(
    head: setup_ui<&'static str>,
    controls: [
        ("MainWindow", nwg_window!( title="Template Example"; size=(280, 60) )),
        ("HelloButton", nwg_button!( parent="MainWindow"; text="Hello World!"; position=(5, 5); size=(270, 50); font=Some("MainFont") ))
    ];
    events: [
        ("HelloButton", "SaySomething", Event::Click, |_,_,_,_| {
            simple_message("Hello", "Hello World!");
        })
    ];
    resources: [
        ("MainFont", nwg_font!(family="Arial"; size=27))
    ];
    values: [
        ("CustomValue", 1234),
        ("HelloBuddy", Vec::<u32>::new())
    ]
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
