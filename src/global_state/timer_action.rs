use super::state::AppState;
use gloo::timers::callback::Interval;

pub enum TimerAction {
    DecrementTimer,
    StartTimer,
}

pub fn handle_timer_action(action: TimerAction, state: &mut AppState) {
    use TimerAction::*;

    match action {
        DecrementTimer => {
            if state.countdown > 0 {
                state.countdown -= 1;
            }

            if state.countdown == 0 {
                state.started = false;
            }
        }
        StartTimer => state.started = true,
    }
}
