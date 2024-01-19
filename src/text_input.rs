use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_textbox_input: Callback<String>,
}

#[function_component]
pub fn TextInput(props: &Props) -> Html {
    let input_value_handle = use_state(String::default);
    let props = (*props).clone();
    let on_input = {
        Callback::from(move |event: KeyboardEvent| {
            let input_value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            input_value_handle.set(input_value.clone());
            props.on_textbox_input.emit(input_value);
        })};

    html! {
        <textarea onkeyup={on_input} class="text-input" type="text"/>
    }
}