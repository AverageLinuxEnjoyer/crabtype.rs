use components::typing::view::TypingContainer;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

pub mod components;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <TypingContainer />
    }
}
