use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, style};

use crate::router::Route;

#[styled_component(About)]
pub fn about() -> Html {
    let stylesheet = style!(
        r#"
        display: grid;
        place-items: center;
        "#
    ).unwrap();
    let history = use_history().unwrap();
    let onclick = {
        let history = history.clone();
        Callback::once(move |_| history.push(Route::Home))
    };
    let notfound = {
        let history = history.clone();
        Callback::once(move |_| history.push(Route::NotFound))
    };

    html! {
        <div class={stylesheet}>
            <h1>{"About"}</h1>
            <p>{"Welcome to the about page"}</p>
            <button {onclick}>{"Home"}</button>
            <br />
            <button onclick={notfound}>{"404"}</button>
        </div>
    }
}