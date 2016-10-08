#![allow(unused_variables)]

extern crate native_windows_gui as nwg;
use nwg::constants::Error;
use nwg::events::{EventCallback, Event};
use nwg::actions::Action;

fn setup_window(ui: &mut nwg::Ui<&'static str>) {
    let main_window = nwg::controls::Window {
        caption: "Test".to_string(),
        size: (200, 200),
        position: (100, 100),
        visible: false,
        resizable: false
    };

    let r = ui.new_control("MainWindow", main_window);
    assert!(r.is_ok());
}

#[test]
fn buttons() {
    let mut ui: nwg::Ui<&'static str> = nwg::Ui::new();
    setup_window(&mut ui);

    let window = nwg::controls::Window {
        caption: "Test".to_string(),
        size: (200, 200),
        position: (100, 100),
        visible: false,
        resizable: false
    };

    // Assigned control names should be unique
    let r = ui.new_control("MainWindow", window);
    assert!(r.is_err());
    assert!(r.err().unwrap() == Error::CONTROL_EXISTS);

    // Cannot bind event to unused names
    let r = ui.bind("Haha", "...", EventCallback::Click(Box::new(|ui, caller| {} )));
    assert!(r.is_err());
    assert!(r.err().unwrap() == Error::CONTROL_NOT_FOUND);

    // Cannot bind unsupported callbacks
    let r = ui.bind("MainWindow", "...", EventCallback::Click(Box::new(|ui, caller| {} )));
    assert!(r.is_err());
    assert!(r.err().unwrap() == Error::CALLBACK_NOT_SUPPORTED);

    // Cannot execute action on unused names
    let r = ui.exec("Hoho", Action::None);
    assert!(r.is_err());
    assert!(r.err().unwrap() == Error::CONTROL_NOT_FOUND);

    // Cannot bind two events of the same type  with the same name
    ui.bind("MainWindow", "...", EventCallback::Removed( Box::new(|ui, caller|{})) ).unwrap();
    let r = ui.bind("MainWindow", "...", EventCallback::Removed( Box::new(|ui, caller|{})) );
    assert!(r.is_err());
    assert!(r.err().unwrap() == Error::CALLBACK_ID_EXISTS);

    // Unbind and rebind should succeed.
    ui.unbind("MainWindow", "...", Event::Removed).unwrap();
    ui.bind("MainWindow", "...", EventCallback::Removed( Box::new(|ui, caller|{})) ).unwrap();
}