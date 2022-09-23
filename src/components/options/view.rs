use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use crate::components::options::buttons::{get_language_buttons, get_timer_buttons, get_capitalization, get_punctuation};
use crate::components::options::state::State;
use crate::components::word_button::view::WordButton;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<()>,
}

#[styled_component(Options)]
pub fn options() -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let state = use_state(State::default);

    let language_buttons = get_language_buttons(state.clone());
    let timer_buttons = get_timer_buttons(state.clone());
    let capitalization_button = get_capitalization(state.clone());
    let punctuation_button = get_punctuation(state.clone());

    html! {
        <options class={style}>
            {language_buttons}
            {capitalization_button}
            {punctuation_button}
            {timer_buttons}
        </options>
    }
}
