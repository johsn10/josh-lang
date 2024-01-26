mod text_input;
mod interpreter;
mod memory;
mod types;
mod mem_table;

use crate::mem_table::MemTable;
use gloo_console::log;
use interpreter::InterpreterResult;
use yew::prelude::*;
use crate::text_input::*;
use interpreter::Interpreter;
use interpreter::InterpreterErr;
use std::result::Result::*;
use std::vec;


fn run_interpreter(interpreter: &mut Interpreter) {
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

fn get_vec_from_state(state: &UseStateHandle<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![0_i32;0];
    for num in state.iter() {
        result.push(*num);
    }
    result
}

#[function_component]
fn App() -> Html {
    let mem_handle = use_state(|| vec![0_i32;0]);
    let textbox_input_handler = use_state(String::default);

    let mem_handle_clone = mem_handle.clone();
    let on_textbox_input = Callback::from(move |input: String| {
        let mut interpreter = Interpreter::new(100);
        match interpreter.memory.store_instruction_str(&input) {
            Ok(_) => run_interpreter(&mut interpreter),
            Err(line) => log!(format!("Error on line: {}", line)),
        };
        mem_handle_clone.set(interpreter.memory.data);
    });

    html! {
        <div class="window">
            <h1 class="title"> {"WASM Interpreter"} </h1>            
            <TextInput on_textbox_input={on_textbox_input}/>
            <MemTable mem={get_vec_from_state(&mem_handle)}/>
            <p>{textbox_input_handler.to_string()}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}