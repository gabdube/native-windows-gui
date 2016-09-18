extern crate native_windows_gui as nwg;
use nwg::events::EventCallback;
use nwg::constants as nwgc;

#[test]
fn test_ui() {
    let mut ui: nwg::Ui<&'static str> = nwg::Ui::new();

    let main_window = nwg::controls::Window {
        caption: "Test".to_string(),
        size: (500, 500),
        position: (100, 100),
        visible: true,
        resizable: false
    };

    ui.new_control("MainWindow", main_window).unwrap();

    ui.bind("MainWindow", EventCallback::MouseUp(Box::new(|ui, caller, x, y, btn, modifiers|{
        println!("Left mouse button pressed: {:?}", (btn & nwgc::BTN_MOUSE_LEFT) != 0 );
        println!("Ctrl pressed: {:?}", (modifiers & nwgc::MOD_MOUSE_CTRL) != 0 );
        println!("Mouse position: {:?} {:?}", x, y);
    })));

    nwg::dispatch_events();
}