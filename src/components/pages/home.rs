use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, style};
use yewdux::prelude::*;
use yewdux_functional::*;

use crate::{router::Route, stores::{local::LocalStore, session::SessionStore}};

#[styled_component(Home)]
pub fn home() -> Html {
    let local_store = use_store::<PersistentStore<LocalStore>>();
    let increment_local = local_store.dispatch().reduce_callback(|s| s.count += 1);
    let decrement_local = local_store.dispatch().reduce_callback(|s| s.count -= 1);
    let session_store = use_store::<PersistentStore<SessionStore>>();
    let increment_session = session_store.dispatch().reduce_callback(|s| s.count += 1);
    let decrement_session = session_store.dispatch().reduce_callback(|s| s.count -= 1);
    let stylesheet = style!(
        r#"
        display: grid;
        place-items: center;
        "#
    ).unwrap();
    html! {
        <div class={stylesheet}>
            <h1>{"Home"}</h1>
            <p>{"Welcome to the home page"}</p>
            <Link<Route> to={Route::About}>{"About"}</Link<Route>>
            <button onclick={increment_local}>{"Increment Local"}</button>
            <button onclick={decrement_local}>{"Decrement Local"}</button>
            <button onclick={increment_session}>{"Increment Session"}</button>
            <button onclick={decrement_session}>{"Decrement Session"}</button>
        </div>
    }
}