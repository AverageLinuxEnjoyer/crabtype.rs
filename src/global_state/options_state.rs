#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OptionsState {
    pub languages: Vec<String>,
    pub selected_language: usize,

    pub capitalization: bool,
    pub punctuation: bool,

    pub timers: Vec<usize>,
    pub selected_timer: usize,
}
