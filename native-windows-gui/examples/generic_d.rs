/*!
    A very simple application which suggests to guess a random number.
    It shows how derive macro parses generics.

    Requires the following features: `cargo run --example generic_d --features "combobox"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::fmt::Display;
use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(NwgUi)]
pub struct GuessApp<VALIDATOR, T: Display + Default + 'static, const W: i32, const H: i32>
    where VALIDATOR: Fn(Option<&T>) -> Result<String, String> + 'static {
    #[nwg_control(size: (W, H), position: (300, 300), title: "Guess the number", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [nwg::stop_thread_dispatch()])]
    window: nwg::Window,

    #[nwg_control(collection: data.combo_items.borrow_mut().take().unwrap_or_default(), size: (280, 40), position: (10, 10))]
    combobox: nwg::ComboBox<T>,
    combo_items: RefCell<Option<Vec<T>>>,

    #[nwg_control(text: "Check", size: (280, 35), position: (10, 60))]
    #[nwg_events(OnButtonClick: [GuessApp::guess])]
    button: nwg::Button,

    validator: VALIDATOR,
}

impl<VALIDATOR, T, const W: i32, const H: i32> GuessApp<VALIDATOR, T, W, H>
    where VALIDATOR: Fn(Option<&T>) -> Result<String, String>,
          T: Display + Default {

    fn guess(&self) {
        let validation = match self.combobox.selection() {
            Some(s) => (self.validator)(self.combobox.collection().get(s)),
            None => Err("Please select any value".to_owned()),
        };
        match validation {
            Err(error) => { nwg::modal_error_message(&self.window, "Fail", &error); }
            Ok(success) => {
                nwg::modal_info_message(&self.window, "Congratulation", &success);
                nwg::stop_thread_dispatch();
            }
        };
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let random_number = (SystemTime::now()
        .duration_since(UNIX_EPOCH).expect("Clock may have gone backwards")
        .as_millis() % 100) as i8;

    let validator = move |c: Option<&i8>| {
        c.filter(|x| **x == random_number)
            .map(|x| format!("You guessed my number: {}", *x))
            .ok_or("Wrong number. Try again".to_owned())
    };

    let combo_items = (-2..=2).into_iter().map(|i| random_number + i).collect();

    const WIDTH: i32 = 300;
    const HEIGHT: i32 = 110;
    let basic_app = GuessApp::<_, _, WIDTH, HEIGHT> {
        validator,
        combo_items: Some(combo_items).into(),
        window: Default::default(),
        button: Default::default(),
        combobox: Default::default(),
    };
    let _ui = GuessApp::build_ui(basic_app).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
