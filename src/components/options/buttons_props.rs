use yew::prelude::*;

use crate::global_state::{
    options_action::OptionsAction,
    state::{AppState, StateAction},
};

pub fn get_language_props(state: UseReducerHandle<AppState>) -> (Vec<String>, Callback<usize>) {
    let language_labels = state.options.languages.clone();
    let language_onclick = Callback::from(move |selected| {
        state.dispatch(StateAction::OptionsAction(OptionsAction::SetLanguage(
            selected,
        )));
    });

    (language_labels, language_onclick)
}

pub fn get_capitalization_props(state: UseReducerHandle<AppState>) -> Callback<bool> {
    Callback::from(move |_| {
        state.dispatch(StateAction::OptionsAction(
            OptionsAction::ToggleCapitalization,
        ));
    })
}

pub fn get_punctuation_props(state: UseReducerHandle<AppState>) -> Callback<bool> {
    Callback::from(move |_| {
        state.dispatch(StateAction::OptionsAction(OptionsAction::TogglePunctuation));
    })
}

pub fn get_timer_props(state: UseReducerHandle<AppState>) -> (Vec<String>, Callback<usize>) {
    let timer_labels = state
        .options
        .timers
        .iter()
        .map(|timer| timer.to_string())
        .collect::<Vec<String>>();
    let timer_onclick = Callback::from(move |selected| {
        state.dispatch(StateAction::OptionsAction(OptionsAction::SetTimer(
            selected,
        )));
    });

    (timer_labels, timer_onclick)
}
