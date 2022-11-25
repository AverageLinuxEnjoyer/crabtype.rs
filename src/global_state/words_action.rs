use super::state::AppState;
use crate::{components::typing::word_data::WordData, global_state::typing_state::TypingStatus};
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
            state.typing.current_letter_index = 0;
            state.typing.current_word_index = 0;
            state.typing.words = RefCell::new(
                words
                    .into_iter()
                    .map(|w| WordData {
                        written: None,
                        target: w,
                    })
                    .collect(),
            )
            .into();

            state.typing.status = TypingStatus::NotStarted;
            state.typing.countdown = state.options.timers[state.options.selected_timer as usize];
        }
        AddWords(words) => {
            state
                .typing
                .words
                .borrow_mut()
                .extend(words.into_iter().map(|w| WordData {
                    written: None,
                    target: w,
                }));
        }
        SetLoaded(value) => state.typing.loaded = value,
    }
}
