use crate::components::options::buttons::{
    get_capitalization, get_language_buttons, get_punctuation, get_timer_buttons,
};
use crate::global_state::state::AppContext;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<()>,
}

#[styled_component(Options)]
pub fn options() -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let state = use_context::<AppContext>().expect("No state context found");

    // TODO: refactor these implementation
    let language_buttons = get_language_buttons(state.clone());
    let timer_buttons = get_timer_buttons(state.clone());
    let capitalization_button = get_capitalization(state.clone());
    let punctuation_button = get_punctuation(state);

    html! {
        <options class={style}>
            {language_buttons}
            {capitalization_button}
            {punctuation_button}
            {timer_buttons}
        </options>
    }
}
