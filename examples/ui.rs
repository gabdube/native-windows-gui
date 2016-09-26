#![allow(unused_variables)]

extern crate native_windows_gui as nwg;
use nwg::events::EventCallback;
use nwg::constants as nwgc;
use nwg::actions::helper as nwga;
use nwg::actions::{Action, ActionReturn};

fn create_controls(ui: &mut nwg::Ui<&'static str>) {
    let main_window = nwg::controls::Window {
        caption: "Test".to_string(),
        size: (500, 290),
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

    let parent_btn = nwg::controls::Button {
        text: "Who is my parent?".to_string(),
        size: (480, 50),
        position: (10, 175),
        parent: "MainWindow"
    };

    let checkbox = nwg::controls::CheckBox {
        text: "A checkbox".to_string(),
        size: (100, 50),
        position: (10, 230),
        parent: "MainWindow",
        tristate: true
    };

    ui.new_control("MainWindow", main_window).unwrap();
    ui.new_control("HelloBtn", hello_btn).unwrap();
    ui.new_control("MoveBtn", move_btn).unwrap();
    ui.new_control("ResizeBtn", resize_btn).unwrap();
    ui.new_control("ParentBtn", parent_btn).unwrap();
    ui.new_control("CheckBox", checkbox).unwrap();
}

fn main() {
    let mut ui: nwg::Ui<&'static str> = nwg::Ui::new();

    create_controls(&mut ui);

    ui.bind("MainWindow", EventCallback::MouseUp(Box::new(|ui, caller, x, y, btn, modifiers| {
        assert!("MainWindow" == *caller);
        println!("Left mouse button pressed: {:?}", (btn & nwgc::BTN_MOUSE_LEFT) != 0 );
        println!("Ctrl pressed: {:?}", (modifiers & nwgc::MOD_MOUSE_CTRL) != 0 );
        println!("Mouse position: {:?} {:?}", x, y);
    }))).unwrap();

    ui.bind("HelloBtn", EventCallback::ButtonClick(Box::new(|ui, caller| {
        assert!("HelloBtn" == *caller);
        ui.exec("MainWindow", nwga::message("Hello", "Hello World!", 0)).unwrap();

        if let ActionReturn::Text(old_text) = ui.exec(caller, Action::GetText).unwrap() {
            let new_text = Box::new(*old_text + "!");
            ui.exec(caller, Action::SetText(new_text)).unwrap();
        }
    }))).unwrap();

    ui.bind("MoveBtn", EventCallback::ButtonClick(Box::new(|ui, caller| {
        if let ActionReturn::Position(x,y) = ui.exec("MoveBtn", Action::GetPosition).unwrap() {
            if x == 10 {
                ui.exec(caller, Action::SetPosition(390, 65)).unwrap();
            } else {
                ui.exec(caller, Action::SetPosition(10, 65)).unwrap();
            }
        }
    }))).unwrap();

    ui.bind("ResizeBtn", EventCallback::ButtonClick(Box::new(|ui, caller| {
        if let ActionReturn::Size(w,h) = ui.exec("ResizeBtn", Action::GetSize).unwrap() {
            if w == 100 {
                ui.exec(caller, Action::SetSize(480, 50)).unwrap();
            } else {
                ui.exec(caller, Action::SetSize(100, 50)).unwrap();
            }
        }
    }))).unwrap();

    ui.bind("ParentBtn", EventCallback::ButtonClick(Box::new(|ui, caller| {
        if let ActionReturn::Parent(parent) = ui.exec(caller, Action::GetParent).unwrap() {
            let parent = parent.unwrap();
            assert!("MainWindow" == parent);
            let parent = format!("{:?} is my parent!", parent);
            ui.exec(caller, Action::SetText(Box::new(parent))).unwrap();
        }
    }))).unwrap();

    nwg::dispatch_events();
}