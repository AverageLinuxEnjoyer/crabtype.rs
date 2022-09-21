use super::super::restart_button::view::RestartButton;
use super::super::words_container::view::WordsContainer;
use crate::components::typing::words_state::{
    event_listener::use_key_listener_effect,
    state::{State, StateAction},
    words_action::WordsAction,
};
use chrono::prelude::*;
use gloo::net::http::Request;
use serde_json::{Result, Value};
use stylist::{yew::styled_component, Style};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[styled_component(TypingContainer)]
pub fn typing_container() -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let state = State::new_reducer();

    let timer = use_state(|| Local::now());

    let state_clone = state.clone();
    let timer_clone = timer.clone();
    let onclick = Callback::from(move |_| {
        let state = state_clone.clone();
        let timer = timer_clone.clone();
        spawn_local(async move {
            let res = Request::post("http://127.0.0.1:3000/words")
                .body(
                    r#"
                {
                    "language":  "english_1k",
                    "count": 50,
                    "with_uppercase": true,
                    "with_punctuation": true
                }
                "#,
                )
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let res: Value = serde_json::from_str(&res).unwrap();
            let v = res.as_array().unwrap();
            let v = v
                .iter()
                .map(|val| val.as_str().unwrap().to_string())
                .collect::<Vec<_>>();

            state.dispatch(StateAction::WordsAction(WordsAction::ResetWords(v)));
            timer.set(Local::now());
        });
    });

    use_key_listener_effect(state.clone());

    let duration = Local::now().signed_duration_since(*timer).num_seconds();

    html! {
        <main class={style}>
            {duration}
            <WordsContainer state={state.clone()} />
            <RestartButton {onclick} />
        </main>
    }
}
