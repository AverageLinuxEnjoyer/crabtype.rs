use super::state::AppState;

pub enum OptionsAction {
    SetLanguage(usize),
    ToggleCapitalization,
    TogglePunctuation,
    SetTimer(usize),
}

pub fn handle_options_action(action: OptionsAction, state: &mut AppState) {
    use OptionsAction::*;

    match action {
        SetLanguage(i) => state.selected_language = i,
        ToggleCapitalization => state.capitalization = !state.capitalization,
        TogglePunctuation => state.punctuation = !state.punctuation,
        SetTimer(i) => state.selected_timer = i,
    }

    state.started = false;
    state.countdown = state.timers[state.selected_timer];
}
