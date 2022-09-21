use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum WordClass {
    Default,
    Current,
    Correct,
    Incorrect,
    LastLetter,
}

impl Display for WordClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WordClass::Default => write!(f, ""),
            WordClass::Current => write!(f, "current"),
            WordClass::Correct => write!(f, "correct"),
            WordClass::Incorrect => write!(f, "incorrect"),
            WordClass::LastLetter => write!(f, "last-letter"),
        }
    }
}
