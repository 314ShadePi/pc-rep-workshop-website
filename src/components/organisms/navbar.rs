use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{yew::styled_component, style};

use crate::router::Route;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let stylesheet = style!(
        r#"
        display: grid;
        place-items: center;

        .inner {
            display: flex;
            flex-direction: row;
            gap: 1rem;
        }
        "#
    ).unwrap();
    html! {
        <header>
            <nav>
                <div class={stylesheet}>
                    <span class="inner">
                        <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                        <Link<Route> to={Route::About}>{"About"}</Link<Route>>
                    </span>
                </div>
            </nav>
        </header>
    }
}