use yew::prelude::*;

use crate::router::Route;
use crate::components::atoms::navbar_item::NavbarItem;

#[function_component(NavbarItems)]
pub fn navbar_items() -> Html {
    html! {
        <ul>
            <li><NavbarItem route={Route::Home} label={"Home"} class_name="home-link" /></li>
            <li><NavbarItem route={Route::About} label={"About"} class_name="about-link" /></li>
        </ul>
    }
}