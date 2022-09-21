use crate::components::typing::words_state::word_data::WordData;

use super::state::State;

pub enum WordsAction {
    ResetWords(Vec<String>),
    AddWords(Vec<String>),
}

pub fn handle_words_action(action: WordsAction, state: &mut State) {
    use WordsAction::*;

    match action {
        ResetWords(words) => {
            state.current_letter_index = 0;
            state.current_word_index = 0;
            state.words = words
                .into_iter()
                .map(|w| WordData {
                    written: None,
                    target: w,
                })
                .collect();
        }
        AddWords(words) => {
            state.words.extend(words.into_iter().map(|w| WordData {
                written: None,
                target: w,
            }));
        }
    }
}
