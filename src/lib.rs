mod stores;
mod router;
mod components;
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, style};
use components::organisms::{header::Header, footer::Footer};

use crate::router::{Route, switch};

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = style!(
        r#"
        display: grid;
        place-items: center;
        "#
    ).unwrap();

    html! {
        <BrowserRouter>
            <Header />
            <div class={stylesheet}>
                <Switch<Route> render={Switch::render(switch)} />
            </div>
            <Footer />
        </BrowserRouter>
    }
}
