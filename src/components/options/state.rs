#[derive(Clone)]
pub struct State {
    pub languages: Vec<&'static str>,
    pub selected_language: usize,

    pub capitalization: bool,
    pub punctuation: bool,

    pub timers: Vec<u32>,
    pub selected_timer: usize,
}

impl Default for State {
    fn default() -> Self {
        State {
            languages: vec!["english_1k", "english_10k", "english_30k"],
            selected_language: 0,
            capitalization: false,
            punctuation: false,
            timers: vec![15, 30, 60, 120],
            selected_timer: 1,
        }
    }
}