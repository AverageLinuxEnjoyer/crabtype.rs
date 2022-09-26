use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<()>,
    pub label: String,
    pub selected: bool,
}

#[styled_component(WordButton)]
pub fn word_button(props: &Props) -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let props_onclick = props.onclick.clone();
    let onclick = Callback::from(move |_| {
        props_onclick.emit(());
    });

    let selected = if props.selected { "selected" } else { "" };

    html! {
        <button type={"button"} {onclick} class={classes!(style, selected)}>
            {&props.label}
        </button>
    }
}
