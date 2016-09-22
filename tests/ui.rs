#![allow(unused_variables)]

extern crate native_windows_gui as nwg;
use nwg::events::EventCallback;
use nwg::constants as nwgc;
use nwg::actions::helper as nwga;
use nwg::actions::{Action, ActionReturn};

#[test]
fn test_ui() {
    let mut ui: nwg::Ui<&'static str> = nwg::Ui::new();

    let main_window = nwg::controls::Window {
        caption: "Hello".to_string(),
        size: (500, 70),
        position: (100, 100),
        visible: true,
        resizable: false
    };

    let hello_btn = nwg::controls::Button {
        text: "Say hello!".to_string(),
        size: (480, 50),
        position: (10, 10),
        parent: "MainWindow"
    };

    ui.new_control("MainWindow", main_window).unwrap();
    ui.new_control("HelloBtn", hello_btn).unwrap();

    ui.bind("MainWindow", EventCallback::MouseUp(Box::new(|ui, caller, x, y, btn, modifiers| {
        println!("Caller: {:?}", caller);
        println!("Left mouse button pressed: {:?}", (btn & nwgc::BTN_MOUSE_LEFT) != 0 );
        println!("Ctrl pressed: {:?}", (modifiers & nwgc::MOD_MOUSE_CTRL) != 0 );
        println!("Mouse position: {:?} {:?}", x, y);
    })));

    ui.bind("HelloBtn", EventCallback::ButtonClick(Box::new(|ui, caller| {
        println!("Caller: {:?}", caller);
        ui.exec("MainWindow", nwga::message("Hello", "Hello World!", 0)).unwrap();

        if let ActionReturn::Text(old_text) = ui.exec(caller, Action::GetText).unwrap() {
            let new_text = Box::new(*old_text + "!");
            ui.exec(caller, Action::SetText(new_text)).unwrap();
        }
    })));

    nwg::dispatch_events();
}