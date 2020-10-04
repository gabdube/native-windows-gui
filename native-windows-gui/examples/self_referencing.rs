extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use std::cell::RefCell;
use std::rc::Rc;

use nwg::NativeUi;
use nwd::NwgUi;

use nwg::stretch::{geometry::{Size, Rect},style::{Dimension, FlexDirection, JustifyContent, Style},};

const PT_10: Dimension = Dimension::Points(10.0);
const PAD: Rect<Dimension> = Rect { start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };

#[derive(Default, NwgUi)]
pub struct MyApp {

    #[nwg_control(title: "test", size: (400, 300), position: (80, 60))]
    #[nwg_events(OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [MyApp::setup_combo_callback(RC_SELF)])]
    window: nwg::Window,
    
    #[nwg_layout(parent: window, padding: PAD, auto_spacing: None, flex_direction: FlexDirection::Column, justify_content: JustifyContent::Center)]
    main_layout: nwg::FlexboxLayout,

    #[nwg_control(text: "Add combobox")]
    #[nwg_layout_item(layout: main_layout, size: Size { width: Dimension::Percent(1.0), height: Dimension::Points(40.0)})]
    #[nwg_events(OnButtonClick: [MyApp::add_combobox])]
    button: nwg::Button,

    #[nwg_control(parent: window, flags: "VISIBLE")]
    #[nwg_layout_item(layout: main_layout, size: Size { width: Dimension::Percent(1.0), height: Dimension::Points(290.0)})]
    frame: nwg::Frame,

    #[nwg_layout(parent: frame, padding: PAD, auto_spacing: None, flex_direction: FlexDirection::Column)]
    frame_layout: nwg::FlexboxLayout,

    combo_box_handler: RefCell<Option<nwg::EventHandler>>,
    combo_boxes: RefCell<Vec<nwg::ComboBox<String>>>,
}

impl MyApp {

    fn setup_combo_callback(app_rc: &Rc<MyApp>) {
        // Clone the app and store it in our own callback
        let app = app_rc.clone();
        let handler = nwg::bind_event_handler(&app_rc.frame.handle, &app_rc.window.handle, move |evt, _evt_data, handle| {
            match evt {
                nwg::Event::OnComboxBoxSelection => { 

                    // Fetch the combobox here
                    let boxes = app.combo_boxes.borrow();
                    for (index, combo) in boxes.iter().enumerate() {
                        if combo.handle == handle {
                            println!("Accessed combobox #{}", index);
                        }
                    }

                },
                _ => {}
            }
        });

        *app_rc.combo_box_handler.borrow_mut() = Some(handler);
    }

    fn add_combobox(&self) {
        let mut new_dd: nwg::ComboBox<String> = nwg::ComboBox::<String>::default();
        let coll = vec![String::from("one"), String::from("two"), String::from("three")];
        nwg::ComboBox::builder()
            .collection(coll)
            .parent(&self.frame)
            .build(&mut new_dd)
            .expect("Failed to create token");

        let style = Style {
            size: Size { width: Dimension::Percent(1.0), height: Dimension::Points(40.0) },
            ..Default::default()
        };

        self.frame_layout
            .add_child(&new_dd, style)
            .expect("Failed to add token to layout");

        self.combo_boxes.borrow_mut().push(new_dd);
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = MyApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
