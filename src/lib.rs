use components::typing::view::TypingContainer;
use components::options::view::Options;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

pub mod components;

#[styled_component(App)]
pub fn app() -> Html {

    let style = Style::new("
        display: flex;
        justify-content: flex-start;
        flex-wrap: wrap;
        flex-direction: column;")
    .unwrap();

    html! {
        <div class={style}>
        <Options />
        <TypingContainer />
        </div>
    }
}
