use super::super::restart_button::view::RestartButton;
use super::super::words_container::view::WordsContainer;
use crate::{
    components::typing::event_listener::use_key_listener_effect,
    global_state::{
        state::{AppContext, StateAction},
        words_action::WordsAction,
    },
};
use chrono::prelude::*;
use gloo::{console::log, net::http::Request};
use serde_json::Value;
use stylist::{yew::styled_component, Style};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[styled_component(TypingContainer)]
pub fn typing_container() -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let state = use_context::<AppContext>().expect("No state context found");

    let state_clone = state.clone();
    let onclick = Callback::from(move |_| {
        let state = state_clone.clone();
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
                .expect("Error connecting to the server.") // TODO: handle this unwrap
                .text()
                .await
                .unwrap(); // TODO: this one as well

            // TODO: handle these unwraps
            let res: Value = serde_json::from_str(&res).unwrap();
            let v = res.as_array().unwrap();
            let v = v
                .iter()
                .map(|val| val.as_str().unwrap().to_string())
                .collect::<Vec<_>>();

            state.dispatch(StateAction::WordsAction(WordsAction::ResetWords(v)));
        });
    });

    use_key_listener_effect(state.clone());

    html! {
        <main class={style}>
            <WordsContainer state={state.clone()} />
            <RestartButton {onclick} />
        </main>
    }
}
