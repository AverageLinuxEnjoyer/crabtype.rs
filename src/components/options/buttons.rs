use super::super::word_button::view::WordButton;
use crate::global_state::options_action::OptionsAction;
use crate::global_state::state::{AppState, StateAction};
use yew::prelude::*;

// This does work, but it's kinda ugly
// TODO:
// Rework the entire structure of these functions
// Extract multiple buttons that share state into a single radio buttons component
// Use templates
// etc.

pub fn get_language_buttons(state: UseReducerHandle<AppState>) -> Html {
    let state_clone = state.clone();
    state
        .languages
        .iter()
        .enumerate()
        .map(|(i, lang)| {
            let state_clone = state_clone.clone();

            let onclick = Callback::from(move |_| {
                state_clone.dispatch(StateAction::OptionsAction(OptionsAction::SetLanguage(i)));
            });

            let selected = state.selected_language == i;

            html! {
                <WordButton {onclick} label={lang.to_string()} {selected}/>
            }
        })
        .collect::<Html>()
}

pub fn get_timer_buttons(state: UseReducerHandle<AppState>) -> Html {
    let state_clone = state.clone();
    state
        .timers
        .iter()
        .enumerate()
        .map(|(i, timer)| {
            let state_clone = state_clone.clone();

            let onclick = Callback::from(move |_| {
                state_clone.dispatch(StateAction::OptionsAction(OptionsAction::SetTimer(i)));
            });

            let selected = state.selected_timer == i;

            html! {
                <WordButton {onclick} label={timer.to_string()} {selected}/>
            }
        })
        .collect::<Html>()
}

pub fn get_capitalization(state: UseReducerHandle<AppState>) -> Html {
    let state_clone = state.clone();
    let onclick = Callback::from(move |_| {
        state_clone.dispatch(StateAction::OptionsAction(
            OptionsAction::ToggleCapitalization,
        ));
    });

    let selected = state.capitalization;

    html! {
        <WordButton {onclick} label={"capitalization"} {selected}  />
    }
}

pub fn get_punctuation(state: UseReducerHandle<AppState>) -> Html {
    let state_clone = state.clone();
    let onclick = Callback::from(move |_| {
        state_clone.dispatch(StateAction::OptionsAction(OptionsAction::TogglePunctuation));
    });

    let selected = state.punctuation;

    html! {
        <WordButton {onclick} label={"punctuation"} {selected}  />
    }
}
