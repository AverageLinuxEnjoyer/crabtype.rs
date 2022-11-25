use super::super::restart_button::view::RestartButton;
use super::super::timer::view::Timer;
use super::super::words_container::view::WordsContainer;
use crate::{
    components::typing::event_listener::use_key_listener_effect,
    fetch::words::{fetch_words, try_load_words},
    global_state::{
        state::{AppContext, StateAction},
        typing_state::TypingStatus,
        words_action::WordsAction,
    },
};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[styled_component(TypingContainer)]
pub fn typing_container() -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let state = use_context::<AppContext>().expect("No state context found");

    let state_clone = state.clone();
    let fetched_words = use_async(async move { fetch_words(state_clone).await });

    try_load_words(fetched_words.clone(), state.clone());

    let onclick = Callback::from(move |_| {
        fetched_words.run();
    });

    use_key_listener_effect(state.clone());

    let status = match state.typing.status {
        TypingStatus::Started => "started",
        TypingStatus::Finished => "finished",
        TypingStatus::NotStarted => "not-started",
    };

    let loaded = match state.typing.loaded {
        true => "loaded",
        false => "loading",
    };

    html! {
        <main class={classes!(style, loaded, status)}>
            <Timer />
            <WordsContainer state={state.clone()} />
            <RestartButton {onclick} />
        </main>
    }
}
