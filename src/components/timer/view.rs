use crate::global_state::{
    state::{AppContext, StateAction},
    timer_action::TimerAction,
};
use gloo::timers::callback::Interval;
use stylist::Style;
use yew::prelude::*;

#[function_component(Timer)]
pub fn timer() -> Html {
    let style = Style::new(include_str!("style.css")).unwrap();

    let state = use_context::<AppContext>().expect("No app context found");

    let timer = use_state(|| Option::<Interval>::None);

    let state_clone = state.clone();

    match *timer {
        None if state_clone.started => timer.set(Some(Interval::new(1000, move || {
            state_clone.dispatch(StateAction::TimerAction(TimerAction::DecrementTimer))
        }))),
        Some(_) if !state_clone.started => timer.set(None),
        _ => (),
    }

    html! {
        <h1 class={style}> {state.countdown} </h1>
    }
}
