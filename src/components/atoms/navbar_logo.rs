use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: Route,
    pub label: String,
}

#[function_component(NavbarLogo)]
pub fn navbar_logo(props: &Props) -> Html {
    html! {
        <span class="navbar-logo">
            <Link<Route> to={props.route.clone()}>{props.label.clone()}</Link<Route>>
        </span>
    }
}
