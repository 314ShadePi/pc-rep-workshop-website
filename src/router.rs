use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Serialize, Deserialize};
use crate::components::pages::*;

#[derive(Clone, Routable, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/games")]
    Games,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <home::Home />
        },
        Route::About => html! {
            <about::About />
        },
        Route::Games => html! {
            <games::page::Games />
        },
        Route::NotFound => html! {
            <div class="page-404">
                <h1>{"404"}</h1>
                <p>{"Page not found"}</p>
            </div>
        },
    }
}