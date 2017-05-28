#![allow(non_upper_case_globals)]
/**
    Simple example on how to create controls on the fly. 
    The example shows a dialog that add dynamically add buttons in the main window.
*/


#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Ui, simple_message, fatal_message, dispatch_events};
use nwg::events as nwge;
use nwg::constants::HTextAlign;

// Identifiers for static controls/resources/values
const MainWindow: usize = 0;
const ControlsGroup: usize = 1;
const AddNewButton: usize = 2;
const ClearButtons: usize = 3;
const Label1: usize = 4;
const Label2: usize = 5;
const TextSelect: usize = 6;
const MsgSelect: usize = 7;

const TextFont: usize = 20;

const ButtonList: usize = 30;
const NextId: usize = 31;

const AddCallback: usize = 40;
const ClearCallback: usize = 41;

// Identifiers for the dynamic buttons
const MinDynamicButtonsId: usize = 1000;

nwg_template!(
    head: setup_ui<usize>,
    controls: [
        (MainWindow, nwg_window!( title="Button creator!"; size=(400, 305); resizable=true )),
        (ControlsGroup, nwg_groupbox!(parent=MainWindow; position=(5, 5); align=HTextAlign::Center; size=(390, 130); text="Controls"; font=Some(TextFont))),
        (AddNewButton, nwg_button!(parent=ControlsGroup; position=(235, 95); size=(150, 30); text="Create button"; font=Some(TextFont))),
        (ClearButtons, nwg_button!(parent=ControlsGroup; position=(80, 95); size=(150, 30); text="Clear buttons"; font=Some(TextFont))),
        (Label1, nwg_label!(parent=ControlsGroup; position=(48, 20); size=(80, 25); text="New text:"; font=Some(TextFont))),
        (Label2, nwg_label!(parent=ControlsGroup; position=(10, 50); size=(100, 25); text="New message:"; font=Some(TextFont))),
        (TextSelect, nwg_textinput!(parent=ControlsGroup; position=(130, 20); size=(250, 22); text="Click me!"; font=Some(TextFont))),
        (MsgSelect, nwg_textinput!(parent=ControlsGroup; position=(130, 50); size=(250, 22); text="Hello World!"; font=Some(TextFont)))
    ];
    events: [
        (AddNewButton, AddCallback, nwge::button::Click, |ui,_,_,_| {
            // Borrow references to the controls and values that will be used.
            let (mut button_list, mut next_id) = nwg_get_mut!(ui; [
                (ButtonList, Vec<usize>),
                (NextId, usize)
            ]);

            let (text, msg) = nwg_get!(ui; [
                (TextSelect, nwg::TextInput),
                (MsgSelect, nwg::TextInput)
            ]);

            // Add a new button to the Ui
            let new_id = **next_id;
            let height_offset = 140 + (30 * (button_list.len() / 2)) as i32;
            let width_offset = if new_id % 2 == 0 { 5 } else { 200 };
            let t = nwg::ButtonT {
                text: text.get_text(),
                position: (width_offset, height_offset), size: (195, 30), 
                visible: true, disabled: false, 
                parent:  MainWindow, font: Some(TextFont)
            };
            ui.pack_control(&new_id, t);

            // Bind a callback
            let msg_txt = msg.get_text();
            let next_event_id = new_id+1000;
            ui.bind(&new_id, &next_event_id, nwge::button::Click, move |_,_,_,_|{
                simple_message("Dynamic button!", &msg_txt);
            });

            // Save the id to clear the buttons later
            button_list.push(new_id);

            // Increase the id counter
            **next_id += 1;

            // pack & bind will be processed by the event loop after this closure returns.
        }),

        (ClearButtons, ClearCallback, nwge::button::Click, |ui,_,_,_| {
            let (mut button_list, mut next_id) = nwg_get_mut!(ui; [
                (ButtonList, Vec<usize>),
                (NextId, usize)
            ]);

            for id in (*button_list).iter() {
                ui.unpack(id);
            }

            // Reset the existing ids to default
            button_list.clear();
            **next_id = MinDynamicButtonsId;
        })
    ];
    resources: [
        (TextFont, nwg_font!(family="Arial"; size=17))
    ];
    values: [
        (ButtonList, Vec::<usize>::with_capacity(16)),  // The "nwg way" assumes global are evil, especially if they are mutable
        (NextId, MinDynamicButtonsId)                   // Any variable that needs to be accessible to a UI should be stored in the values.
    ]
);

fn main() {
    let app: Ui<usize>;

    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    if let Err(e) = setup_ui(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    dispatch_events();
}
