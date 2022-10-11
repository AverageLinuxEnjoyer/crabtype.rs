use super::super::restart_button::view::RestartButton;
use super::super::timer::view::Timer;
use super::super::words_container::view::WordsContainer;
use crate::{
    components::typing::event_listener::use_key_listener_effect,
    fetch::words::fetch_words,
    global_state::{
        state::{AppContext, StateAction},
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

    let fetched_words = use_async(async move { fetch_words("english_1k", true, true).await });

    if fetched_words.loading {
        state.dispatch(StateAction::WordsAction(WordsAction::SetLoaded(false)));
    } else if fetched_words.data.is_some() && !state.loaded {
        state.dispatch(StateAction::WordsAction(WordsAction::ResetWords(
            fetched_words.data.as_ref().unwrap().clone(),
        )));

        state.dispatch(StateAction::WordsAction(WordsAction::SetLoaded(true)));
    }

    let onclick = Callback::from(move |_| {
        fetched_words.run();
    });

    use_key_listener_effect(state.clone());

    let loaded = if state.loaded { "loaded" } else { "loading" };

    html! {
        <main class={classes!(style, loaded)}>
            <Timer />
            <WordsContainer state={state.clone()} />
            <RestartButton {onclick} />
        </main>
    }
}
