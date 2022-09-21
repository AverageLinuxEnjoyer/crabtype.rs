use super::class::WordClass;
use crate::components::letter::{class::LetterClass, view::Letter, view::LetterProps};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct WordProps {
    pub content: Vec<LetterProps>,
    pub class: WordClass,
}

#[styled_component(Word)]
pub fn word(props: &WordProps) -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let mut props = props.clone();

    props.content.push(LetterProps {
        letter: '\u{00a0}',
        class: match props.class {
            WordClass::LastLetter => LetterClass::Current,
            _ => LetterClass::Default,
        },
    });

    let html_letters = props
        .content
        .iter()
        .map(|letter_prop| {
            html! {
                <Letter letter={letter_prop.letter} class={letter_prop.class} />
            }
        })
        .collect::<Html>();

    html! {
        <word class={classes!(props.class.to_string(), style)}>
            {html_letters}
        </word>
    }
}
