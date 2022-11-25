use super::{
    options_action::OptionsAction,
    options_state::OptionsState,
    structure_state::StructureState,
    timer_action::TimerAction,
    typing_state::{TypingState, TypingStatus},
    words_action::WordsAction,
    write_action::WriteAction,
};
use crate::{
    components::typing::word_data::WordData,
    global_state::{
        options_action::handle_options_action, timer_action::handle_timer_action,
        words_action::handle_words_action, write_action::handle_write_action,
    },
};
use std::{cell::RefCell, fmt, rc::Rc};
use yew::prelude::*;

pub type AppContext = UseReducerHandle<AppState>;

#[derive(Debug, PartialEq, Clone)]
pub struct AppState {
    pub options: OptionsState,

    pub structure: StructureState,

    pub typing: TypingState,
}

// for debugging purposes
impl fmt::Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "languages: {:?}\nselected_language:{}\n capitalization:{}\n punctuation:{}\ntimers:{:?}\nselected_timer:{}\ncurrent_word_index:{}\ncurrent_letter_index:{} countdown:{}\ntyping_status:{:?}\n", self.options.languages, self.options.selected_language, self.options.capitalization, self.options.punctuation, self.options.timers, self.options.selected_timer, self.typing.current_word_index, self.typing.current_letter_index, self.typing.countdown, self.typing.status  )
    }
}

pub enum StateAction {
    WriteAction(WriteAction),
    WordsAction(WordsAction),
    OptionsAction(OptionsAction),
    TimerAction(TimerAction),
}

impl Reducible for AppState {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = self.as_ref().clone();

        use StateAction::*;
        match action {
            WriteAction(action) => handle_write_action(action, &mut state),
            WordsAction(action) => handle_words_action(action, &mut state),
            OptionsAction(action) => handle_options_action(action, &mut state),
            TimerAction(action) => handle_timer_action(action, &mut state),
        }

        state.into()
    }
}

impl AppState {
    pub fn new_reducer() -> UseReducerHandle<AppState> {
        use_reducer(AppState::default)
    }
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            options: OptionsState {
                languages: vec![
                    "english_1k".into(),
                    "english_10k".into(),
                    "english_30k".into(),
                ],
                selected_language: 0,

                capitalization: false,
                punctuation: false,

                timers: vec![15, 30, 60, 120],
                selected_timer: 1,
            },

            structure: StructureState {
                letters_per_row: 60,
                rows: 3,
                max_written_rows: 2,
            },

            typing: TypingState {
                words: RefCell::new(
                    String::from(include_str!("words.txt"))
                        .split(' ')
                        .map(|st| WordData {
                            target: st.to_owned(),
                            written: None,
                        })
                        .collect::<Vec<WordData>>()[0..60]
                        .to_vec(),
                )
                .into(),
                loaded: true,

                current_word_index: 0,
                current_letter_index: 0,

                countdown: 30,
                status: TypingStatus::NotStarted,
            },
        }
    }
}
