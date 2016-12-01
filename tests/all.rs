#![allow(unused_must_use)]
#![allow(unused_variables)]

extern crate native_windows_gui as nwg;

use nwg::*;

fn setup_ui() -> Ui<u64> { Ui::new().unwrap() }

#[test]
fn test_ui_new() {
    match Ui::<u64>::new() {
        Ok(ui) => ui,
        Err(e) => panic!("Ui creation failed: {:?}", e)
    };
}

#[test]
fn test_ui_pack_user_value() {
    let mut ui = setup_ui();

    ui.pack_value(1000, "Test");
    ui.pack_value(1001, vec![5u32, 6, 7]);

    assert!(!ui.has_id(&1000), "ID 1000 was found in id before commit");
    assert!(!ui.has_id(&1001), "ID 1001 was found in id before commit");

    ui.commit().expect("Commit was not successful");
    
    assert!(ui.has_id(&1000), "ID 1000 was not found in id after commit");
    assert!(ui.has_id(&1001), "ID 1000 was not found in id after commit");

    ui.pack_value(1002, "Test");
    ui.pack_value(1001, 5u32);

    assert!(ui.commit().is_err(), "Commit was successful");

    assert!(ui.has_id(&1002), "ID 1002 was not found in id after commit");

    let x = ui.get::<Vec<u16>>(&1001);
    let y = ui.get::<&'static str>(&1000);
    panic!("{:?} {:?}", x, y);
}