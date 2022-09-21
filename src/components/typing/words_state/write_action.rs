use super::state::State;

pub enum WriteAction {
    Space,
    Backspace,
    CtrlBackspace,
    Other(char),
}

pub fn handle_write_action(action: WriteAction, state: &mut State) {
    use WriteAction::*;

    match action {
        Space => {
            state.current_word_index += 1;
            state.current_letter_index = 0;
        }
        Backspace => {
            if state.current_letter_index > 0 {
                state.words[state.current_word_index]
                    .written
                    .as_mut()
                    .unwrap()
                    .pop();

                state.current_letter_index -= 1;
            } else if state.current_word_index > 0
                && !state.words[state.current_word_index - 1].is_correct()
            {
                state.words[state.current_word_index].written = None;
                state.current_word_index -= 1;
                let current = &state.words[state.current_word_index];

                state.current_letter_index = match current.written.as_ref() {
                    None => 0,
                    Some(written) => written.len(),
                }
            }
        }
        CtrlBackspace => {
            if state.current_letter_index > 0 {
                state.current_letter_index = 0;
                state.words[state.current_word_index].written = None;
            }
        }
        Other(key) => {
            let current = &mut state.words[state.current_word_index].written;

            match current {
                None => {
                    let _ = current.insert(key.into());
                }
                Some(written) => {
                    written.push(key);
                }
            };

            state.current_letter_index += 1;
        }
    }
}
