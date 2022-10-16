use gloo::net::http::Request;
use serde_json::Value;
use yew::UseReducerHandle;
use yew_hooks::UseAsyncHandle;

use crate::global_state::{
    state::{AppState, StateAction},
    words_action::WordsAction,
};

pub async fn fetch_words(state: UseReducerHandle<AppState>) -> Result<Vec<String>, ()> {
    let language = state.languages[state.selected_language].clone();
    let capitalization = state.capitalization;
    let punctuation = state.punctuation;

    // enough words for 240 wpm
    let count = state.timers[state.selected_timer] * 3;

    let body = format!(
        "{{
            \"language\": \"{}\",
            \"count\": {},
            \"with_uppercase\": {},
            \"with_punctuation\": {}
        }}",
        language, count, capitalization, punctuation
    );
    let res = Request::post("http://127.0.0.1:3000/words");

    let body = res
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Error connecting to the server.") // TODO: handle this unwrap
        .text()
        .await
        .unwrap(); // TODO: this one as well

    // TODO: handle these unwraps
    let body: Value = serde_json::from_str(&body).unwrap();
    let arr = body.as_array().unwrap();
    let arr = arr
        .iter()
        .map(|val| val.as_str().unwrap().to_string())
        .collect::<Vec<_>>();

    Ok(arr)
}

pub fn try_load_words(
    fetched_words: UseAsyncHandle<Vec<String>, ()>,
    state: UseReducerHandle<AppState>,
) {
    if fetched_words.loading {
        state.dispatch(StateAction::WordsAction(WordsAction::SetLoaded(false)));
    } else if fetched_words.data.is_some() && !state.loaded {
        state.dispatch(StateAction::WordsAction(WordsAction::ResetWords(
            fetched_words.data.as_ref().unwrap().clone(),
        )));

        state.dispatch(StateAction::WordsAction(WordsAction::SetLoaded(true)));
    }
}
