use crate::{
    components::{
        word::view::WordProps,
        words_container::{row::Row, rows::Rows},
    },
    global_state::state::AppState,
};
use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub state: UseReducerHandle<AppState>,
}

#[styled_component(WordsContainer)]
pub fn words_container(props: &ContainerProps) -> Html {
    // from which word onwards words are displayed
    // modifies when rows are shifted up
    let start_word_index = use_state(|| 0usize);

    // when words reset, state.current_word_index becomes 0 so we reset the start_word_index as well
    if *start_word_index != 0 && props.state.current_word_index == 0 {
        start_word_index.set(0);
    }

    let mut rows = Vec::<Vec<WordProps>>::from_state(&props.state, *start_word_index);

    rows.mark_current_classes(&props.state, *start_word_index);

    if rows
        .get(props.state.max_written_rows)
        .unwrap()
        .first_letter_current()
    {
        let first_row_word_count = rows.get(0).unwrap().word_count();

        start_word_index.set(*start_word_index + first_row_word_count);

        rows = Vec::<Vec<WordProps>>::from_state(&props.state, *start_word_index);
    }

    let html_rows = rows.to_html();

    let container_width = Style::new(format!("width: {}ch;", props.state.letters_per_row)).unwrap();

    let style = Style::new(include_str!("style.css")).unwrap();

    html! {
        <words_container class={classes!(style, container_width)}>
            {html_rows}
        </words_container>
    }
}
