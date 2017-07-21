/**
    Example that show every control implemented in NWG
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Ui, EventArgs, dispatch_events, exit as nwg_exit};
use nwg::constants::{FONT_WEIGHT_BLACK, FONT_DECO_ITALIC, CheckState, FileDialogAction, HTextAlign, PickerDate, ImageType};
use nwg::events as nwge;    

static EMBED_BMP01: &'static [u8; 27702]  = include_bytes!("../img/rust-logo.bmp");

nwg_template!(
    head: setup_ui<&'static str>,
    controls: [
        // Window
        ("MainWindow", nwg_window!(title="Nwg Showcase"; position=(100, 100); size=(500, 400); icon=Some("RustLogoIcon"))),
        
        // Menus
        ("FileMenu", nwg_menu!(parent="MainWindow"; text="&File")),
        ("TestSubmenu1", nwg_menu!(parent="FileMenu"; text="&Submenu")),
        ("TestDisabledSubmenu", nwg_menu!(parent="FileMenu"; text="Disabled Submenu"; disabled=true)),
        ("TestSubmenu2", nwg_menu!(parent="TestSubmenu1"; text="&Another submenu")),
        ("NestedAction", nwg_menuitem!(parent="TestSubmenu2"; text="H&ello")),
        ("DisabledNestedAction", nwg_menuitem!(parent="TestSubmenu2"; text="Disabled"; disabled=true)),
        ("S1", nwg_separator!(parent="FileMenu")),
        ("QuitItem", nwg_menuitem!(parent="FileMenu"; text="&Quit")),
        ("WindowAction", nwg_menuitem!(parent="MainWindow"; text="&Action")),

        // Context Menu
        ("Context", nwg_contextmenu!()),
        ("Action1", nwg_menuitem!(parent="Context"; text="Action 1")),
        ("Action2", nwg_menuitem!(parent="Context"; text="Action 2")),
        ("S2", nwg_separator!(parent="Context")),
        ("TestSubmenu3", nwg_menu!(parent="Context"; text="&Submenu")),
        ("Action3", nwg_menuitem!(parent="TestSubmenu3"; text="SayHello")),
        ("Action4", nwg_menuitem!(parent="TestSubmenu3"; text="Disabled :("; disabled=true)),
        
        // Tabs
        ("TabView", nwg_tabsview!(parent="MainWindow"; position=(5, 5); size=(490, 370))),
        ("Tab1", nwg_tab!(parent="TabView"; text="Simple controls")),
        ("Tab2", nwg_tab!(parent="TabView"; text="Images And Trees")),

        // Timers
        ("TimerButton", nwg_button!(parent="Tab1"; text="Start timer"; position=(10,5); size=(100, 30); font=Some("Font2"))),
        ("TimerLabel", nwg_label!(parent="Tab1"; text="Time elapsed: 0 seconds"; position=(120, 10); size=(200, 25); font=Some("Font2"))),
        ("Timer", nwg_timer!(interval=500)),

        // Combobox
        ("SchoolSupplyComboBox ", 
            nwg_combobox!(
                parent="Tab1";
                position=(325, 10); size=(145, 30);
                placeholder=Some("Choose plz"); font=Some("Font1");
                collection=vec!["Pencil", "Eraser", "Scissor", "Calculator", "Notebook"])),

        // File dialog
        ("FileDialogButton", nwg_button!(parent="Tab1"; text="Browse File"; position=(10,40); size=(100, 30); font=Some("Font1"))),
        ("FilePathInput", nwg_textinput!(parent="Tab1"; position=(120, 45); size=(350, 24); readonly=true; font=Some("Font1"))),
        ("FileDialog", nwg_filedialog!(parent=Some("MainWindow"); action=FileDialogAction::Open; filters=Some("Test(*.txt;*.rs)|Any(*.*)"))),

        // List
        ("NameList", nwg_listbox!(
            parent="Tab1"; 
            position=(10, 75); size=(150, 100);
            collection=vec!["A Listbox", "Jimmy", "Sam", "Coconut", "Waldo", "David", "John"])),
        
        // Checkbox & radios
        ("HappyCheckBox", nwg_checkbox!(parent="Tab1"; text="I am happy"; position=(170, 75); size=(110, 30); checkstate=CheckState::Checked; font=Some("Font1"))),
        ("TriCheckBox", nwg_checkbox!(parent="Tab1"; text="Three states"; position=(290, 75); size=(110, 30); tristate=true; checkstate=CheckState::Indeterminate; font=Some("Font1"))),
        ("CatRadio", nwg_radiobutton!(parent="Tab1"; text="I have a cat"; position=(170, 105); size=(110, 30); checkstate=CheckState::Checked; font=Some("Font1"))),
        ("DogRadio", nwg_radiobutton!(parent="Tab1"; text="I have a dog"; position=(290, 105); size=(110, 30); font=Some("Font1"))),
        
        // Groupbox
        ("YesNoGroup", nwg_groupbox!(parent="Tab1"; text="Choose one"; position=(10, 185); size=(150, 60);  align=HTextAlign::Center; font=Some("Font1") )),
        ("YesRadio", nwg_radiobutton!(parent="YesNoGroup"; text="Blue"; position=(10, 20); size=(50, 30); font=Some("Font1"))),
        ("NoRadio", nwg_radiobutton!(parent="YesNoGroup"; text="Red"; position=(90, 20); size=(50, 30); font=Some("Font1"))),
        
        // Textbox
        ("RandomStuffLabel", nwg_label!(parent="Tab1"; text="Write your notes here:"; position=(170, 135); size=(180, 25); font=Some("Font1"))),
        ("RandomStuffTextBox", nwg_textbox!(parent="Tab1"; position=(170, 160); size=(290, 100); scrollbars=(false, true))),
        
        // Progress bar
        ("InstallCatLabel", nwg_label!(parent="Tab1"; text="Installing cat.exe ..."; position=(10, 265); size=(150, 25); font=Some("Font1") )),
        ("CatProgress", nwg_progressbar!(parent="Tab1"; position=(10, 290); size=(200, 25); range=(0, 100); value=85)),
        
        // Date picker
        ("DateLabel", nwg_label!(parent="Tab1"; text="Select the birth date of your cat"; position=(220, 265); size=(220, 25); font=Some("Font1") )),
        ("DatePicker", nwg_datepicker!(parent="Tab1"; value=Some(PickerDate{year:2016, month:12, day:1}); format=" dd MMMM yyyy"; position=(220, 290); size=(240, 25); font=Some("Font1"))),
        
        // ImageFrame
        ("RustLogoFrame", nwg_image_frame!(parent="Tab2"; image=Some("RustLogo"); position=(190, 215); size=(100,100))),
        ("OtherFrame", nwg_image_frame!(parent="Tab2"; image=Some("RustLogo"); position=(295, 215); size=(100,100))),
        
        // TreeView
        ("TreeSelected", nwg_textinput!(parent="Tab2"; position=(190, 5); size=(280, 22); placeholder=Some("Selected Item Text"); font=Some("Font1") )),
        ("TreeView", nwg_treeview!(parent="Tab2"; position=(5, 5); size=(180, 315))),
        ("Tree_Root", nwg_treeview_item!(parent="TreeView"; text="Department")),
        ("TreeDirector", nwg_treeview_item!(parent="Tree_Root"; text="Director & Associate")),
        ("TreeBob", nwg_treeview_item!(parent="TreeDirector"; text="Bob Stalone")),
        ("TreeJob", nwg_treeview_item!(parent="TreeDirector"; text="Job Drake")),
        ("TreeManagement", nwg_treeview_item!(parent="Tree_Root"; text="Management")),
        ("TreeSally", nwg_treeview_item!(parent="TreeManagement"; text="Sally Foo")),
        ("TreeTI", nwg_treeview_item!(parent="Tree_Root"; text="TI"))
    ];
    events: [
        ("RandomStuffTextBox", "AllSystemEvents", nwge::Any, |_,_,_,args| {
            let (msg, w) = match args {
                &EventArgs::Raw(msg, w, _) => (msg, w),
                _ => unreachable!()
            };

            if msg == 0x0100 { // WM_KEYDOWN
                println!("The virtual key {:?} was pressed", w);
            }
        }),

        ("TreeView", "ItemSelected", nwge::treeview::ItemChanged, |app,_,_,_| {
            let (tree, text) = nwg_get!(app; [
                ("TreeView", nwg::TreeView),
                ("TreeSelected", nwg::TextInput)
            ]);

            if let Some(item_id) = tree.get_selected_item(app) {
                let item = nwg_get!(app; (item_id, nwg::TreeViewItem));
                text.set_text( &item.get_text() );
            }
        }),

        ("TimerButton", "Start Timer", nwge::button::Click, |app,_,_,_|{
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

        ("Timer", "UpdateLabel", nwge::timer::Tick, |app,_,_,_|{
            let (label, timer) = nwg_get!(app; [("TimerLabel", nwg::Label), ("Timer", nwg::Timer)]);
            label.set_text(format!("Time elapsed: {:?} seconds", timer.elapsed().as_secs()).as_ref());
        }),

        ("FileDialogButton", "ChooseFile", nwge::button::Click, |app,_,_,_|{
            let (dialog, file_path) = nwg_get_mut!(app; [
                ("FileDialog", nwg::FileDialog),
                ("FilePathInput", nwg::TextInput)
            ]);

            if dialog.run() {
                file_path.set_text(&dialog.get_selected_item().unwrap());
            }
        }),

        ("DatePicker", "DD", nwge::datepicker::DateChanged, |app,_,_,_| {
            println!("{:?}", nwg_get!(app; ("DatePicker", nwg::DatePicker)).get_value());
        }),

        ("RustLogoFrame", "Logo", nwge::image_frame::Click, |app,_,_,_| {
            let img = nwg_get!(app; ("RustLogoFrame", nwg::ImageFrame));
            if let Some("RustLogo") = img.get_image(app) {
                img.set_image(app, Some(&"RustMascot")).unwrap();
            } else {
                img.set_image(app, Some(&"RustLogo")).unwrap();
            }
        }),

        ("MainWindow", "context", nwge::MouseDown, |app, _, _, args| {
            match args {
                &EventArgs::MouseClick{ref btn, pos: _} => {
                    if let nwg::constants::MouseButton::Right = *btn {
                        // WATCH OUT! The position param of MouseClick use local coordinate, but the `pop_at` method requires global coordinates
                        // To make sure the menu appears at the right place, always use the `Cursor::get_position` method.
                        let (x, y) = nwg::Cursor::get_position();   
                        nwg_get!(app; ("Context", nwg::ContextMenu)).pop_at(x, y);
                    }
                },
                _ => unreachable!()
            }
        }),

        ("NestedAction", "SayHello", nwge::menu::Triggered, |_,_,_,_| { nwg::simple_message("Hello", "Hello World!");  }),
        ("Action3", "SayHello", nwge::menu::Triggered, |_,_,_,_| { nwg::simple_message("Hello", "Hello World!"); }),
        ("QuitItem", "Quit", nwge::menu::Triggered, |_,_,_,_| { nwg_exit() })

    ];
    resources: [
        ("Font1", nwg_font!(family="Calibri"; size=17 )),
        ("Font2", nwg_font!(family="Arial"; size=17; weight=FONT_WEIGHT_BLACK; decoration=FONT_DECO_ITALIC)),
        ("RustLogo", nwg_image!(source="img\\rust-logo.bmp"; image_type=ImageType::Bitmap)), // Make sure to use '\\' and not '/'
        ("RustMascot", nwg_image!(source="img\\rust-mascot.bmp"; image_type=ImageType::Bitmap; size=(100, 100))), // Make sure to use '\\' and not '/'
        ("RustLogoIcon", nwg_image!(source="img\\rust-logo.ico"; image_type=ImageType::Icon))
        //("RustLogoMemory", nwg::MemoryImageT{source: Vec::from( &EMBED_BMP01[..] )} )
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