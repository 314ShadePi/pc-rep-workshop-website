use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, style};

use crate::router::Route;

#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = style!(
        r#"
        display: grid;
        place-items: center;
        
        h1 {
            color: aqua;
        }
        
        p {
            color: green;
        }
        
        li {
            color: red;
        }
        "#
    ).unwrap();
    html! {
        <div class={stylesheet}>
            <h1>{"Home"}</h1>
            <p>{"Welcome to the home page"}</p>
            <Link<Route> to={Route::About}>{"About"}</Link<Route>>
        </div>
    }
}