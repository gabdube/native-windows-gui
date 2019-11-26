/*!
    A calculator that use the grid layout of NWG. Macro version.
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Mult,
    Div
}


#[derive(Default, NwgUi)]
pub struct Calculator {

    #[nwg_control(size: (300, 150), position: (300, 300), title: "Calculator")]
    #[nwg_events( OnWindowClose: [Calculator::exit] )]
    window: nwg::Window,

    //layout: GridLayout,

    #[nwg_control(text: "", align: nwg::HTextAlign::Right, readonly: true)]
    input: nwg::TextInput,

    #[nwg_control(text: "0")] 
    btn0: nwg::Button,

    #[nwg_control(text: "1")] 
    btn1: nwg::Button,

    #[nwg_control(text: "2")] 
    btn2: nwg::Button,

    #[nwg_control(text: "3")] 
    btn3: nwg::Button,

    #[nwg_control(text: "4")] 
    btn4: nwg::Button,

    #[nwg_control(text: "5")] 
    btn5: nwg::Button,

    #[nwg_control(text: "6")] 
    btn6: nwg::Button,

    #[nwg_control(text: "7")] 
    btn7: nwg::Button,

    #[nwg_control(text: "8")] 
    btn8: nwg::Button,

    #[nwg_control(text: "9")] 
    btn9: nwg::Button,

    #[nwg_control(text: "+")] 
    btn_plus: nwg::Button,

    #[nwg_control(text: "-")] 
    btn_minus: nwg::Button,

    #[nwg_control(text: "*")] 
    btn_mult: nwg::Button,

    #[nwg_control(text: "/")] 
    btn_divide: nwg::Button,

    #[nwg_control(text: "=")] 
    btn_process: nwg::Button,

    #[nwg_control(text: "Clear")]
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


fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = Calculator::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
