/*!
    A calculator that use the grid layout of NWG.
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct Calculator {
    window: nwg::Window,

    //layout: GridLayout,

    input: nwg::TextInput,

    btn0: nwg::Button,
    btn1: nwg::Button,
    btn2: nwg::Button,
    btn3: nwg::Button,
    btn4: nwg::Button,
    btn5: nwg::Button,
    btn6: nwg::Button,
    btn7: nwg::Button,
    btn8: nwg::Button,
    btn9: nwg::Button,

    btn_plus: nwg::Button,
    btn_minus: nwg::Button,
    btn_mult: nwg::Button,
    btn_divide: nwg::Button,
    btn_process: nwg::Button,
    btn_clear: nwg::Button,
}

impl Calculator {

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

//
// ALL of this stuff is handled by native-windows-derive
//
mod calculator_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct CalculatorUi {
        inner: Calculator
    }

    impl nwg::NativeUi<Calculator, CalculatorUi> for Calculator {
        fn build_ui(mut data: Calculator) -> Result<Rc<CalculatorUi>, nwg::SystemError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .size((300, 150))
                .position((300, 300))
                .title("Calculator")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .text("")
                .readonly(true)
                .parent(&data.window)
                .build(&mut data.input)?;

            nwg::Button::builder().text("0").parent(&data.window).build(&mut data.btn0)?;
            nwg::Button::builder().text("1").parent(&data.window).build(&mut data.btn1)?;
            nwg::Button::builder().text("2").parent(&data.window).build(&mut data.btn2)?;
            nwg::Button::builder().text("3").parent(&data.window).build(&mut data.btn3)?;
            nwg::Button::builder().text("4").parent(&data.window).build(&mut data.btn4)?;
            nwg::Button::builder().text("5").parent(&data.window).build(&mut data.btn5)?;
            nwg::Button::builder().text("6").parent(&data.window).build(&mut data.btn6)?;
            nwg::Button::builder().text("7").parent(&data.window).build(&mut data.btn7)?;
            nwg::Button::builder().text("8").parent(&data.window).build(&mut data.btn8)?;
            nwg::Button::builder().text("9").parent(&data.window).build(&mut data.btn9)?;

            nwg::Button::builder().text("+").parent(&data.window).build(&mut data.btn_plus)?;
            nwg::Button::builder().text("-").parent(&data.window).build(&mut data.btn_minus)?;
            nwg::Button::builder().text("*").parent(&data.window).build(&mut data.btn_mult)?;
            nwg::Button::builder().text("/").parent(&data.window).build(&mut data.btn_divide)?;
            nwg::Button::builder().text("Done").parent(&data.window).build(&mut data.btn_process)?;
            nwg::Button::builder().text("Clear").parent(&data.window).build(&mut data.btn_clear)?;
            

            // Wrap-up
            let ui = Rc::new(CalculatorUi { inner: data });

            // Events
            let window_handles = [&ui.window.handle];
            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, _evt_data, handle| {
                    match evt {
                        E::OnWindowClose => {
                            if handle == evt_ui.window.handle {
                                Calculator::exit(&evt_ui.inner);
                            }
                        },
                        _ => {}
                    }
                };

                nwg::bind_event_handler(handle, handle_events);
            }

            // Layouts
            nwg::GridLayout::builder()
                .parent(&ui.window)
                .spacing(2)
                .min_size([150, 140])
                .child_item(nwg::GridLayoutItem::new(&ui.input, 0, 0, 5, 1))
                .child(0, 1, &ui.btn1)
                .child(1, 1, &ui.btn2)
                .child(2, 1, &ui.btn3)
                .child(0, 2, &ui.btn4)
                .child(1, 2, &ui.btn5)
                .child(2, 2, &ui.btn6)
                .child(0, 3, &ui.btn7)
                .child(1, 3, &ui.btn8)
                .child(2, 3, &ui.btn9)
                .child(3, 1, &ui.btn_plus)
                .child(4, 1, &ui.btn_minus)
                .child(3, 2, &ui.btn_mult)
                .child(4, 2, &ui.btn_divide)
                .child_item(nwg::GridLayoutItem::new(&ui.btn_clear, 3, 3, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.btn_process, 3, 4, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.btn0, 0, 4, 3, 1))
                .build();
            
            return Ok(ui);
        }
    }


    impl Deref for CalculatorUi {
        type Target = Calculator;

        fn deref(&self) -> &Calculator {
            &self.inner
        }
    }

}



fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = Calculator::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
