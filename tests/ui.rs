#![allow(unused_variables)]

extern crate native_windows_gui as nwg;
use nwg::events::EventCallback;
use nwg::constants as nwgc;
use nwg::actions::helper as nwga;
use nwg::actions::{Action, ActionReturn};

fn create_controls(ui: &mut nwg::Ui<&'static str>) {
    let main_window = nwg::controls::Window {
        caption: "Hello".to_string(),
        size: (500, 180),
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

    let move_btn = nwg::controls::Button {
        text: "Move me!".to_string(),
        size: (100, 50),
        position: (10, 65),
        parent: "MainWindow"
    };

    let resize_btn = nwg::controls::Button {
        text: "Resize me!".to_string(),
        size: (100, 50),
        position: (10, 120),
        parent: "MainWindow"
    };

    ui.new_control("MainWindow", main_window).unwrap();
    ui.new_control("HelloBtn", hello_btn).unwrap();
    ui.new_control("MoveBtn", move_btn).unwrap();
    ui.new_control("ResizeBtn", resize_btn).unwrap();
}

#[test]
fn test_ui() {
    let mut ui: nwg::Ui<&'static str> = nwg::Ui::new();

    create_controls(&mut ui);

    ui.bind("MainWindow", EventCallback::MouseUp(Box::new(|ui, caller, x, y, btn, modifiers| {
        assert!("MainWindow" == *caller);
        println!("Left mouse button pressed: {:?}", (btn & nwgc::BTN_MOUSE_LEFT) != 0 );
        println!("Ctrl pressed: {:?}", (modifiers & nwgc::MOD_MOUSE_CTRL) != 0 );
        println!("Mouse position: {:?} {:?}", x, y);
    })));

    ui.bind("HelloBtn", EventCallback::ButtonClick(Box::new(|ui, caller| {
        assert!("HelloBtn" == *caller);
        ui.exec("MainWindow", nwga::message("Hello", "Hello World!", 0)).unwrap();

        if let ActionReturn::Text(old_text) = ui.exec(caller, Action::GetText).unwrap() {
            let new_text = Box::new(*old_text + "!");
            ui.exec(caller, Action::SetText(new_text)).unwrap();
        }
    })));

    ui.bind("MoveBtn", EventCallback::ButtonClick(Box::new(|ui, caller| {
        if let ActionReturn::Position(x,y) = ui.exec("MoveBtn", Action::GetPosition).unwrap() {
            if x == 10 {
                ui.exec(caller, Action::SetPosition(380, 65)).unwrap();
            } else {
                ui.exec(caller, Action::SetPosition(10, 65)).unwrap();
            }
        }
    })));

    ui.bind("ResizeBtn", EventCallback::ButtonClick(Box::new(|ui, caller| {
        if let ActionReturn::Size(w,h) = ui.exec("ResizeBtn", Action::GetSize).unwrap() {
            if w == 100 {
                ui.exec(caller, Action::SetSize(480, 50)).unwrap();
            } else {
                ui.exec(caller, Action::SetSize(100, 50)).unwrap();
            }
        }
    })));

    nwg::dispatch_events();
}