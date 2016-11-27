extern crate native_windows_gui as nwg;

use nwg::*;

#[test]
fn test_ui() {
    let ui: Ui<u64> = match Ui::new() {
        Ok(ui) => ui,
        Err(e) => panic!("Ui creation failed: {:?}", e)
    };
}