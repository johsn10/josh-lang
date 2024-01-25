mod text_input;
mod interpreter;
mod memory;
mod types;

use gloo_console::log;
use interpreter::InterpreterResult;
use yew::prelude::*;
use crate::text_input::*;
use interpreter::Interpreter;
use interpreter::InterpreterErr;
use std::result::Result::*;


fn run_interpreter(mut interpreter: Interpreter) {
    match interpreter.run() {
        Ok(result) => {
            match result {
                InterpreterResult::AddOverflow => log!("Stopped on adding overflow"),
                InterpreterResult::NonExistingMem => log!("Stopped on trying to get not existing memory"),
                InterpreterResult::IndexOveflow => log!("Index got overflowed"),
                InterpreterResult::TooLong => log!("Runned too long"),
                InterpreterResult::Ok => log!("Finished ok"),
            }
            interpreter.memory.log_memory();
        },
        Err(interpreter_err) => {
            match interpreter_err {
                InterpreterErr::ErrLine(line) => log!(format!("Runtime error on line: {}", line)),
                InterpreterErr::Err => panic!("Error: Interpreter returned Err"),
            }
        },
    }
}

#[function_component]
fn App() -> Html {
    let textbox_input_handler = use_state(String::default);

    let on_textbox_input = Callback::from(|input: String| {
        let mut interpreter = Interpreter::new(100);
        match interpreter.memory.store_instruction_str(&input) {
            Ok(_) => run_interpreter(interpreter),
            Err(line) => log!(format!("Error on line: {}", line)),
        };
    });
    
    html! {
        <div class="window">
            <h1 class="title"> {"WASM Interpreter"} </h1>            
            <TextInput on_textbox_input={on_textbox_input}/>
            <p>{textbox_input_handler.to_string()}</p>
        </div>
    }
}



fn main() {
    yew::Renderer::<App>::new().render();
}