use super::state::AppState;
use crate::components::typing::word_data::WordData;
use std::cell::RefCell;

pub enum WordsAction {
    ResetWords(Vec<String>),
    AddWords(Vec<String>),
    SetLoaded(bool),
}

pub fn handle_words_action(action: WordsAction, state: &mut AppState) {
    use WordsAction::*;

    match action {
        ResetWords(words) => {
            state.current_letter_index = 0;
            state.current_word_index = 0;
            state.words = RefCell::new(
                words
                    .into_iter()
                    .map(|w| WordData {
                        written: None,
                        target: w,
                    })
                    .collect(),
            )
            .into();

            state.started = false;
            state.countdown = state.timers[state.selected_timer as usize];
        }
        AddWords(words) => {
            state
                .words
                .borrow_mut()
                .extend(words.into_iter().map(|w| WordData {
                    written: None,
                    target: w,
                }));
        }
        SetLoaded(value) => state.loaded = value,
    }
}
