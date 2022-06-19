use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: Route,
    pub label: String,
    pub class_name: String,
}

#[function_component(NavbarItem)]
pub fn navbar_item(props: &Props) -> Html {
    html! {
        <Link<Route> to={props.route.clone()} classes={string_to_static_str(props.class_name.clone())}>{props.label.clone()}</Link<Route>>
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}