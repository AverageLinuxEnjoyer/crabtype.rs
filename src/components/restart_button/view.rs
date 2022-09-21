use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<()>,
}

#[styled_component(RestartButton)]
pub fn restart_button(props: &Props) -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });

    html! {
        <button type={"button"} onclick={button_onclick} class={style}>
            //<i class={style}></i>
        </button>
    }
}
