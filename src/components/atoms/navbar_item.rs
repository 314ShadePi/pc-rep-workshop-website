use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, style};

use crate::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: Route,
    pub label: String,
}

#[styled_component(NavbarItem)]
pub fn navbar_item(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        display: grid;
        place-items: center;
        "#
    ).unwrap();
    html! {
        <span class={stylesheet}>
            <Link<Route> to={props.route.clone()}>{props.label.clone()}</Link<Route>>
        </span>
    }
}