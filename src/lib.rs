mod router;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, style};
use crate::router::{Route, switch};

#[styled_component(App)]
pub fn app() -> Html {
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
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </div>
    }
}
