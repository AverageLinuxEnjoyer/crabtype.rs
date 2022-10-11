use super::{
    options_action::OptionsAction, timer_action::TimerAction, words_action::WordsAction,
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
    pub languages: Vec<&'static str>,
    pub selected_language: usize,

    pub capitalization: bool,
    pub punctuation: bool,

    pub timers: Vec<usize>,
    pub selected_timer: usize,

    pub letters_per_row: usize,
    pub rows: usize,
    pub max_written_rows: usize,

    pub words: Rc<RefCell<Vec<WordData>>>,
    pub loaded: bool,

    pub current_word_index: usize,
    pub current_letter_index: usize,

    pub countdown: usize,
    pub started: bool,
}

impl fmt::Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "languages: {:?}\nselected_language:{}\n capitalization:{}\n punctuation:{}\ntimers:{:?}\nselected_timer:{}\ncurrent_word_index:{}\ncurrent_letter_index:{} countdown:{}\nstarted:{}\n", self.languages, self.selected_language, self.capitalization, self.punctuation, self.timers, self.selected_timer, self.current_word_index, self.current_letter_index, self.countdown, self.started  )
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
        let mut state = AppState {
            languages: self.languages.clone(),
            selected_language: self.selected_language,

            capitalization: self.capitalization,
            punctuation: self.punctuation,

            timers: self.timers.clone(),
            selected_timer: self.selected_timer,

            words: self.words.clone(),
            loaded: self.loaded,

            current_word_index: self.current_word_index,
            current_letter_index: self.current_letter_index,

            letters_per_row: self.letters_per_row,
            max_written_rows: self.max_written_rows,
            rows: self.rows,

            countdown: self.countdown,
            started: self.started,
        };

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
            languages: vec!["english_1k", "english_10k", "english_30k"],
            selected_language: 0,

            capitalization: false,
            punctuation: false,

            timers: vec![15, 30, 60, 120],
            selected_timer: 1,

            letters_per_row: 60,
            rows: 3,
            max_written_rows: 2,

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
            started: false,
        }
    }
}
