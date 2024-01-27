use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MemTableProps {
    pub mem: Vec<i32>,
}

#[function_component]
pub fn MemTable(props: &MemTableProps) -> Html{
    let props = props.clone();
    let mut text = String::new();
    for num in props.mem {
        text = format!("{} {}", text, num.to_string());
    }
    html! {
        <p class="memory">{text}</p>
    }
}