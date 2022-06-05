use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, style};

use crate::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: Route,
    pub label: String,
}

#[styled_component(NavbarLogo)]
pub fn navbar_logo(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        display: grid;
        place-items: center;
        font-size: 1.5rem;
        "#
    ).unwrap();
    html! {
        <span class={stylesheet}>
            <Link<Route> to={props.route.clone()}>{props.label.clone()}</Link<Route>>
        </span>
    }
}