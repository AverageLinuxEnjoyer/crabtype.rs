use crate::{
    components::{
        letter::class::LetterClass,
        word::{
            class::WordClass,
            view::{Word, WordProps},
        },
    },
    global_state::state::AppState,
};
use stylist::css;
use yew::prelude::*;

pub trait Rows {
    fn from_state(state: &AppState, start_word_index: usize) -> Vec<Vec<WordProps>>;
    fn mark_current_classes(&mut self, state: &AppState, start_word_index: usize);
    fn to_html(self) -> Html;
}

impl Rows for Vec<Vec<WordProps>> {
    fn from_state(state: &AppState, start_word_index: usize) -> Vec<Vec<WordProps>> {
        let mut letter_count = 0;
        let mut rows: Vec<Vec<WordProps>> = vec![Vec::new()];
        for word in state.words.iter().skip(start_word_index) {
            let word = word.to_props();

            if word.content.len() + letter_count >= state.letters_per_row {
                if rows.len() == state.rows {
                    break;
                } else {
                    rows.push(Vec::new());
                    letter_count = 0;
                }
            }

            letter_count += word.content.len() + 1;

            // guaranteed not to panic, rows has at least 1 elem inside (see line 23) 
            rows.last_mut().unwrap().push(word);
        }

        rows
    }

    fn mark_current_classes(&mut self, state: &AppState, start_word_index: usize) {
        let mut word_index = start_word_index;
        'outer: for row in self.iter_mut() {
            for word in row.iter_mut() {
                if word_index == state.current_word_index {
                    word.class = WordClass::Current;

                    for (letter_index, letter) in word.content.iter_mut().enumerate() {
                        if letter_index == state.current_letter_index {
                            letter.class = LetterClass::Current;
                        }
                    }

                    if state.current_letter_index == word.content.len() {
                        word.class = WordClass::LastLetter;
                    }

                    break 'outer;
                }

                if word
                    .content
                    .iter()
                    .all(|letter| letter.class == LetterClass::Default)
                {
                    word.class = WordClass::Incorrect;
                }

                word_index += 1;
            }
        }
    }

    fn to_html(self) -> Html {
        self.into_iter().map(|row| {
            let html_row = row
                .into_iter()
                .map(|word| {
                    html! {
                        <Word content={word.content} class={word.class} />
                    }
                })
                .collect::<Html>();

            html! {
                <row class={css!("display: flex; justify-content: flex-start; width: auto; flex-wrap: wrap;")}>
                    {html_row}
                </row>
            }
        }).collect::<Html>()
    }
}
