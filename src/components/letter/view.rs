use super::class::LetterClass;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct LetterProps {
    pub letter: char,
    pub class: LetterClass,
}

#[styled_component(Letter)]
pub fn letter(props: &LetterProps) -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    html! {
        <letter class={classes!(props.class.to_string(), style)}>
            {props.letter}
        </letter>
    }
}
