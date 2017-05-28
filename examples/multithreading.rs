/**
    A simple example demonstrating multithreading. Two simple ui and allow the user to pause the thread.
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Ui, fatal_message, dispatch_events};
use nwg::events as nwge;
use std::thread;
use std::time::Duration;

nwg_template!(
    head: setup_sleep_window<&'static str>,
    controls: [
        ("Sleep", nwg_window!( title="Sleep"; size=(200, 200); position=(200, 200) )),
        ("SleepButton", nwg_button!( parent="Sleep"; text="SLEEP"; size=(200, 200); position=(0, 0) ))
    ];
    events: [
        ("SleepButton", "Sleep", nwge::button::Click, |ui,_,_,_| {
            let btn = nwg_get!(ui; ("SleepButton", nwg::Button));
            btn.set_text("SLEEPING!");
            thread::sleep(Duration::from_millis(5000));
            btn.set_text("SLEEP");
        })
    ];
    resources: [];
    values: []
);

nwg_template!(
    head: setup_test_window<&'static str>,
    controls: [
        ("Test", nwg_window!( title="Test"; size=(200, 200); position=(420, 200) ))
    ];
    events: [];
    resources: [];
    values: []
);

fn main() {
    // Create the main window on the current thread
    let app: Ui<&'static str>;
    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    if let Err(e) = setup_test_window(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    // Create another ui on a new thread
    let t = thread::spawn(||{
        let app2: Ui<&'static str>;
        match Ui::new() {
            Ok(_app2) => { app2 = _app2; },
            Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
        }

        if let Err(e) = setup_sleep_window(&app2) {
            fatal_message("Fatal Error", &format!("{:?}", e));
        }

        dispatch_events();
    });
    
    // Wait for both event loop to finish
    dispatch_events();
    t.join().unwrap();
}
