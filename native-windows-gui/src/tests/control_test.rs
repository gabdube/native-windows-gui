use crate::*;
use std::cell::RefCell;

static BALL_DATA: &'static [u8] = include_bytes!("../../test_rc/ball.bmp");


#[derive(Default)]
#[allow(dead_code)]
pub struct TestRun {
    window: bool,
    button: bool,
    check: bool,
    combo: bool,
    date: bool,
    font: bool,
    list: bool,
    menu: bool,
    radio: bool,
    text: bool,
    progress: bool,
    track: bool,
    tooltip: bool,
    status: bool,
}


#[derive(Default)]
pub struct ControlsTest {
    // data
    runs: RefCell<TestRun>,

    // Resources
    window_icon: Icon,
    love_icon: Icon,
    love_small_icon: Icon,
    ferris: Bitmap,
    popcorn: Bitmap,
    popcorn_small: Bitmap,
    ball: Bitmap,
    arial_font: Font,
    segoe_font: Font,
    wait_cursor: Cursor,
    tabs_image_list: ImageList,
    
    // Dialogs
    open_file_dialog: FileDialog,
    open_directory_dialog: FileDialog,
    save_file_dialog: FileDialog,
    color_dialog: ColorDialog,
    font_dialog: FontDialog,

    // Layouts
    dialog_tab_layout: GridLayout,
    tree_tab_layout: GridLayout,
    list_view_tab_layout: GridLayout,
    panel_layout: GridLayout,
    tab_container_layout: FlexboxLayout,

    // Control window
    pub window: Window,

    tray_icon: TrayNotification,
    tray_icon_2: TrayNotification,
    status: StatusBar,

    // First Tab
    controls_holder: TabsContainer,
    basics_control_tab: Tab,
    basics_control_tab2: Tab,
    dialog_tab: Tab,
    tree_tab: Tab,
    list_view_tab: Tab,
    test_button: Button,
    test_checkbox1: CheckBox,
    test_checkbox2: CheckBox,
    test_combo: ComboBox<&'static str>,
    test_date: DatePicker,
    test_img_frame: ImageFrame,
    test_label: Label,
    test_list_box1: ListBox<&'static str>,
    test_list_box2: ListBox<&'static str>,
    test_radio1: RadioButton,
    test_radio2: RadioButton,
    test_radio3: RadioButton,
    test_radio4: RadioButton,
    test_text_input: TextInput,
    test_text_box: TextBox,
    test_progress1: ProgressBar,
    test_progress2: ProgressBar,
    test_track1: TrackBar,
    test_track2: TrackBar,

    // Second Tab
    test_image_button: Button,
    test_image_button2: Button,
    test_image_button3: Button,
    test_button_checkbox: CheckBox,
    test_number_select: NumberSelect,
    test_rich: RichTextBox,
    test_scroll_h: ScrollBar,
    test_scroll_v: ScrollBar,
    test_maximize: Button,
    test_minimize: Button,
    test_restore: Button,

    // Third Tab
    test_open_file_button: Button,
    test_open_directory_button: Button,
    test_save_file_button: Button,
    file_dialog_result: TextBox,
    test_select_color_button: Button,
    test_color_output: TextInput,
    test_select_font_button: Button,
    test_font_output: TextInput,

    // Fourth Tab
    test_tree: TreeView,
    test_tree_input: TextInput,
    test_tree_add: Button,
    test_tree_remove: Button,

    // Fifth Tab
    test_list_view: ListView,

    // Tooltip
    test_ttp1: Tooltip,
    test_ttp2: Tooltip,
    test_ttp3: Tooltip,

    // Menu
    window_menu: Menu,
    window_submenu1: Menu,
    window_menu_sep: MenuSeparator,
    window_menu_item1: MenuItem,
    window_menu_item2: MenuItem,
    window_menu_item3: MenuItem,

    pop_menu: Menu,
    pop_menu_item1: MenuItem,
    pop_menu_item2: MenuItem,

    // Control panel
    pub panel: Window,
    run_window_test: Button,
    run_button_test: Button,
    run_check_box_test: Button,
    run_combo_test: Button,
    run_date_test: Button,
    run_font_test: Button,
    run_list_test: Button,
    run_menu_test: Button,
    run_radio_test: Button,
    run_text_test: Button,
    run_progress_test: Button,
    run_track_test: Button,
    run_tooltip_test: Button,
    run_status_test: Button,
    run_tray_test: Button,
}

mod partial_controls_test_ui {
    use super::*;
    use crate::{PartialUi, NwgError, ControlHandle};

    impl PartialUi for ControlsTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut ControlsTest, _parent: Option<W>) -> Result<(), NwgError> {
            
            //
            // Resources
            //
            Icon::builder()
                .source_file(Some("./test_rc/cog.ico"))
                .build(&mut data.window_icon)?;

            Icon::builder()
                .source_file(Some("./test_rc/love.ico"))
                .build(&mut data.love_icon)?;

            Icon::builder()
                .source_file(Some("./test_rc/love.ico"))
                .size(Some((25, 25)))
                .build(&mut data.love_small_icon)?;

            Bitmap::builder()
                .source_file(Some("./test_rc/ferris.bmp"))
                .build(&mut data.ferris)?;

            Bitmap::builder()
                .source_bin(Some(BALL_DATA))
                .build(&mut data.ball)?;

            Bitmap::builder()
                .source_file(Some("./test_rc/popcorn.bmp"))
                .size(Some((80, 80)))
                .build(&mut data.popcorn)?;

            Bitmap::builder()
                .source_file(Some("./test_rc/popcorn.bmp"))
                .size(Some((25, 25)))
                .build(&mut data.popcorn_small)?;

            Cursor::builder()
                .source_system(Some(OemCursor::Wait))
                .build(&mut data.wait_cursor)?;

            ImageList::builder()
                .size((16, 16))
                .build(&mut data.tabs_image_list)?;

            data.tabs_image_list.add_bitmap_from_filename("./test_rc/list_0.png")?;
            data.tabs_image_list.add_bitmap_from_filename("./test_rc/list_1.png")?;
            data.tabs_image_list.add_bitmap_from_filename("./test_rc/list_2.png")?;
            data.tabs_image_list.add_bitmap_from_filename("./test_rc/list_3.png")?;
            
            let dir = ::std::env::current_dir().unwrap();
            FileDialog::builder()
                .action(FileDialogAction::Open)
                .multiselect(true)
                .title("Open a file")
                .default_folder(dir.to_str().unwrap())
                .build(&mut data.open_file_dialog)?;

            FileDialog::builder()
                .action(FileDialogAction::OpenDirectory)
                .title("Open a directory")
                .build(&mut data.open_directory_dialog)?;

            FileDialog::builder()
                .action(FileDialogAction::Save)
                .title("Save a file")
                .filters("Text(*.txt)|Any(*.*)")
                .build(&mut data.save_file_dialog)?;

            ColorDialog::builder()
                .build(&mut data.color_dialog)?;

            FontDialog::builder()
                .build(&mut data.font_dialog)?;

            Font::builder()
                .size(20)
                .family("Arial")
                .build(&mut data.arial_font)?;

            Font::builder()
                .size(23)
                .family("Segoe UI")
                .build(&mut data.segoe_font)?;

            //
            //  Controls holder
            //

            Window::builder()
                .flags(WindowFlags::MAIN_WINDOW)
                .size((480, 450))
                .position((100, 100))
                .title("Controls")
                .icon(Some(&data.window_icon))
                .build(&mut data.window)?;

            
            TrayNotification::builder()
                .parent(&data.window)
                .icon(Some(&data.window_icon))
                .tip(Some("Native Windows GUI tests"))
                .build(&mut data.tray_icon)?;

            StatusBar::builder()
                .text("Ready for tests ;)")
                .parent(&data.window)
                .build(&mut data.status)?;

            TabsContainer::builder()
                .parent(&data.window)
                .image_list(Some(&data.tabs_image_list))
                .build(&mut data.controls_holder)?;

            Tab::builder()
                .text("Basic")
                .parent(&data.controls_holder)
                .image_index(Some(0))
                .build(&mut data.basics_control_tab)?;

            Tab::builder()
                .text("Basic 2")
                .parent(&data.controls_holder)
                .image_index(Some(1))
                .build(&mut data.basics_control_tab2)?;

            Tab::builder()
                .text("Dialog")
                .parent(&data.controls_holder)
                .image_index(Some(2))
                .build(&mut data.dialog_tab)?;

            Tab::builder()
                .text("Tree view")
                .parent(&data.controls_holder)
                .image_index(Some(3))
                .build(&mut data.tree_tab)?;

            Tab::builder()
                .text("List view")
                .parent(&data.controls_holder)
                .image_index(Some(3))
                .build(&mut data.list_view_tab)?;

            Button::builder()
                .text("A simple button")
                .position((10, 10))
                .size((130, 30))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_button)?;

            CheckBox::builder()
                .text("I like bacon")
                .position((10, 50))
                .size((130, 30))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_checkbox1)?;

            CheckBox::builder()
                .flags(CheckBoxFlags::VISIBLE | CheckBoxFlags::TRISTATE)
                .text("Three state")
                .position((10, 80))
                .size((130, 30))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_checkbox2)?;

            ComboBox::builder()
                .position((10, 120))
                .size((130, 30))
                .collection(vec!["Chocolate", "Strawberry", "Blueberry"])
                .selected_index(Some(0))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_combo)?;

            DatePicker::builder()
                .position((10, 160))
                .size((130, 30))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_date)?;

            Label::builder()
                .text("A label\r\nSecond line")
                .position((10, 200))
                .size((130, 50))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_label)?;

            ListBox::builder()
                .position((10, 260))
                .size((130, 100))
                .parent(&data.basics_control_tab)
                .collection(vec!["Red", "White", "Green", "Yellow"])
                .selected_index(Some(1))
                .build(&mut data.test_list_box1)?;

            ListBox::builder()
                .flags(ListBoxFlags::VISIBLE | ListBoxFlags::MULTI_SELECT)
                .position((150, 10))
                .size((130, 100))
                .parent(&data.basics_control_tab)
                .collection(vec!["Cat", "Dog", "Parrot", "Horse", "Ogre"])
                .multi_selection(vec![0, 2, 3])
                .build(&mut data.test_list_box2)?;

            ImageFrame::builder()
                .position((150, 110))
                .size((130, 99))
                .parent(&data.basics_control_tab)
                .bitmap(Some(&data.ferris))
                .background_color(Some([255,255,255]))
                .build(&mut data.test_img_frame)?;

            RadioButton::builder()
                .flags(RadioButtonFlags::GROUP | RadioButtonFlags::VISIBLE)
                .text("Cats")
                .position((150, 220))
                .size((130, 25))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_radio1)?;

            RadioButton::builder()
                .text("Dogs")
                .position((150, 245))
                .size((130, 25))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_radio2)?;

            RadioButton::builder()
                .flags(RadioButtonFlags::GROUP | RadioButtonFlags::VISIBLE)
                .text("Energy drink")
                .position((150, 280))
                .size((130, 25))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_radio3)?;

            RadioButton::builder()
                .text("Chocolate")
                .position((150, 305))
                .size((130, 25))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_radio4)?;

            TextInput::builder()
                .text("Hello World!")
                .position((290, 10))
                .size((150, 25))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_text_input)?;

            TextBox::builder()
                .text("Multi\r\nLine\r\nText")
                .flags(TextBoxFlags::VISIBLE | TextBoxFlags::AUTOVSCROLL | TextBoxFlags::AUTOHSCROLL | TextBoxFlags::TAB_STOP)
                .position((290, 40))
                .size((150, 100))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_text_box)?;

            ProgressBar::builder()
                .position((290, 150))
                .size((150, 30))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_progress1)?;

            ProgressBar::builder()
                .flags(ProgressBarFlags::VISIBLE | ProgressBarFlags::VERTICAL | ProgressBarFlags::MARQUEE)
                .position((340, 220))
                .size((30, 110))
                .range(0..100)
                .pos(50)
                .marquee(true)
                .marquee_update(0)
                .parent(&data.basics_control_tab)
                .build(&mut data.test_progress2)?;

            TrackBar::builder()
                .position((290, 190))
                .size((150, 20))
                .parent(&data.basics_control_tab)
                .background_color(Some([255, 255, 255]))
                .build(&mut data.test_track1)?;

            TrackBar::builder()
                .flags(TrackBarFlags::VISIBLE | TrackBarFlags::RANGE | TrackBarFlags::VERTICAL | TrackBarFlags::AUTO_TICK)
                .position((290, 220))
                .size((40, 110))
                .background_color(Some([255, 255, 255]))
                .parent(&data.basics_control_tab)
                .build(&mut data.test_track2)?;
            

            //
            // Control tab 2
            //
            Button::builder()
                .flags(ButtonFlags::VISIBLE | ButtonFlags::BITMAP)
                .position((10, 10))
                .size((90, 90))
                .bitmap(Some(&data.popcorn))
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_image_button)?;

            Button::builder()
                .position((110, 10))
                .size((140, 40))
                .icon(Some(&data.love_small_icon))
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_image_button2)?;

            Button::builder()
                .position((110, 55))
                .size((140, 40))
                .bitmap(Some(&data.ball))
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_image_button3)?;

            CheckBox::builder()
                .flags(CheckBoxFlags::VISIBLE | CheckBoxFlags::PUSHLIKE)
                .text("A check box button")
                .position((260, 10))
                .size((140, 40))
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_button_checkbox)?;

            NumberSelect::builder()
                .value_float(10.50)
                .step_float(0.5)
                .decimals(2)
                .position((10, 110))
                .size((140, 25))
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_number_select)?;

            RichTextBox::builder()
                .text("That's a rich text box!")
                .position((10, 140))
                .size((200, 200))
                .parent(&data.basics_control_tab2)
                .font(Some(&data.segoe_font))
                .build(&mut data.test_rich)?;

            ScrollBar::builder()
                .position((220,140))
                .size((20, 200))
                .parent(&data.basics_control_tab2)
                .range(Some(0..100))
                .pos(Some(30))
                .build(&mut data.test_scroll_v)?;

            ScrollBar::builder()
                .position((160, 110))
                .size((90, 20))
                .range(Some(0..10))
                .flags(ScrollBarFlags::VISIBLE | ScrollBarFlags::HORIZONTAL)
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_scroll_h)?;

            Button::builder()
                .text("Maximize")
                .position((260, 55))
                .size((140, 40))
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_maximize)?;

            Button::builder()
                .text("Minimize")
                .position((260, 100))
                .size((140, 40))
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_minimize)?;

            Button::builder()
                .text("Restore")
                .position((260, 145))
                .size((140, 40))
                .parent(&data.basics_control_tab2)
                .build(&mut data.test_restore)?;

            //
            // Dialogs
            //
            Button::builder()
                .text("Open file")
                .parent(&data.dialog_tab)
                .enabled(cfg!(feature="file-dialog"))
                .build(&mut data.test_open_file_button)?;

            Button::builder()
                .text("Open directory")
                .parent(&data.dialog_tab)
                .enabled(cfg!(feature="file-dialog"))
                .build(&mut data.test_open_directory_button)?;

            Button::builder()
                .text("Save file")
                .parent(&data.dialog_tab)
                .enabled(cfg!(feature="file-dialog"))
                .build(&mut data.test_save_file_button)?;

            TextBox::builder()
                .parent(&data.dialog_tab)
                .flags(TextBoxFlags::VISIBLE | TextBoxFlags::AUTOVSCROLL | TextBoxFlags::AUTOHSCROLL | TextBoxFlags::TAB_STOP)
                .build(&mut data.file_dialog_result)?;

            Button::builder()
                .text("Select a color")
                .parent(&data.dialog_tab)
                .enabled(cfg!(feature="color-dialog"))
                .build(&mut data.test_select_color_button)?;

            TextInput::builder()
                .parent(&data.dialog_tab)
                .placeholder_text(Some("The color will go here"))
                .background_color(Some([255, 255, 255]))
                .build(&mut data.test_color_output)?;

            Button::builder()
                .text("Select a font")
                .parent(&data.dialog_tab)
                .enabled(cfg!(feature="font-dialog"))
                .build(&mut data.test_select_font_button)?;

            TextInput::builder()
                .parent(&data.dialog_tab)
                .placeholder_text(Some("The font will go here"))
                .background_color(Some([255, 255, 255]))
                .build(&mut data.test_font_output)?;

            //
            // Treeview
            //
            TreeView::builder()
                .parent(&data.tree_tab)
                .build(&mut data.test_tree)?;

            TextInput::builder()
                .text("New Item")
                .background_color(Some([255, 255, 255]))
                .parent(&data.tree_tab)
                .build(&mut data.test_tree_input)?;

            Button::builder()
                .text("Add item")
                .parent(&data.tree_tab)
                .build(&mut data.test_tree_add)?;

            Button::builder()
                .text("Remove item")
                .parent(&data.tree_tab)
                .build(&mut data.test_tree_remove)?;

            //
            // Listview
            //
            ListView::builder()
                .parent(&data.list_view_tab)
                .ex_flags(ListViewExFlags::GRID | ListViewExFlags::FULL_ROW_SELECT)
                .list_style(ListViewStyle::Detailed)
                .build(&mut data.test_list_view)?;


            //
            // Tooltip
            //
            Tooltip::builder()
                .register(&data.test_button, "A test button")
                .register(&data.test_date, "A test date picker")
                .register(&data.test_combo, "A test combobox")
                .register_callback(&data.window)
                .register_callback(&data.test_text_input)
                .build(&mut data.test_ttp1)?;

            Tooltip::builder()
                .decoration(Some("Tooltip title (fancy)"), Some(&data.window_icon))
                .register(&data.test_img_frame, "Hello rust!")
                .build(&mut data.test_ttp2)?;

            Tooltip::builder()
                .default_decoration(Some("More info"), Some(TooltipIcon::InfoLarge))
                .register(&data.test_list_box1, "Simple list")
                .register(&data.test_list_box2, "Multi select list")
                .build(&mut data.test_ttp3)?;

            //
            // Menu
            //
            Menu::builder()
                .text("&Test menu")
                .parent(&data.window)
                .build(&mut data.window_menu)?;

            Menu::builder()
                .text("Test &Submenu")
                .parent(&data.window_menu)
                .build(&mut data.window_submenu1)?;
            
            MenuSeparator::builder()
                .parent(&data.window_menu)
                .build(&mut data.window_menu_sep)?;

            MenuItem::builder()
                .text("Test item 1")
                .parent(&data.window_menu)
                .build(&mut data.window_menu_item1)?;

            MenuItem::builder()
                .text("Test item 2")
                .check(true)
                .parent(&data.window_submenu1)
                .build(&mut data.window_menu_item2)?;

            MenuItem::builder()
                .text("Test item 3")
                .parent(&data.window)
                .build(&mut data.window_menu_item3)?;

            Menu::builder()
                .popup(true)
                .parent(&data.window)
                .build(&mut data.pop_menu)?;

            MenuItem::builder()
                .text("Popup item 1\tCTRL+P")
                .parent(&data.pop_menu)
                .build(&mut data.pop_menu_item1)?;

            MenuItem::builder()
                .text("Popup item 2")
                .parent(&data.pop_menu)
                .build(&mut data.pop_menu_item2)?;


            //
            // Run tests
            //

            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((300, 360))
                .position((650, 100))
                .title("Action panel")
                .icon(Some(&data.window_icon))
                .parent(Some(&data.window))
                .build(&mut data.panel)?;

            TrayNotification::builder()
                .parent(&data.panel)
                .flags(TrayNotificationFlags::SILENT | TrayNotificationFlags::USER_ICON | TrayNotificationFlags::LARGE_ICON)
                .icon(Some(&data.love_icon))
                .balloon_icon(Some(&data.love_icon))
                .info(Some("Tray notification by NWG"))
                .info_title(Some("Native Windows GUI tests"))
                .tip(Some("Hello!"))
                .build(&mut data.tray_icon_2)?;
            
            Button::builder()
                .text("Run window test")
                .parent(&data.panel)
                .build(&mut data.run_window_test)?;

            Button::builder()
                .text("Run button test")
                .parent(&data.panel)
                .build(&mut data.run_button_test)?;

            Button::builder()
                .text("Run checkbox test")
                .parent(&data.panel)
                .build(&mut data.run_check_box_test)?;

            Button::builder()
                .text("Run combo test")
                .parent(&data.panel)
                .build(&mut data.run_combo_test)?;

            Button::builder()
                .text("Run date test")
                .parent(&data.panel)
                .build(&mut data.run_date_test)?;

            Button::builder()
                .text("Run font test")
                .parent(&data.panel)
                .build(&mut data.run_font_test)?;

            Button::builder()
                .text("Run list test")
                .parent(&data.panel)
                .build(&mut data.run_list_test)?;

            Button::builder()
                .text("Run menu test")
                .parent(&data.panel)
                .build(&mut data.run_menu_test)?;

            Button::builder()
                .text("Run radio test")
                .parent(&data.panel)
                .build(&mut data.run_radio_test)?;

            Button::builder()
                .text("Run text test")
                .parent(&data.panel)
                .build(&mut data.run_text_test)?;

            Button::builder()
                .text("Run progress test")
                .parent(&data.panel)
                .build(&mut data.run_progress_test)?;

            Button::builder()
                .text("Run track test")
                .parent(&data.panel)
                .build(&mut data.run_track_test)?;
            
            Button::builder()
                .text("Run tooltip test")
                .parent(&data.panel)
                .build(&mut data.run_tooltip_test)?;

            Button::builder()
                .text("Run status test")
                .parent(&data.panel)
                .build(&mut data.run_status_test)?;
            Button::builder()
                .text("Run tray test")
                .parent(&data.panel)
                .build(&mut data.run_tray_test)?;

            //
            // Layout
            //

            use stretch::style::Dimension as D;
            use stretch::geometry::Rect;
            FlexboxLayout::builder()
                .parent(&data.window)
                .border(Rect { start: D::Points(2.0), end: D::Points(2.0), top: D::Points(2.0), bottom: D::Points(20.0) } )
                .child(&data.controls_holder)
                .build(&data.tab_container_layout)?;

            GridLayout::builder()
                .parent(&data.panel)
                .spacing(1)
                .max_row(Some(8))
                .child(0, 0, &data.run_window_test)
                .child(1, 0, &data.run_button_test)
                .child(0, 1, &data.run_check_box_test)
                .child(1, 1, &data.run_combo_test)
                .child(0, 2, &data.run_date_test)
                .child(1, 2, &data.run_font_test)
                .child(0, 3, &data.run_list_test)
                .child(1, 3, &data.run_menu_test)
                .child(0, 4, &data.run_radio_test)
                .child(1, 4, &data.run_text_test)
                .child(0, 5, &data.run_progress_test)
                .child(1, 5, &data.run_track_test)
                .child(0, 6, &data.run_tooltip_test)
                .child(1, 6, &data.run_status_test)
                .child(0, 7, &data.run_tray_test)
                .build(&data.panel_layout)?;
            
            GridLayout::builder()
                .parent(&data.dialog_tab)
                .min_size([400, 150])
                .max_size([u32::max_value(), 200])
                .child(0, 0, &data.test_open_file_button)
                .child(1, 0, &data.test_open_directory_button)
                .child(2, 0, &data.test_save_file_button)
                .child_item(GridLayoutItem::new(&data.file_dialog_result, 0, 1, 3, 1))
                .child(0, 2, &data.test_select_color_button)
                .child_item(GridLayoutItem::new(&data.test_color_output, 1, 2, 2, 1))
                .child(0, 3, &data.test_select_font_button)
                .child_item(GridLayoutItem::new(&data.test_font_output, 1, 3, 2, 1))
                .build(&data.dialog_tab_layout)?;
            
            GridLayout::builder()
                .parent(&data.tree_tab)
                .min_size([400, 220])
                .child_item(GridLayoutItem::new(&data.test_tree, 0, 0, 1, 7))
                .child(1, 0, &data.test_tree_input)
                .child(1, 1, &data.test_tree_add)
                .child(1, 2, &data.test_tree_remove)
                .build(&data.tree_tab_layout)?;

            GridLayout::builder()
                .parent(&data.list_view_tab)
                .min_size([400, 220])
                .child_item(GridLayoutItem::new(&data.test_list_view, 0, 0, 1, 7))
                .build(&data.list_view_tab_layout)?;

            Ok(())
        }

        fn process_event<'a>(&self, evt: Event, _evt_data: &EventData, handle: ControlHandle) {
            use crate::Event as E;

            match evt {
                E::OnInit => 
                    if &handle == &self.window {
                        init_tree(self);
                        init_list_view(self);
                        init_rich_text_box(self);
                    },
                E::OnWindowClose => 
                    if &handle == &self.window {
                        self.panel.set_visible(false);
                    },
                E::OnButtonClick =>
                    if &handle == &self.run_window_test {
                        run_window_tests(self, evt);
                    } else if &handle == &self.run_button_test {
                        run_button_tests(self, evt);
                    } else if &handle == &self.run_check_box_test {
                        run_check_box_tests(self, evt);
                    } else if &handle == &self.run_combo_test {
                        run_combo_tests(self, evt);
                    } else if &handle == &self.run_date_test {
                        run_date_tests(self, evt);
                    } else if &handle == &self.run_font_test {
                        run_font_tests(self, evt);
                    } else if &handle == &self.run_list_test {
                        run_list_tests(self, evt);
                    } else if &handle == &self.run_menu_test {
                        run_menu_tests(self, evt);
                    } else if &handle == &self.run_radio_test {
                        run_radio_tests(self, evt);
                    } else if &handle == &self.run_text_test {
                        run_text_tests(self, evt);
                    } else if &handle == &self.run_progress_test {
                        run_progress_tests(self, evt);
                    } else if &handle == &self.run_track_test {
                        run_track_tests(self, evt);
                    } else if &handle == &self.run_tooltip_test {
                        run_tooltip_tests(self, evt);
                    } else if &handle == &self.run_status_test {
                        run_status_tests(self, evt);
                    } else if &handle == &self.test_open_file_button {
                        open_file(self, evt);
                    } else if &handle == &self.test_open_directory_button {
                        open_directory(self, evt);
                    } else if &handle == &self.test_save_file_button {
                        save_file(self, evt);
                    } else if &handle == &self.test_tree_add {
                        tree_tests(self, &self.test_tree_add.handle);
                    } else if &handle == &self.test_tree_remove {
                        tree_tests(self, &self.test_tree_remove.handle);
                    } else if &handle == &self.test_select_color_button {
                        color_select(self);
                    } else if &handle == &self.test_select_font_button {
                        font_select(self);
                    } else if &handle == &self.run_tray_test {
                        run_tray_tests(self);
                    } else if &handle == &self.test_maximize {
                        self.window.maximize();
                    } else if &handle == &self.test_minimize {
                        self.window.minimize();
                    } else if &handle == &self.test_restore {
                        self.window.restore();
                    },
                E::OnContextMenu => 
                    if &handle == &self.window {
                        show_pop_menu(self, evt);
                    } else if &handle == &self.basics_control_tab {
                        show_pop_menu(self, evt);
                    } else if &handle == &self.tray_icon_2 {
                        show_pop_menu(self, evt);
                    },
                E::OnTooltipText => 
                    if &handle == &self.window {
                        set_tooltip_dynamic(self, &self.window.handle, _evt_data.on_tooltip_text());
                    } else if &handle == &self.test_text_input {
                        set_tooltip_dynamic(self, &self.test_text_input.handle, _evt_data.on_tooltip_text());
                    },
                E::OnMenuItemSelected => 
                    if &handle == &self.window_menu_item1 {
                        item_hello("menu item");
                    } else if &handle == &self.pop_menu_item1 {
                        item_hello("popup menu item");
                    },
                E::OnChar => {
                    if &handle == &self.test_rich {
                        print_char(_evt_data);
                    }
                },
                E::OnListViewColumnClick => {
                    if &handle == &self.test_list_view {
                        set_lv_sort(&self.test_list_view, _evt_data);
                    }
                },
                _ => {}
            }
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle, &self.panel.handle]
        }

    }
}

fn item_hello(m: &'static str) {
    simple_message("Hello", &format!("Hello from {}!", m));
}

fn init_rich_text_box(app: &ControlsTest) {
    let rich = &app.test_rich;

    rich.set_selection(0..1000);
    rich.set_char_format(&CharFormat {
        effects: Some(CharEffects::BOLD),
        height: Some(250),
        text_color: Some([200, 0, 0]),
        ..Default::default()
    })
}

fn init_tree(app: &ControlsTest) {
    let tree = &app.test_tree;
    let item = tree.insert_item("Hello", None, TreeInsert::Root);
    let view = tree.insert_item("A tree View", Some(&item), TreeInsert::First);
    tree.insert_item("AHHHHHHH", Some(&view), TreeInsert::First);
    tree.insert_item("Items", Some(&item), TreeInsert::First);

    let other = tree.insert_item("Another root children", Some(&item), TreeInsert::Last);
    tree.insert_item("Banana", Some(&other), TreeInsert::First);
    tree.insert_item("Pinapple", Some(&other), TreeInsert::First);
}

fn init_list_view(app: &ControlsTest) {
    let list = &app.test_list_view;

    for &column in &["Name", "Price", "Quantity"] {
        list.insert_column(column);
    }
    list.set_headers_enabled(true);
    list.set_column_sort_arrow(1, Some(ListViewColumnSortArrow::Down));

    let data: &[&[&str]] = &[
        // &["Name", "Price (USD $)", "Quantity"],
        &["Banana", "10.0", "1000"],
        &["Apple", "2.0", "345"],
        &["Kiwi", "5.0", "194"],
        &["Oranges", "5.0", "15"],
        &["Lettuce", "1.0", "257"],
    ];

    for d in data {
        list.insert_items_row(None, d);
    }
}

fn show_pop_menu(app: &ControlsTest, _evt: Event) {
    let (x, y) = GlobalCursor::position();
    app.pop_menu.popup(x, y);
}

fn run_window_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().window {

        assert_eq!(&app.window.text(), "Controls");
        app.window.set_text("Controls New title");
        assert_eq!(&app.window.text(), "Controls New title");

        assert_eq!(app.window.visible(), true);
        app.window.set_visible(false);
        assert_eq!(app.window.visible(), false);
        app.window.set_visible(true);

        assert_eq!(app.window.enabled(), true);
        app.window.set_enabled(false);
        assert_eq!(app.window.enabled(), false);
        app.window.set_enabled(true);

        app.window.set_position(100, 100);
        assert_eq!(app.window.position(), (100, 100));

        app.window.set_size(500, 420);
        // The actual size return here might be less because it does not take account of the menubar
        // assert_eq!(app.window.size(), (500, 400));

        app.runs.borrow_mut().window = true;
    } else {
        app.window.set_text("Controls");
        app.runs.borrow_mut().window = false;
    }
}

fn run_button_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().button {

        assert_eq!(&app.test_button.text(), "A simple button");
        app.test_button.set_text("New Text");
        assert_eq!(&app.test_button.text(), "New Text");

        assert_eq!(app.test_button.position(), (10, 10));
        app.test_button.set_position(5, 5);
        assert_eq!(app.test_button.position(), (5, 5));

        assert_eq!(app.test_button.size(), (130, 30));
        app.test_button.set_size(120, 35);
        assert_eq!(app.test_button.size(), (120, 35));

        if app.basics_control_tab.visible() {
            assert_eq!(app.test_button.visible(), true);
            app.test_button.set_visible(false);
            assert_eq!(app.test_button.visible(), false);
            app.test_button.set_visible(true);
        }

        app.test_button.set_focus();
        assert_eq!(app.test_button.focus(), true);
        app.window.set_focus();
        assert_eq!(app.test_button.focus(), false);

        assert_eq!(app.test_button.enabled(), true);
        app.test_button.set_enabled(false);
        assert_eq!(app.test_button.enabled(), false);


        let mut icon = None;
        let mut bitmap = None;
        app.test_image_button2.image(&mut bitmap, &mut icon);
        assert!(icon.is_some() && bitmap.is_none());

        app.test_image_button2.set_bitmap(Some(&app.popcorn_small));
        app.test_image_button2.image(&mut bitmap, &mut icon);
        assert!(icon.is_none() && bitmap.is_some());

        app.test_image_button2.set_icon(None);
        app.test_image_button2.image(&mut bitmap, &mut icon);
        assert!(icon.is_none() && bitmap.is_none());

        app.runs.borrow_mut().button = true;
    } else {
        app.test_button.set_text("A simple button");
        app.test_button.set_position(10, 10);
        app.test_button.set_size(130, 30);
        app.test_button.set_enabled(true);
        app.test_image_button2.set_icon(Some(&app.love_small_icon));
        app.runs.borrow_mut().button = false;
    }
}

fn run_check_box_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().check {

        assert_eq!(app.test_checkbox2.tristate(), true);
        assert_eq!(app.test_checkbox1.tristate(), false);

        app.test_checkbox1.set_tristate(true);
        assert_eq!(app.test_checkbox1.tristate(), true);

        app.test_checkbox1.set_check_state(CheckBoxState::Checked);
        assert_eq!(app.test_checkbox1.check_state(), CheckBoxState::Checked);

        app.test_checkbox1.set_check_state(CheckBoxState::Unchecked);
        assert_eq!(app.test_checkbox1.check_state(), CheckBoxState::Unchecked);

        app.test_checkbox1.set_check_state(CheckBoxState::Indeterminate);
        assert_eq!(app.test_checkbox1.check_state(), CheckBoxState::Indeterminate);

        app.runs.borrow_mut().check = true;
    } else {
        app.test_checkbox1.set_tristate(false);
        app.runs.borrow_mut().check = false;
    }
}

fn run_combo_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().combo {
        {
            let col = app.test_combo.collection();
            assert_eq!(&col as &[&'static str], &["Chocolate", "Strawberry", "Blueberry"]);
        }

        {
            let mut col = app.test_combo.collection_mut();
            col.push("Hello");
        }

        app.test_combo.sync();
        app.test_combo.push("World!");
        assert_eq!(app.test_combo.len(), 5);

        app.test_combo.set_selection(None);
        assert_eq!(app.test_combo.selection(), None);
        assert_eq!(app.test_combo.selection_string(), None);

        app.test_combo.set_selection(Some(2));
        assert_eq!(app.test_combo.selection(), Some(2));
        assert_eq!(app.test_combo.selection_string(), Some("Blueberry".to_string()));

        assert_eq!(app.test_combo.set_selection_string("hel"), Some(3));
        assert_eq!(app.test_combo.selection(), Some(3));
        assert_eq!(app.test_combo.selection_string(), Some("Hello".to_string()));

        app.test_combo.sort();
        assert_eq!(app.test_combo.set_selection_string("Blue"), Some(0));

        app.test_combo.insert(1, "BOO!");
        app.test_combo.insert(std::usize::MAX, "Ahoy!!");
        assert_eq!(app.test_combo.set_selection_string("BOO!"), Some(1));
        assert_eq!(app.test_combo.set_selection_string("Ahoy!!"), Some(6));

        app.test_combo.remove(0);

        app.test_combo.dropdown(true);

        app.runs.borrow_mut().combo = true;
    } else {
        app.test_combo.set_collection(vec!["Chocolate", "Strawberry", "Blueberry"]);
        app.runs.borrow_mut().combo = false;
    }
}

fn run_date_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().date {
        
        let v = DatePickerValue { year: 2000, month: 10, day: 5 };
        app.test_date.set_value(Some(v));
        assert_eq!(app.test_date.value(), Some(v));
        assert_eq!(app.test_date.checked(), true);

        app.test_date.set_value(None);
        assert_eq!(app.test_date.value(), None);
        assert_eq!(app.test_date.checked(), false);

        app.test_date.set_format(Some("'YEAR: 'yyyy"));

        let up = DatePickerValue { year: 2000, month: 1, day: 1 };
        let down = DatePickerValue { year: 2001, month: 1, day: 1 };
        app.test_date.set_range(&[up, down]);
        assert_eq!(app.test_date.range(), [up, down]);

        app.runs.borrow_mut().date = true;
    } else {
        app.test_date.set_format(None);

        let up = DatePickerValue { year: 1950, month: 1, day: 1 };
        let down = DatePickerValue { year: 2020, month: 12, day: 30 };
        app.test_date.set_range(&[up, down]);
        app.runs.borrow_mut().date = false;
    }
}

fn run_font_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().font {
        app.test_label.set_font(Some(&app.arial_font));
        app.test_button.set_font(Some(&app.arial_font));
        app.test_checkbox1.set_font(Some(&app.arial_font));
        app.test_checkbox2.set_font(Some(&app.arial_font));
        app.test_combo.set_font(Some(&app.arial_font));
        app.test_date.set_font(Some(&app.arial_font));
        app.test_date.set_font(Some(&app.arial_font));
        app.test_list_box1.set_font(Some(&app.arial_font));
        app.test_list_box2.set_font(Some(&app.arial_font));
        app.controls_holder.set_font(Some(&app.arial_font));
        app.test_text_input.set_font(Some(&app.arial_font));
        app.test_text_box.set_font(Some(&app.arial_font));
        app.test_tree.set_font(Some(&app.arial_font));

        assert_eq!(app.test_label.font().as_ref(), Some(&app.arial_font));

        app.runs.borrow_mut().font = true;
    } else {
        app.test_label.set_font(None);
        app.test_button.set_font(None);
        app.test_checkbox1.set_font(None);
        app.test_checkbox2.set_font(None);
        app.test_combo.set_font(None);
        app.test_date.set_font(None);
        app.test_list_box1.set_font(None);
        app.test_list_box2.set_font(None);
        app.controls_holder.set_font(None);
        app.test_tree.set_font(None);

        app.test_list_box1.set_size(130, 100);
        app.test_list_box2.set_size(130, 100);

        app.runs.borrow_mut().font = false;
    }
}

fn run_list_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().list {
        app.test_list_box2.unselect_all();

        {
            let col = app.test_list_box1.collection();
            assert_eq!(&col as &[&'static str], &["Red", "White", "Green", "Yellow"]);
        }

        {
            let mut col = app.test_list_box1.collection_mut();
            col.push("Blue");
        }

        app.test_list_box1.sync();
        app.test_list_box1.push("Hello!");
        assert_eq!(app.test_list_box1.len(), 6);

        app.test_list_box1.set_selection(Some(0));
        assert_eq!(app.test_list_box1.selected(0), true);
        
        
        app.test_list_box1.set_selection(None);
        assert_eq!(app.test_list_box1.selected(0), false);
        assert_eq!(app.test_list_box1.selection(), None);
        assert_eq!(app.test_list_box1.selection_string(), None);

        app.test_list_box1.set_selection(Some(2));
        assert_eq!(app.test_list_box1.selection(), Some(2));
        assert_eq!(app.test_list_box1.selection_string(), Some("Green".to_string()));

        app.test_list_box1.insert(1, "BOO!");
        app.test_list_box1.insert(std::usize::MAX, "Ahoy!!");
        assert_eq!(app.test_list_box1.set_selection_string("BOO!"), Some(1));
        assert_eq!(app.test_list_box1.set_selection_string("Ahoy!!"), Some(7));

        app.test_list_box1.remove(0);

        
        app.test_list_box2.multi_add_selection(0);
        app.test_list_box2.multi_add_selection(2);
        app.test_list_box2.multi_add_selection(3);
        assert_eq!(app.test_list_box2.multi_selection_len(), 3);
        assert_eq!(app.test_list_box2.multi_selection(), vec![0, 2, 3]);

        
        app.test_list_box2.multi_remove_selection(0);
        assert_eq!(app.test_list_box2.multi_selection_len(), 2);
        assert_eq!(app.test_list_box2.multi_selection(), vec![2, 3]);

        app.test_list_box2.select_all();
        assert_eq!(app.test_list_box2.multi_selection_len(), 5);
        assert_eq!(app.test_list_box2.multi_selection(), vec![0, 1, 2, 3, 4]);

        app.test_list_box2.unselect_all();
        assert_eq!(app.test_list_box2.multi_selection_len(), 0);
        assert_eq!(app.test_list_box2.multi_selection(), vec![]);

        app.test_list_box2.multi_select_range(0..2);
        assert_eq!(app.test_list_box2.multi_selection_len(), 3);
        assert_eq!(app.test_list_box2.multi_selection(), vec![0, 1, 2]);

        app.test_list_box2.multi_unselect_range(0..1);
        assert_eq!(app.test_list_box2.multi_selection_len(), 1);
        assert_eq!(app.test_list_box2.multi_selection(), vec![2]);

        app.runs.borrow_mut().list = true;
    } else {
        app.test_list_box2.unselect_all();
        app.test_list_box1.set_collection(vec!["Red", "White", "Green", "Yellow"]);

        app.runs.borrow_mut().list = false;
    }
}

fn run_menu_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().menu {
        app.window_menu_item1.set_enabled(false);
        assert_eq!(app.window_menu_item1.enabled(), false);

        app.window_submenu1.set_enabled(false);
        assert_eq!(app.window_submenu1.enabled(), false);

        app.pop_menu_item1.set_enabled(false);
        assert_eq!(app.pop_menu_item1.enabled(), false);

        app.pop_menu.set_enabled(false);

        app.runs.borrow_mut().menu = true;
    } else {
        app.pop_menu_item1.set_enabled(true);
        app.window_submenu1.set_enabled(true);
        app.window_menu_item1.set_enabled(true);
        app.runs.borrow_mut().menu = false;
    }
}

fn run_radio_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().radio {
        app.test_radio1.set_check_state(RadioButtonState::Checked);
        assert_eq!(app.test_radio1.check_state(), RadioButtonState::Checked);

        app.test_radio2.set_check_state(RadioButtonState::Checked);
        assert_eq!(app.test_radio2.check_state(), RadioButtonState::Checked);

        app.test_radio2.set_check_state(RadioButtonState::Unchecked);
        assert_eq!(app.test_radio2.check_state(), RadioButtonState::Unchecked);

        app.runs.borrow_mut().radio = true;
    } else {
        app.runs.borrow_mut().radio = false;
    }
}

fn run_text_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().text {
        app.test_text_input.set_text("New Text");
        assert_eq!(&app.test_text_input.text(), "New Text");

        app.test_text_input.set_limit(32);
        assert_eq!(app.test_text_input.limit(), 32);

        assert_eq!(app.test_text_input.password_char(), None);
        app.test_text_input.set_password_char(Some('X'));
        assert_eq!(app.test_text_input.password_char(), Some('X'));

        app.test_text_input.set_modified(true);
        assert_eq!(app.test_text_input.modified(), true);

        app.test_text_input.set_selection(0..4);
        assert_eq!(app.test_text_input.selection(), 0..4);

        assert_eq!(app.test_text_input.len(), 8);

        assert_eq!(app.test_text_input.visible(), true);
        app.test_text_input.set_visible(false);
        assert_eq!(app.test_text_input.visible(), false);
        app.test_text_input.set_visible(true);

        app.test_text_input.set_focus();
        assert_eq!(app.test_text_input.focus(), true);
        app.window.set_focus();
        assert_eq!(app.test_text_input.focus(), false);

        assert_eq!(app.test_text_input.readonly(), false);
        app.test_text_input.set_readonly(true);
        assert_eq!(app.test_text_input.readonly(), true);

        assert_eq!(app.test_text_input.enabled(), true);
        app.test_text_input.set_enabled(false);
        assert_eq!(app.test_text_input.enabled(), false);

        app.test_text_input.set_placeholder_text(Some("Placeholder!"));

        app.runs.borrow_mut().text = true;
    } else {
        app.test_text_input.set_text("Hello World");
        app.test_text_input.set_enabled(true);
        app.test_text_input.set_readonly(false);
        app.test_text_input.set_password_char(None);
        app.runs.borrow_mut().text = false;
    }
}

fn run_progress_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().progress {
        app.test_progress1.set_range(0..1000);

        let r = app.test_progress1.range();
        assert!(r.start == 0 && r.end == 1000);

        app.test_progress1.set_pos(500);
        assert!(app.test_progress1.pos() == 500);

        app.test_progress1.set_step(100);
        assert!(app.test_progress1.step() == 100);

        app.test_progress1.set_state(ProgressBarState::Paused);
        assert!(app.test_progress1.state() == ProgressBarState::Paused);

        app.test_progress1.advance();
        assert!(app.test_progress1.pos() == 600);

        app.test_progress1.advance_delta(50);
        assert!(app.test_progress1.pos() == 650);

        app.runs.borrow_mut().progress = true;
    } else {
        app.test_progress1.set_pos(0);
        app.test_progress1.set_state(ProgressBarState::Normal);
        app.runs.borrow_mut().progress = false;
    }
}

fn run_track_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().track {
        
        app.test_track1.set_range_min(0);
        app.test_track1.set_range_max(10);

        assert_eq!(app.test_track1.range_min(), 0);
        assert_eq!(app.test_track1.range_max(), 10);

        app.test_track1.set_pos(3);
        assert_eq!(app.test_track1.pos(), 3);
        
        app.test_track2.set_range_min(0);
        app.test_track2.set_range_max(5);
        app.test_track2.set_selection_range_pos(0..3);
        assert_eq!(app.test_track2.selection_range_pos(), 0..3);

        app.runs.borrow_mut().track = true;
    } else {
        app.runs.borrow_mut().track = false;
    }
}

fn run_tooltip_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().tooltip {

        app.test_ttp2.set_enabled(false);

        app.test_ttp1.set_delay_time(Some(100));
        assert_eq!(app.test_ttp1.delay_time(), 100);

        app.test_ttp1.register(&app.test_checkbox1, "A simple checkbox");
        app.test_ttp1.register(&app.test_checkbox2, "A checkbox with 3 states!");

        app.test_ttp3.set_default_decoration("Changed!", TooltipIcon::None);

        app.test_ttp1.set_text(&app.test_button.handle, "New tool tip!");
        assert_eq!(&app.test_ttp1.text(&app.test_button.handle, None), "New tool tip!");

        app.test_ttp1.unregister(&app.test_button);

        app.runs.borrow_mut().tooltip = true;
    } else {
        app.test_ttp1.register(&app.test_button, "A button");
        app.test_ttp2.set_enabled(true);
        app.runs.borrow_mut().tooltip = false;
    }
}

fn run_status_tests(app: &ControlsTest, _evt: Event) {
    if !app.runs.borrow().status {

        app.status.set_text(0, "Status changed!");
        assert_eq!(&app.status.text(0), "Status changed!");

        app.status.set_font(Some(&app.arial_font));
        assert_eq!(app.status.font().as_ref(), Some(&app.arial_font));

        app.status.set_min_height(55);

        app.runs.borrow_mut().status = true;
    } else {

        app.status.set_font(None);
        app.status.set_min_height(25);

        app.runs.borrow_mut().status = false;
    }
}

fn run_tray_tests(app: &ControlsTest) {
    app.tray_icon.set_visibility(false);
    app.tray_icon_2.set_icon(&app.window_icon);
    app.tray_icon_2.set_tip("Changed the toolip and the icon!");

    let icon = Some(&app.love_icon);
    let flags = Some(TrayNotificationFlags::USER_ICON | TrayNotificationFlags::SILENT | TrayNotificationFlags::LARGE_ICON);

    app.tray_icon_2.show("OH NO!", Some("Just a title"), flags, icon);
    app.tray_icon_2.show("I'm spamming the system tray popup!", Some("Just a title"), flags, icon);
    app.tray_icon_2.show("You can't stop me!!!!!", Some("Just a title (really)"), flags, Some(&app.window_icon));
}

fn set_tooltip_dynamic<'a>(app: &ControlsTest, handle: &ControlHandle, data: &ToolTipTextData) {
    if &app.window == handle {
        data.set_text(&format!("Control text: \"{}\"", app.window.text()));
    } else if &app.test_text_input == handle {
        data.set_text(&format!("Control text: \"{}\"", app.test_text_input.text()));
    }
}

fn tree_tests(app: &ControlsTest, handle: &ControlHandle) {
    let add = &app.test_tree_add == handle;
    let remove = &app.test_tree_remove == handle;

    if add {
        let text = app.test_tree_input.text();
        match app.test_tree.root() {
            Some(root) => match app.test_tree.selected_item() {
                None =>    { app.test_tree.insert_item(&text, Some(&root), TreeInsert::Last); },
                Some(i) => { app.test_tree.insert_item(&text, Some(&i), TreeInsert::Last); },
            },
            None => { app.test_tree.insert_item(&text, None, TreeInsert::Root); },
        }
    }

    if remove {
        match app.test_tree.selected_item() {
            Some(item) => { app.test_tree.remove_item(&item);},
            None => {}
        }
    }

    app.test_tree.set_focus();
}

#[cfg(feature = "file-dialog")]
fn open_file(app: &ControlsTest, _evt: Event) {
    if app.open_file_dialog.run(Some(&app.window)) {
        app.file_dialog_result.clear();
        if let Ok(file_names) = app.open_file_dialog.get_selected_items() {
            let mut names = String::new();
            for name in file_names {
                names.push_str(&name.into_string().unwrap());
                names.push_str("\r\n")
            }

            app.file_dialog_result.set_text(&names);
        }
    }
}

#[cfg(not(feature = "file-dialog"))]
fn open_file(_app: &ControlsTest, _evt: Event) {}

#[cfg(feature = "file-dialog")]
fn open_directory(app: &ControlsTest, _evt: Event) {
    if app.open_directory_dialog.run(Some(&app.window)) {
        app.file_dialog_result.clear();
        if let Ok(directory) = app.open_directory_dialog.get_selected_item() {
            app.file_dialog_result.set_text(&directory.into_string().unwrap());
        }
    }
}

#[cfg(not(feature = "file-dialog"))]
fn open_directory(_app: &ControlsTest, _evt: Event) {}

#[cfg(feature = "file-dialog")]
fn save_file(app: &ControlsTest, _evt: Event) {
    if app.save_file_dialog.run(Some(&app.window)) {
        app.file_dialog_result.clear();
        if let Ok(file) = app.save_file_dialog.get_selected_item() {
            app.file_dialog_result.set_text(&file.into_string().unwrap());
        }
    }
}

#[cfg(not(feature = "file-dialog"))]
fn save_file(_app: &ControlsTest, _evt: Event) {}

#[cfg(feature = "color-dialog")]
fn color_select(app: &ControlsTest) {
    if app.color_dialog.run(Some(&app.window)) {
        app.test_color_output.set_text(&format!("{:?}", app.color_dialog.color()))
    }
}

#[cfg(not(feature = "color-dialog"))]
fn color_select(_app: &ControlsTest) {}

#[cfg(feature = "font-dialog")]
fn font_select(app: &ControlsTest) {
    if app.font_dialog.run(Some(&app.window)) {
        app.test_font_output.set_text(&format!("{:?}", app.font_dialog.font()))
    }
}

#[cfg(not(feature = "font-dialog"))]
fn font_select(_app: &ControlsTest) {}

fn print_char(data: &EventData) {
    match data {
        EventData::OnChar(c) => println!("{:?}", c),
        _=>{}
    }
}

fn set_lv_sort(lv: &ListView, data: &EventData) {
    match data {
        EventData::OnListViewItemIndex { row_index: _, column_index } => {
            for i in 0..lv.column_len() {
                if *column_index != i {
                    lv.set_column_sort_arrow(i, None);
                } else {
                    match lv.column_sort_arrow(i) {
                        Some(ListViewColumnSortArrow::Up) | None => lv.set_column_sort_arrow(i, Some(ListViewColumnSortArrow::Down)),
                        Some(ListViewColumnSortArrow::Down) => lv.set_column_sort_arrow(i, Some(ListViewColumnSortArrow::Up)),
                    }
                }
            }
        },
        _ => {}
    }
}
