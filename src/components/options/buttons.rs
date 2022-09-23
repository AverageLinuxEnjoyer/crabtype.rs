use yew::prelude::*;
use super::state::State;
use super::super::word_button::view::WordButton;


// This does work, but it's kinda ugly
// TODO:
// Rework the entire behaviour of these functions
// Extract multiple buttons that share state into a single radio buttons component
// Use templates
// etc.

pub fn get_language_buttons(state: UseStateHandle<State>) -> Html {
    let state_clone = state.clone();
    state.languages.iter().enumerate().map(|(i, lang)| {
        let state_clone = state_clone.clone();

        let onclick = Callback::from(move |_| {
            let mut state_clone2 = (*state_clone).clone();
            state_clone2.selected_language = i;
            state_clone.set(state_clone2);
        });

        let selected = state.selected_language == i;

        html! {
            <WordButton {onclick} label={lang.to_string()} {selected}/>
        }
    }).collect::<Html>()
}

pub fn get_timer_buttons(state: UseStateHandle<State>) -> Html {
    let state_clone = state.clone();
    state.timers.iter().enumerate().map(|(i, timer)| {
        let state_clone = state_clone.clone();

        let onclick = Callback::from(move |_| {
            let mut state_clone2 = (*state_clone).clone();
            state_clone2.selected_timer = i;
            state_clone.set(state_clone2);
        });

        let selected = state.selected_timer == i;

        html! {
            <WordButton {onclick} label={timer.to_string()} {selected}/>
        }
    }).collect::<Html>()
}

pub fn get_capitalization(state: UseStateHandle<State>) -> Html {
    let state_clone = state.clone();
    let onclick = Callback::from(move |_| {
        let mut state_clone2 = (*state_clone).clone();
        state_clone2.capitalization = !state_clone.capitalization;
        state_clone.set(state_clone2)
    });

    let selected = state.capitalization;

    html! {
        <WordButton {onclick} label={"capitalization"} {selected}  />
    }
}

pub fn get_punctuation(state: UseStateHandle<State>) -> Html {
    let state_clone = state.clone();
    let onclick = Callback::from(move |_| {
        let mut state_clone2 = (*state_clone).clone();
        state_clone2.punctuation = !state_clone.punctuation;
        state_clone.set(state_clone2)
    });

    let selected = state.punctuation;

    html! {
        <WordButton {onclick} label={"punctuation"} {selected}  />
    }
}