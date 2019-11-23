/*!
    A calculator that use the grid layout of NWG.
    It does not actually do anything though...
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct Calculator {
    window: nwg::Window,
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
                .size((300, 315))
                .position((300, 300))
                .title("The calculator")
                .build(&mut data.window)?;

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
