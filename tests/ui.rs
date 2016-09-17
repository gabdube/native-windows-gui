extern crate native_windows_gui as nwg;


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

}