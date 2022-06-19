use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::*;

use crate::{router::Route, stores::{local::LocalStore, session::SessionStore}};

#[function_component(Home)]
pub fn home() -> Html {
    let local_store = use_store::<PersistentStore<LocalStore>>();
    let increment_local = local_store.dispatch().reduce_callback(|s| s.count += 1);
    let decrement_local = local_store.dispatch().reduce_callback(|s| s.count -= 1);
    let session_store = use_store::<PersistentStore<SessionStore>>();
    // TODO: Implement callback to set active page in session store
    // FIXME: When running the following two lines, the website in unable to load. Find another way to set active page.
    // SessionStore::page does no longer exist.
    /*let on_load_set_active_page = session_store.dispatch().reduce_callback_with(|s, v: Route| s.page = v);
    on_load_set_active_page.emit(Route::Home);*/
    let increment_session = session_store.dispatch().reduce_callback(|s| s.count += 1);
    let decrement_session = session_store.dispatch().reduce_callback(|s| s.count -= 1);
    html! {
        <div class="page-home">
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