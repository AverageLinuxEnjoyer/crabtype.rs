use components::options::view::Options;
use components::timer::view::Timer;
use components::typing::view::TypingContainer;
use global_state::state::{AppContext, AppState};
use stylist::{yew::styled_component, Style};
use yew::{prelude::*, ContextProvider};

pub mod components;
pub mod fetch;
pub mod global_state;

#[styled_component(App)]
pub fn app() -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();
    let ctx = use_reducer(AppState::default);

    html! {
        <ContextProvider<AppContext> context={ctx}>
            <div class={style}>
            <Options />
            <TypingContainer />
            </div>
        </ ContextProvider<AppContext>>
    }
}
