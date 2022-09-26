use crate::global_state::state::AppState;
use crate::global_state::timer_action::TimerAction;
use crate::global_state::{state::StateAction, write_action::WriteAction};
use gloo::{console::log, events::EventListener};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

pub fn use_key_listener_effect(state: UseReducerHandle<AppState>) {
    use_effect(move || {
        let document = gloo::utils::document();

        let listener = EventListener::new(&document, "keydown", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();

            if event.key() == "Backspace" {
                state.dispatch(StateAction::WriteAction(WriteAction::Backspace));
                // TODO
            } else if event.key() == "???" {
                state.dispatch(StateAction::WriteAction(WriteAction::CtrlBackspace));
            } else if event.key() == " " {
                state.dispatch(StateAction::WriteAction(WriteAction::Space));
            } else if event.key().len() == 1 {
                if !state.started {
                    state.dispatch(StateAction::TimerAction(TimerAction::StartTimer));
                }

                // guaranteed not to panic, since we checked the length (see line 22)
                let ch = event.key().chars().next().unwrap();
                state.dispatch(StateAction::WriteAction(WriteAction::Other(ch)));
            }
        });

        || drop(listener)
    });
}
