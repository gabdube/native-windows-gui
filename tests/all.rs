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
    ui.pack();
    ui.commit().expect("Commit was not successful");
}