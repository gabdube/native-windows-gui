/*!
    Example on how to use custom types directly with native windows derive

    `cargo run --example subclassing_d --features "flexbox"`
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use nwg::stretch::style::FlexDirection;

type UserButton = nwg::Button;

#[allow(dead_code)]
#[derive(Default)]
pub struct CustomButton {
    base: nwg::Button,
    data: usize
}

// Implements default trait so that the control can be used by native windows derive
// The parameters are: subclass_control!(user type, base type, base field name)
nwg::subclass_control!(CustomButton, Button, base);

//
// Implement a builder API compatible with native window derive
//

impl CustomButton {
    fn builder<'a>() -> CustomButtonBuilder<'a> {
        CustomButtonBuilder {
            button_builder: nwg::Button::builder().text("Custom button with builder"),
            data: 0
        }
    }
}

pub struct CustomButtonBuilder<'a> {
    button_builder: nwg::ButtonBuilder<'a>,
    data: usize
}

impl<'a> CustomButtonBuilder<'a> {
    pub fn data(mut self, v: usize) -> CustomButtonBuilder<'a> {
        self.data = v;
        self
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> CustomButtonBuilder<'a> {
        self.button_builder = self.button_builder.parent(p);
        self
    }

    pub fn build(self, btn: &mut CustomButton) -> Result<(), nwg::NwgError> {
        self.button_builder.build(&mut btn.base)?;
        btn.data = self.data;
        Ok(())
    }
}

//
// Actual interface code
//

#[derive(Default, NwgUi)]
pub struct SubclassApp {
    #[nwg_control(size: (300, 300), position: (700, 300), title: "Subclass example")]
    #[nwg_events( OnWindowClose: [SubclassApp::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: "Simple button", focus: true)]
    #[nwg_layout_item(layout: layout)]
    #[nwg_events( OnButtonClick: [SubclassApp::button_click1] )]
    button1: nwg::Button,

    #[nwg_control(text: "User type button")]
    #[nwg_layout_item(layout: layout)]
    #[nwg_events( OnButtonClick: [SubclassApp::button_click2] )]
    button2: UserButton,

    #[nwg_control(ty: Button, text: "Subclassed button")]
    #[nwg_layout_item(layout: layout)]
    #[nwg_events( OnButtonClick: [SubclassApp::button_click3(SELF, CTRL)] )]
    button3: CustomButton,

    #[nwg_control(data: 100)]
    #[nwg_layout_item(layout: layout)]
    #[nwg_events( OnButtonClick: [SubclassApp::button_click3(SELF, CTRL)] )]
    button4: CustomButton,
}


impl SubclassApp {

    fn button_click1(&self) {
        nwg::modal_info_message(&self.window, "Simple button", "Hey, I'm a simple button!");
    }

    fn button_click2(&self) {
        nwg::modal_info_message(&self.window, "User type button", "Hey, I'm a button with a user type def!");
    }

    fn button_click3(&self, button: &CustomButton) {
        nwg::modal_info_message(&self.window, "Custom button", &format!("Hey, I'm a button with custom data! The data is {}!", button.data) );
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = SubclassApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
