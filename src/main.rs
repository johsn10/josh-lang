mod text_input;
mod interpreter;
mod memory;

use gloo_console::log;
use yew::prelude::*;
use crate::text_input::*;
use interpreter::Interpreter;



#[function_component]
fn App() -> Html {
    let textbox_input_handler = use_state(String::default);
    let on_textbox_input = Callback::from(|input: String| {
        let mut interpreter = Interpreter::new();
        match interpreter.memory.instructions_string_to_instructions(&input) {
            None => log!("Not valid"),
            Some(()) => log!("Valid"),
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