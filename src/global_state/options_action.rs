use super::state::AppState;
use crate::global_state::typing_state::TypingStatus;

pub enum OptionsAction {
    SetLanguage(usize),
    ToggleCapitalization,
    TogglePunctuation,
    SetTimer(usize),
}

pub fn handle_options_action(action: OptionsAction, state: &mut AppState) {
    use OptionsAction::*;

    match action {
        SetLanguage(i) => state.options.selected_language = i,
        ToggleCapitalization => state.options.capitalization = !state.options.capitalization,
        TogglePunctuation => state.options.punctuation = !state.options.punctuation,
        SetTimer(i) => state.options.selected_timer = i,
    }

    state.typing.status = TypingStatus::NotStarted;
    state.typing.countdown = state.options.timers[state.options.selected_timer];
}
