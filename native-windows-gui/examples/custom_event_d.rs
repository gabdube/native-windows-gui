/*!
    Shows how to use the `CustomEvent` control.

    In this example, we have a list of buttons implemented as a partial. Instead on binding the same callback
    on each control in the parent UI, the partial defines an event that is raised internally by the partial.

    That event is then catched in the parent window.
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::{NwgUi, NwgPartial};
use nwg::{NativeUi, stretch};
use stretch::geometry::Size;
use nwg::stretch::{style::{*, Dimension::*}, geometry::Rect};

use std::cell::RefCell;
use std::rc::Rc;


#[derive(Default, NwgPartial)]
pub struct ButtonList {
    pub use_later: Rc<RefCell<String>>,

    // Collection for the dynamic buttons
    buttons: RefCell<Vec<nwg::Button>>,
    handlers: RefCell<Vec<nwg::EventHandler>>,

    // Custom event that will be raised by this partial
    #[nwg_control]
    pub buttons_clicked: nwg::CustomEvent,

    // Outer layout/frame to contain the buttons
    #[nwg_layout]
    grid: nwg::GridLayout,

    #[nwg_control(flags: "VISIBLE")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0)]
    frame: nwg::Frame,

    // Inner layout for the buttons
    #[nwg_layout(parent: frame, auto_spacing: None, flex_direction: FlexDirection::Column)]
    button_grid:  nwg::FlexboxLayout,

    // A button that adds buttons to this list
    #[nwg_control(parent: frame, text: "Add a button")]
    #[nwg_layout_item(layout: button_grid, flex_shrink: 0.0, size: Size { width: Percent(1.0), height: Points(45.0) } )]
    #[nwg_events( OnButtonClick: [ButtonList::add_button] )]
    button1: nwg::Button,
}

impl ButtonList {

    pub fn add_button(&self) {
        let mut buttons = self.buttons.borrow_mut();
        let mut handlers = self.handlers.borrow_mut();

        let button_index = buttons.len();
        let button_name = format!("Button {}", button_index + 1);

        // Button creation
        buttons.push(nwg::Button::default());
        nwg::Button::builder()
            .text(&button_name)
            .parent(&self.frame)
            .build(&mut buttons[button_index]).expect("Failed to create button");

        // Adding the button to the layout
        let style = Style {
            size: Size { width: Auto, height: Points(100.0) },
            margin: Rect { top: Points(5.0), ..Default::default() },
            justify_content: JustifyContent::Center,
            ..Default::default()
        };

        self.button_grid.add_child(&buttons[button_index], style).expect("Failed to add button to layout");
        
        // Binding an handler
        let current_value = self.use_later.clone();
        let button_clicked_event = self.buttons_clicked.clone();
        let button_handle = buttons[button_index].handle;

        let handler = nwg::bind_event_handler(&buttons[button_index].handle, &self.frame.handle, move |event, _data, handle| {
            if event == nwg::Event::OnButtonClick && handle == button_handle {
                *current_value.borrow_mut() = button_name.clone();
                button_clicked_event.trigger();
            }
        });

        handlers.push(handler);

    }

}

impl Drop for ButtonList {

    fn drop(&mut self) {
        let mut handlers = self.handlers.borrow_mut();
        for handler in handlers.drain(..) {
            nwg::unbind_event_handler(&handler)
        }
    }

}


#[derive(Default, NwgUi)]
pub struct CustomEventApp {
    #[nwg_control(size: (300, 300), center: true, title: "Custom event example")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    // Using `nwg_events` we catch when the partial trigger the event instead on adding a callback for every
    // button in the partial. This also allows the partial to add buttons dynamically.
    #[nwg_partial(parent: window)]
    #[nwg_events((buttons_clicked, OnCustomEvent): [CustomEventApp::display_value])]
    partial: ButtonList,    
}

impl CustomEventApp {

    pub fn display_value(&self) {
        let value = self.partial.use_later.borrow();
        let msg = format!("Here's your selected value: {}", &value);
        nwg::modal_info_message(&self.window, "Selected value", &msg);
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = CustomEventApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
