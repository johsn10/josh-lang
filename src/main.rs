use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="window">
            <h1 class="title"> {"WASM Interpreter"} </h1>            
            <input type="text" class="input"/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}