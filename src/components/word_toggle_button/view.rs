use super::super::word_button::view::WordButton;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub selected: Option<bool>,
    pub onclick: Callback<bool>,
}

#[styled_component(WordToggleButton)]
pub fn word_toggle_button(props: &Props) -> Html {
    let selected = use_state(|| props.selected.unwrap_or(false));

    let label = props.label.clone();

    let selected_copy = selected.clone();
    let onclick = props.onclick.clone();

    let onclick = Callback::from(move |_| {
        selected_copy.set(!*selected_copy);
        onclick.emit(*selected_copy);
    });

    html! {
        <WordButton {onclick} {label} selected={*selected} />
    }
}
