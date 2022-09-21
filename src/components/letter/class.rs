use std::fmt::Display;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LetterClass {
    Default,
    Current,
    Correct,
    Incorrect,
    Extra,
    ExtraCurrent,
}

impl Display for LetterClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LetterClass::Default => write!(f, ""),
            LetterClass::Current => write!(f, "current"),
            LetterClass::Correct => write!(f, "correct"),
            LetterClass::Incorrect => write!(f, "incorrect"),
            LetterClass::Extra => write!(f, "extra"),
            LetterClass::ExtraCurrent => write!(f, "extra current"),
        }
    }
}
