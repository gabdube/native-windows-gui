extern crate native_windows_gui as nwg;

use nwg::{Ui, Event, WindowT, MenuT, MenuItemT, ButtonT, FontT, ListBoxT, dispatch_events, exit as nwg_exit};
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
    app.pack_resource(&"MainFont", FontT{ family: "Calibri", size: 20, weight: FONT_WEIGHT_BLACK, decoration: FONT_DECO_ITALIC });

    // Pack the controls
    app.pack_control(&"MainWindow", default_window());
    app.pack_control(&"FileMenu", MenuT{ text: "&File", parent: "MainWindow" });
    app.pack_control(&"QuitItem", MenuItemT{ text: "&Quit", parent: "FileMenu" });
    app.pack_control(&"TestButton", ButtonT{text: "TEST", position:(10, 10), size: (100, 30), visible: true, disabled: false, parent: "MainWindow", font: Some("MainFont") });

    app.pack_control(&"TestList", ListBoxT{
        collection: vec!["Test1", "Test2", "Test3", "Test1", "Test2", "Test3"],
        position:(10, 50),
        size: (100, 60),
        visible: true,
        disabled: false,
        parent: "MainWindow",
        font: None 
    });

    // Bind the events
    app.bind(&"QuitItem", &"Quit", Event::Clicked, |_,_,_,_|{
        nwg_exit()
    });

    // Execute the commands
    app.commit().expect("Commit failed");

    {
        let x = app.get::<::nwg::Window>(&"MainWindow").unwrap();
        println!("{:?}", x.get_position());
    
        let y = app.get::<::nwg::ListBox<&'static str>>(&"TestList").unwrap();
        y.set_size(200, 52);
        println!("{:?}", y.get_size());
    }

    dispatch_events();

    // Although not required, its always better to explicitly destroy the controls first.
    // This will also unpack the window children
    app.unpack(&"MainWindow");
    app.commit().expect("Commit failed");
}