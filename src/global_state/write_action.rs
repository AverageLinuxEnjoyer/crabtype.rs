use gloo::console::log;

use super::state::AppState;

pub enum WriteAction {
    Space,
    Backspace,
    CtrlBackspace,
    Other(char),
}

pub fn handle_write_action(action: WriteAction, state: &mut AppState) {
    use WriteAction::*;

    match action {
        Space => {
            state.typing.current_word_index += 1;
            state.typing.current_letter_index = 0;
        }
        Backspace => {
            if state.typing.current_letter_index > 0 {
                state.typing.words.borrow_mut()[state.typing.current_word_index]
                    .written
                    .as_mut()
                    .unwrap()
                    .pop();

                state.typing.current_letter_index -= 1;
            } else if state.typing.current_word_index > 0
                && !state.typing.words.borrow()[state.typing.current_word_index - 1].is_correct()
            {
                state.typing.words.borrow_mut()[state.typing.current_word_index].written = None;
                state.typing.current_word_index -= 1;
                let current = &state.typing.words.borrow_mut()[state.typing.current_word_index];

                state.typing.current_letter_index = match current.written.as_ref() {
                    None => 0,
                    Some(written) => written.len(),
                }
            }
        }
        CtrlBackspace => {
            if state.typing.current_letter_index > 0 {
                state.typing.current_letter_index = 0;
                state.typing.words.borrow_mut()[state.typing.current_word_index].written = None;
            }
        }
        Other(key) => {
            let current =
                &mut state.typing.words.borrow_mut()[state.typing.current_word_index].written;

            match current {
                None => {
                    let _ = current.insert(key.into());
                }
                Some(written) => {
                    written.push(key);
                }
            };

            state.typing.current_letter_index += 1;
        }
    }
}
