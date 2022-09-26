use components::options::view::Options;
use components::timer::view::Timer;
use components::typing::view::TypingContainer;
use global_state::state::{AppContext, AppState};
use stylist::{yew::styled_component, Style};
use yew::{prelude::*, ContextProvider};

pub mod components;
pub mod global_state;

#[styled_component(App)]
pub fn app() -> Html {
    let ctx = use_reducer(AppState::default);

    let style = Style::new(
        "
        display: flex;
        justify-content: flex-start;
        flex-wrap: wrap;
        flex-direction: column;",
    )
    .unwrap();

    html! {
        <ContextProvider<AppContext> context={ctx}>
            <div class={style}>
            <Timer />
            <Options />
            <TypingContainer />
            </div>
        </ ContextProvider<AppContext>>
    }
}
