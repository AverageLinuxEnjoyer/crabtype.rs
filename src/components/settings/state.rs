use super::language::Language;

#[derive(Default)]
pub struct State {
    pub language: Language,
    pub capitalization: bool,
    pub punctuation: bool,
    pub timers: u32,
}
