use crate::components::{
    letter::{class::LetterClass, view::LetterProps},
    word::{class::WordClass, view::WordProps},
};
use itertools::{EitherOrBoth::*, Itertools};

#[derive(Debug, Clone, PartialEq)]
pub struct WordData {
    pub written: Option<String>,
    pub target: String,
}

impl WordData {
    pub fn to_props(&self) -> WordProps {
        match self.written.as_ref() {
            None => WordProps {
                class: WordClass::Default,
                content: self
                    .target
                    .chars()
                    .map(|letter| LetterProps {
                        letter,
                        class: LetterClass::Default,
                    })
                    .collect::<Vec<LetterProps>>(),
            },
            Some(written) => {
                let content = self
                    .target
                    .chars()
                    .zip_longest(written.chars())
                    .map(|it| match it {
                        Both(t, w) => {
                            if t == w {
                                LetterProps {
                                    letter: t,
                                    class: LetterClass::Correct,
                                }
                            } else {
                                LetterProps {
                                    letter: t,
                                    class: LetterClass::Incorrect,
                                }
                            }
                        }
                        Left(t) => LetterProps {
                            letter: t,
                            class: LetterClass::Default,
                        },
                        Right(w) => LetterProps {
                            letter: w,
                            class: LetterClass::Extra,
                        },
                    })
                    .collect::<Vec<LetterProps>>();

                let class = if content.iter().all(|it| it.class == LetterClass::Correct) {
                    WordClass::Correct
                } else {
                    WordClass::Incorrect
                };

                WordProps { content, class }
            }
        }
    }

    pub fn is_correct(&self) -> bool {
        match self.written.as_ref() {
            None => false,
            Some(written) => *written == self.target,
        }
    }
}
