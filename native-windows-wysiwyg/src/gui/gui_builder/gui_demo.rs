use super::GuiBuilder;
use crate::parser::GuiStruct;


enum AllControls {
    Window(nwg::Window),
    Button(nwg::Button),
    TextInput(nwg::TextInput),
}

enum AllControlBuilders<'a> {
    Window(nwg::WindowBuilder<'a>),
    Button(nwg::ButtonBuilder<'a>),
    TextInput(nwg::TextInputBuilder<'a>),
}

impl<'a> AllControlBuilders<'a> {

    fn from_str(ty: &str) -> Result<AllControlBuilders<'a>, String> {
        let r = if ty.find("Window").is_some() {
            Ok(AllControlBuilders::Window(nwg::Window::builder()))
        } else if ty.find("Button").is_some() {
            Ok(AllControlBuilders::Button(nwg::Button::builder()))
        } else if ty.find("TextInput").is_some() {
            Ok(AllControlBuilders::TextInput(nwg::TextInput::builder()))
        } else {
            Err(format!("Failed to match type {:?}", ty))
        };

        r
    }

}

/// Holds a dynamically create gui from a `parser::GuiStruct`
pub(super) struct GuiDemo {
    controls: Vec<AllControls>,
}


/// Gui builder method to load a window in the demo window
impl GuiBuilder {

    /// Duh, don't close the demo window
    pub(super) fn show_demo_window(&self) {
        self.demo_window.set_visible(true);
    }

    /// Clear the current demo window in the UI (if there is one) and load `gui_struct`
    pub(super) fn build_demo_window(&self, gui_struct: &GuiStruct) {
        self.clear_demo_window();

        let mut controls = Vec::new();
        for member in gui_struct.members() {
            let builder = match AllControlBuilders::from_str(member.ty()) {
                Ok(builder) => builder,
                Err(e) => {
                    println!("Failed to load gui component {:?}: {}", member.name(), e);
                    continue;
                }
            };
        }

        let mut demo = self.demo.borrow_mut();
        let gui_demo = GuiDemo {
            controls
        };

        *demo = Some(gui_demo);

        println!("Demo loaded!");
    }

    pub(super) fn clear_demo_window(&self) {
        let mut demo = self.demo.borrow_mut();
        *demo = None;
    }

}
