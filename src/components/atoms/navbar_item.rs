use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: Route,
    pub label: String,
}

#[function_component(NavbarItem)]
pub fn navbar_item(props: &Props) -> Html {
    html! {
        <Link<Route> to={props.route.clone()}>{props.label.clone()}</Link<Route>>
    }
}