use super::super::word_radio_buttons::view::WordRadioButtons;
use super::super::word_toggle_button::view::WordToggleButton;
use crate::components::options::buttons_props::{
    get_capitalization_props, get_language_props, get_punctuation_props, get_timer_props,
};
use crate::fetch::words::{fetch_words, try_load_words};
use crate::global_state::options_action::OptionsAction;
use crate::global_state::state::{AppContext, StateAction};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_hooks::use_async;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<()>,
}

#[styled_component(Options)]
pub fn options() -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let state = use_context::<AppContext>().expect("No state context found");

    let (language_labels, language_onclick) = get_language_props(state.clone());
    let capitalization_onclick = get_capitalization_props(state.clone());
    let punctuation_onclick = get_punctuation_props(state.clone());
    let (timer_labels, timer_onclick) = get_timer_props(state);

    let language_onclick = Callback::from(move |i: usize| {
        language_onclick.emit(i);
    });

    html! {
        <options class={style}>
            <WordRadioButtons labels={language_labels} onclick={language_onclick} />
            <WordToggleButton label={"capitalization".to_string()} onclick={capitalization_onclick} />
            <WordToggleButton label={"punctuation".to_string()} onclick={punctuation_onclick} />
            <WordRadioButtons labels={timer_labels} onclick={timer_onclick} />
        </options>
    }
}
