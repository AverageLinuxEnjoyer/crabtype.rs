use crate::components::typing::words_state::words_action::handle_words_action;

use super::{
    word_data::WordData,
    words_action::WordsAction,
    write_action::{handle_write_action, WriteAction},
};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Default, Debug, PartialEq)]
pub struct State {
    // at some point this vec will be wrapped inside
    // a Rc<RefCelL>, and the number of copies made
    // will be reduced to 0 (compared to now, where
    // a copy is made for every state change, eg: a space)
    // However, it'll be left as it is as we tweak
    // the logic, so it's easier to work with it
    pub words: Vec<WordData>,
    pub current_word_index: usize,
    pub current_letter_index: usize,

    pub letters_per_row: usize,
    pub rows: usize,
    pub max_written_rows: usize,
}

pub enum StateAction {
    WriteAction(WriteAction),
    WordsAction(WordsAction),
}

impl Reducible for State {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = State {
            words: self.words.clone(),
            current_word_index: self.current_word_index,
            current_letter_index: self.current_letter_index,

            letters_per_row: self.letters_per_row,
            max_written_rows: self.max_written_rows,
            rows: self.rows,
        };

        use StateAction::*;
        match action {
            WriteAction(action) => handle_write_action(action, &mut state),
            WordsAction(action) => handle_words_action(action, &mut state),
        }

        state.into()
    }
}

impl State {
    pub fn new_reducer() -> UseReducerHandle<State> {
        use_reducer(|| State {
            words: String::from(include_str!("words.txt"))
                .split(' ')
                .map(|st| WordData {
                    target: st.to_owned(),
                    written: None,
                })
                .collect::<Vec<WordData>>()[0..60]
                .to_vec(),
            current_word_index: 0,
            current_letter_index: 0,

            letters_per_row: 60,
            rows: 3,
            max_written_rows: 2,
        })
    }
}
