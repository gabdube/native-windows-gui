extern crate native_windows_gui as nwg;

use nwg::{Ui, WindowT, dispatch_events};

pub fn default_window() -> WindowT<&'static str> {
    WindowT { 
        title: "NWG Showcase",
        position: (100, 100), size: (500, 400),
        resizable: true, visible: true,
        disabled: false, exit_on_close: true 
    }
}

pub fn main() {
    let mut app: Ui<&'static str> = Ui::new().expect("Failed to initialize the Ui");
    
    app.pack_control("MainWindow", default_window());

    app.commit().expect("Commit failed");

    dispatch_events();
}