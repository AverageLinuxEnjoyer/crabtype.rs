use crate::components::typing::word_data::WordData;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypingStatus {
    NotStarted,
    Started,
    Finished,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypingState {
    pub words: Rc<RefCell<Vec<WordData>>>,
    pub loaded: bool,

    pub current_word_index: usize,
    pub current_letter_index: usize,

    pub countdown: usize,
    pub status: TypingStatus,
}
