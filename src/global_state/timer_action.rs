use super::state::AppState;
use crate::global_state::typing_state::TypingStatus;
use gloo::{console::log, timers::callback::Interval};

pub enum TimerAction {
    DecrementTimer,
    StartTimer,
}

pub fn handle_timer_action(action: TimerAction, state: &mut AppState) {
    use TimerAction::*;

    match action {
        DecrementTimer => {
            if state.typing.countdown > 0 {
                state.typing.countdown -= 1;
            }

            if state.typing.countdown == 0 {
                state.typing.status = TypingStatus::Finished;
            }
        }
        StartTimer => state.typing.status = TypingStatus::Started,
    }
}
