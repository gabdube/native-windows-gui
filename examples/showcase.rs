extern crate native_windows_gui as nwg;

use nwg::{Ui, Event, WindowT, MenuT, MenuItemT, ButtonT, FontT, dispatch_events, exit as nwg_exit};
use nwg::constants::{FONT_WEIGHT_BLACK, FONT_DECO_ITALIC};

pub fn default_window() -> WindowT<&'static str> {
    WindowT { 
        title: "NWG Showcase",
        position: (100, 100), size: (500, 400),
        resizable: true, visible: true,
        disabled: false, exit_on_close: true 
    }
}

pub fn main() {
    let app: Ui<&'static str> = Ui::new().expect("Failed to initialize the Ui");
    
    // Always create the resources first because they will be used in the controls.
    app.pack_resource(&"MainFont", FontT{ family: "Calibri", size: 20, weight: FONT_WEIGHT_BLACK, decoration: 0 });

    // Pack the controls
    app.pack_control(&"MainWindow", default_window());
    app.pack_control(&"FileMenu", MenuT{ text: "&File", parent: "MainWindow" });
    app.pack_control(&"QuitItem", MenuItemT{ text: "&Quit", parent: "FileMenu" });
    app.pack_control(&"TestButton", ButtonT{text: "TEST", position:(10, 10), size: (100, 30), visible: true, disabled: false, parent: "MainWindow", font: Some("MainFont") });

    // Bind the events
    app.bind(&"QuitItem", &"Quit", Event::Clicked, |_,_,_,_|{
        nwg_exit()
    });

    // Execute the commands
    app.commit().expect("Commit failed");

    dispatch_events();
}