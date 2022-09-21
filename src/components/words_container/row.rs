use gloo::console::log;

use crate::components::{letter::class::LetterClass, word::view::WordProps};

pub trait Row {
    fn chars_count(&self) -> usize;
    fn letter_count(&self) -> usize;
    fn word_count(&self) -> usize;
    fn is_finished(&self) -> bool;
    fn first_letter_current(&self) -> bool;
}

impl Row for Vec<WordProps> {
    fn chars_count(&self) -> usize {
        let mut count = 0;
        for word in self.iter() {
            count += word.content.len() + 1;
        }
        count
    }

    fn letter_count(&self) -> usize {
        let mut count = 0;
        for word in self.iter() {
            count += word.content.len();
        }
        count
    }

    fn word_count(&self) -> usize {
        self.len()
    }

    fn is_finished(&self) -> bool {
        let last_word = self.last().unwrap();
        let last_letter = last_word.content.last().unwrap();

        last_letter.class != LetterClass::Default
            && last_letter.class != LetterClass::Current
            && last_letter.class != LetterClass::ExtraCurrent
    }

    fn first_letter_current(&self) -> bool {
        let first_word = self.first().unwrap();
        let first_letter = first_word.content.first().unwrap();

        first_letter.class == LetterClass::Current
    }
}
