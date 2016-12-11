extern crate native_windows_gui as nwg;

use nwg::{Ui, WindowT, MenuT, MenuItemT, dispatch_events, exit as nwg_exit};
use nwg::{Event};

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
    
    app.pack_control(&"MainWindow", default_window());
    app.pack_control(&"FileMenu", MenuT{ text: "&File", parent: "MainWindow" });
    app.pack_control(&"QuitItem", MenuItemT{ text: "&Quit", parent: "FileMenu" });
    
    app.bind(&"QuitItem", &"Quit", Event::Clicked, |_,_,_,_|{
        nwg_exit()
    });

    app.commit().expect("Commit failed");

    dispatch_events();
}