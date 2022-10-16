use super::super::word_button::view::WordButton;
use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub labels: Vec<String>,
    pub selected: Option<usize>,
    pub onclick: Callback<usize>,
}

#[styled_component(WordRadioButtons)]
pub fn word_radio_buttons(props: &Props) -> Html {
    let selected = use_state(|| props.selected.unwrap_or(0).clamp(0, props.labels.len()));

    props
        .labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let selected_clone = selected.clone();
            let onclick = props.onclick.clone();
            let onclick = Callback::from(move |_| {
                selected_clone.set(i);
                onclick.emit(i);
            });

            let selected = *selected == i;
            html! {
                <WordButton {onclick} label={label.clone()} {selected} />
            }
        })
        .collect::<Html>()
}
