use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::*;

use crate::{router::Route, stores::{local::LocalStore, session::SessionStore}};

#[function_component(About)]
pub fn about() -> Html {
    let local_store = use_store::<PersistentStore<LocalStore>>();
    let local_count = local_store.state().map(|s| s.count).unwrap_or_default();
    let session_store = use_store::<PersistentStore<SessionStore>>();
    let session_count = session_store.state().map(|s| s.count).unwrap_or_default();
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
        <div class="page page-about">
            <h1 class="title">{"About"}</h1>
            <p>{"Welcome to the about page"}</p>
            <button {onclick}>{"Home"}</button>
            <br />
            <button onclick={notfound}>{"404"}</button>
            <br />
            <p>{"Local count: "}{local_count}</p>
            <p>{"Session count: "}{session_count}</p>
        </div>
    }
}