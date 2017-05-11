/**
    Example that show every control implemented in NWG
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Ui, Event, EventArgs, dispatch_events, exit as nwg_exit};
use nwg::constants::{FONT_WEIGHT_BLACK, FONT_DECO_ITALIC, CheckState, FileDialogAction, HTextAlign, PickerDate};

nwg_template!(
    head: setup_ui<&'static str>,
    controls: [
        ("MainWindow", nwg_window!(title="Nwg Showcase"; position=(100, 100); size=(500, 400))),
        ("FileMenu", nwg_menu!(parent="MainWindow"; text="&File")),
        ("TestSubmenu1", nwg_menu!(parent="FileMenu"; text="&Submenu")),
        ("TestDisabledSubmenu", nwg_menu!(parent="FileMenu"; text="Disabled Submenu"; disabled=true)),
        ("TestSubmenu2", nwg_menu!(parent="TestSubmenu1"; text="&Another submenu")),
        ("NestedAction", nwg_menuitem!(parent="TestSubmenu2"; text="H&ello")),
        ("DisabledNestedAction", nwg_menuitem!(parent="TestSubmenu2"; text="Disabled"; disabled=true)),
        ("S1", nwg_separator!(parent="FileMenu")),
        ("QuitItem", nwg_menuitem!(parent="FileMenu"; text="&Quit")),
        ("WindowAction", nwg_menuitem!(parent="MainWindow"; text="&Action")),
        ("TimerButton", nwg_button!(parent="MainWindow"; text="Start timer"; position=(10,85); size=(100, 30); font=Some("Font2"))),
        ("TimerLabel", nwg_label!(parent="MainWindow"; text="Time elapsed: 0 seconds"; position=(120, 90); size=(200, 25); font=Some("Font2"))),
        ("Timer", nwg_timer!(interval=500)),
        ("FileDialogButton", nwg_button!(parent="MainWindow"; text="Browse File"; position=(10,120); size=(100, 30); font=Some("Font1"))),
        ("FilePathInput", nwg_textinput!(parent="MainWindow"; position=(120, 125); size=(300, 24); readonly=true; font=Some("Font1"))),
        ("FileDialog", nwg_filedialog!(parent=Some("MainWindow"); action=FileDialogAction::Open; filters=Some("Test(*.txt;*.rs)|Any(*.*)"))),
        ("NameList", nwg_listbox!(parent="MainWindow"; position=(10, 10); size=(100, 60); collection=vec!["A Listbox", "Jimmy", "Sam", "Coconut", "Waldo", "David", "John"])),
        ("HappyCheckBox", nwg_checkbox!(parent="MainWindow"; text="I am happy"; position=(120, 10); size=(110, 30); checkstate=CheckState::Checked; font=Some("Font1"))),
        ("TriCheckBox", nwg_checkbox!(parent="MainWindow"; text="Three states"; position=(240, 10); size=(110, 30); tristate=true; checkstate=CheckState::Indeterminate; font=Some("Font1"))),
        ("CatRadio", nwg_radiobutton!(parent="MainWindow"; text="I have a cat"; position=(120, 50); size=(110, 30); checkstate=CheckState::Checked; font=Some("Font1"))),
        ("DogRadio", nwg_radiobutton!(parent="MainWindow"; text="I have a dog"; position=(240, 50); size=(110, 30); font=Some("Font1"))),
        ("YesNoGroup", nwg_groupbox!(parent="MainWindow"; text="Choose one"; position=(360, 40); size=(130, 80);  align=HTextAlign::Center; font=Some("Font1") )),
        ("YesRadio", nwg_radiobutton!(parent="YesNoGroup"; text="Yes"; position=(10, 20); size=(110, 30); font=Some("Font1"))),
        ("NoRadio", nwg_radiobutton!(parent="YesNoGroup"; text="No"; position=(10, 45); size=(110, 30); font=Some("Font1"))),
        ("SchoolSupplyComboBox ", nwg_combobox!(parent="MainWindow"; position=(360, 10); size=(130, 30); placeholder=Some("Choose plz"); font=Some("Font1"); collection=vec!["Pencil", "Eraser", "Scissor", "Calculator", "Notebook"])),
        ("RandomStuffLabel", nwg_label!(parent="MainWindow"; text="Write some notes in here:"; position=(10, 160); size=(180, 25); font=Some("Font1"))),
        ("RandomStuffTextBox", nwg_textbox!(parent="MainWindow"; position=(10, 185); size=(200, 60); scrollbars=(false, true))),
        ("InstallCatLabel", nwg_label!(parent="MainWindow"; text="Installing cat.exe ..."; position=(230, 160); size=(180, 25); font=Some("Font1") )),
        ("CatProgress", nwg_progressbar!(parent="MainWindow"; position=(230, 190); size=(240, 25); range=(0, 100); value=85)),
        ("DatePicker", nwg_datepicker!(parent="MainWindow"; value=Some(PickerDate{year:2016, month:12, day:1}); format=" dd MMMM yyyy"; position=(230, 220); size=(240, 25); font=Some("Font1")))
    ];
    events: [
        ("RandomStuffTextBox", "AllSystemEvents", Event::Raw, |_,_,_,args| {
            let (msg, w) = match args {
                &EventArgs::Raw(msg, w, _) => (msg, w),
                _ => unreachable!()
            };

            if msg == 0x0100 { // WM_KEYDOWN
                println!("The virtual key {:?} was pressed", w);
            }
        }),

        ("NestedAction", "SayHello", Event::Triggered, |_,_,_,_| {
            nwg::simple_message("Hello", "Hello World!");
        }),

        ("QuitItem", "Quit", Event::Triggered, |_,_,_,_| {
            nwg_exit()
        }),

        ("TimerButton", "Start Timer", Event::Click, |app,_,_,_|{
            let (mut timer, btn) = nwg_get_mut!(app; [
                ("Timer", nwg::Timer),
                ("TimerButton", nwg::Button)
            ]);

            if timer.running() {
                btn.set_text("Start Timer");
                timer.stop();
            } else {
                btn.set_text("Stop Timer");
                timer.start();
            }
        }),

        ("Timer", "UpdateLabel", Event::Tick, |app,_,_,args|{
            let label = nwg_get!(app; ("TimerLabel", nwg::Label));
            let elapsed = match args { 
                &EventArgs::Tick(ref d) => d,
                _ => unreachable!()
            };

            label.set_text(format!("Time elapsed: {:?} seconds", elapsed.as_secs()).as_ref());
        }),

        ("FileDialogButton", "ChooseFile", Event::Click, |app,_,_,_|{
            let (dialog, file_path) = nwg_get_mut!(app; [
                ("FileDialog", nwg::FileDialog),
                ("FilePathInput", nwg::TextInput)
            ]);

            if dialog.run() {
                file_path.set_text(&dialog.get_selected_item().unwrap());
            }
        }),

        ("DatePicker", "DD", Event::DateChanged, |app,_,_,_| {
            println!("{:?}", nwg_get!(app; ("DatePicker", nwg::DatePicker)).get_value());
        })
    ];
    resources: [
        ("Font1", nwg_font!(family="Calibri"; size=20 )),
        ("Font2", nwg_font!(family="Arial"; size=17; weight=FONT_WEIGHT_BLACK; decoration=FONT_DECO_ITALIC))
    ];
    values: []
);

pub fn main() {
    let app: Ui<&'static str> = Ui::new().expect("Failed to initialize the Ui");
    
    // Pack the control in the application
    if let Err(e) = setup_ui(&app) {
        panic!("Commit failed: {:?}", e);
    }

    // Dispatch the events until the user quits
    dispatch_events();
}