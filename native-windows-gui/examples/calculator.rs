/*!
    A calculator that use the grid layout of NWG.
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Mult,
    Div
}


#[derive(Default)]
pub struct Calculator {
    window: nwg::Window,

    layout: nwg::GridLayout,

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

    fn number(&self, button: &nwg::Button) {
        let text = self.input.text();
        self.input.set_text(&format!("{}{}", text, button.text()));
    }

    fn clear(&self) {
        self.input.set_text("");
    }

    fn compute(&self) {
        use Token::*;
        static SYMBOLS: &'static [char] = &['+', '-', '*', '/'];

        let eq = self.input.text();
        if eq.len() == 0 {
            return;
        }

        let mut tokens: Vec<Token> = Vec::with_capacity(5);
        let mut last = 0;

        for (i, chr) in eq.char_indices() {
            if SYMBOLS.iter().any(|&s| s == chr) {
                let left = &eq[last..i];
                match left.parse::<i32>() {
                    Ok(i) => tokens.push(Token::Number(i)),
                    _ => {
                        nwg::error_message("Error", "Invalid equation!");
                        self.input.set_text("");
                        return
                    }
                }

                let tk = match chr {
                    '+' => Plus,
                    '-' => Minus,
                    '*' => Mult,
                    '/' => Div,
                    _ => unreachable!()
                };

                tokens.push(tk);

                last = i+1;
            }
        }

        let right = &eq[last..];
        match right.parse::<i32>() {
            Ok(i) => tokens.push(Token::Number(i)),
            _ =>  {
                nwg::error_message("Error", "Invalid equation!");
                self.input.set_text("");
                return
            }
        }

        let mut i = 1;
        let mut result = match &tokens[0] { Token::Number(n) => *n, _ => unreachable!() };
        while i < tokens.len() {
            match [&tokens[i], &tokens[i+1]] {
                [Plus, Number(n)] => { result += n; },
                [Minus, Number(n)] => { result -= n;},
                [Mult, Number(n)] => { result *= n; },
                [Div, Number(n)] => { result /= n; },
                _ => unreachable!()
            }
            i += 2;
        }

        self.input.set_text(&result.to_string());
    }

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
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct CalculatorUi {
        inner: Rc<Calculator>,
        default_handler: RefCell<Vec<nwg::EventHandler>>
    }

    impl nwg::NativeUi<CalculatorUi> for Calculator {
        fn build_ui(mut data: Calculator) -> Result<CalculatorUi, nwg::NwgError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .size((300, 150))
                .position((300, 300))
                .title("Calculator")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .text("")
                .align(nwg::HTextAlign::Right)
                .readonly(true)
                .parent(&data.window)
                .build(&mut data.input)?;

            nwg::Button::builder()
                .text("1")
                .parent(&data.window)
                .focus(true)
                .build(&mut data.btn1)?;
           
            nwg::Button::builder().text("2").parent(&data.window).build(&mut data.btn2)?;
            nwg::Button::builder().text("3").parent(&data.window).build(&mut data.btn3)?;
            nwg::Button::builder().text("4").parent(&data.window).build(&mut data.btn4)?;
            nwg::Button::builder().text("5").parent(&data.window).build(&mut data.btn5)?;
            nwg::Button::builder().text("6").parent(&data.window).build(&mut data.btn6)?;
            nwg::Button::builder().text("7").parent(&data.window).build(&mut data.btn7)?;
            nwg::Button::builder().text("8").parent(&data.window).build(&mut data.btn8)?;
            nwg::Button::builder().text("9").parent(&data.window).build(&mut data.btn9)?;
            nwg::Button::builder().text("0").parent(&data.window).build(&mut data.btn0)?;
            
            nwg::Button::builder().text("+").parent(&data.window).build(&mut data.btn_plus)?;
            nwg::Button::builder().text("-").parent(&data.window).build(&mut data.btn_minus)?;
            nwg::Button::builder().text("*").parent(&data.window).build(&mut data.btn_mult)?;
            nwg::Button::builder().text("/").parent(&data.window).build(&mut data.btn_divide)?;
            nwg::Button::builder().text("Clear").parent(&data.window).build(&mut data.btn_clear)?;
            nwg::Button::builder().text("=").parent(&data.window).build(&mut data.btn_process)?;
            
            
            // Wrap-up
            let ui = CalculatorUi {
                inner: Rc::new(data),
                default_handler: Default::default()
            };

            // Events
            let window_handles = [&ui.window.handle];
            for handle in window_handles.iter() {
                let evt_ui = Rc::downgrade(&ui.inner);
                let handle_events = move |evt, _evt_data, handle| {
                    if let Some(evt_ui) = evt_ui.upgrade() {
                        match evt {
                            E::OnButtonClick =>
                                if      &handle == &evt_ui.btn0 { Calculator::number(&evt_ui, &evt_ui.btn0); }
                                else if &handle == &evt_ui.btn1 { Calculator::number(&evt_ui, &evt_ui.btn1); }
                                else if &handle == &evt_ui.btn2 { Calculator::number(&evt_ui, &evt_ui.btn2); }
                                else if &handle == &evt_ui.btn3 { Calculator::number(&evt_ui, &evt_ui.btn3); }
                                else if &handle == &evt_ui.btn4 { Calculator::number(&evt_ui, &evt_ui.btn4); }
                                else if &handle == &evt_ui.btn5 { Calculator::number(&evt_ui, &evt_ui.btn5); }
                                else if &handle == &evt_ui.btn6 { Calculator::number(&evt_ui, &evt_ui.btn6); }
                                else if &handle == &evt_ui.btn7 { Calculator::number(&evt_ui, &evt_ui.btn7); }
                                else if &handle == &evt_ui.btn8 { Calculator::number(&evt_ui, &evt_ui.btn8); }
                                else if &handle == &evt_ui.btn9 { Calculator::number(&evt_ui, &evt_ui.btn9); }
    
                                else if &handle == &evt_ui.btn_plus { Calculator::number(&evt_ui, &evt_ui.btn_plus); }
                                else if &handle == &evt_ui.btn_minus { Calculator::number(&evt_ui, &evt_ui.btn_minus); }
                                else if &handle == &evt_ui.btn_mult { Calculator::number(&evt_ui, &evt_ui.btn_mult); }
                                else if &handle == &evt_ui.btn_divide { Calculator::number(&evt_ui, &evt_ui.btn_divide); }
    
                                else if &handle == &evt_ui.btn_clear { Calculator::clear(&evt_ui); }
    
                                else if &handle == &evt_ui.btn_process { Calculator::compute(&evt_ui); }
                            E::OnWindowClose => 
                                if &handle == &evt_ui.window {
                                    Calculator::exit(&evt_ui);
                                },
                            _ => {}
                        }
                    }
                };

                ui.default_handler.borrow_mut().push(
                    nwg::full_bind_event_handler(handle, handle_events)
                );
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
                .build(&ui.layout)?;
            
            return Ok(ui);
        }
    }

    impl Drop for CalculatorUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
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
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = Calculator::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
