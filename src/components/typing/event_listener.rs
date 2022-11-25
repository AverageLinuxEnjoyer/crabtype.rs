use crate::global_state::state::AppState;
use crate::global_state::timer_action::TimerAction;
use crate::global_state::typing_state::TypingStatus;
use crate::global_state::{state::StateAction, write_action::WriteAction};
use gloo::console::log;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

pub fn use_key_listener_effect(state: UseReducerHandle<AppState>) {
    use_effect(move || {
        let document = gloo::utils::document();

        let listener = EventListener::new(&document, "keydown", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            log!(format!("{:?}", state.typing.status));

            if state.typing.countdown != 0 {
                if event.key() == "Backspace" {
                    state.dispatch(StateAction::WriteAction(WriteAction::Backspace));
                } else if event.key() == "???" {
                    // TODO
                    state.dispatch(StateAction::WriteAction(WriteAction::CtrlBackspace));
                } else if event.key() == " " {
                    state.dispatch(StateAction::WriteAction(WriteAction::Space));
                } else if event.key().len() == 1 {
                    if state.typing.status == TypingStatus::NotStarted {
                        state.dispatch(StateAction::TimerAction(TimerAction::StartTimer));
                    }

                    // guaranteed not to panic, since we checked the length in the else if
                    let ch = event.key().chars().next().unwrap();
                    state.dispatch(StateAction::WriteAction(WriteAction::Other(ch)));
                }
            }
        });

        || drop(listener)
    });
}
