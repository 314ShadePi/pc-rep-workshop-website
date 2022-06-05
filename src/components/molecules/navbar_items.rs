use yew::prelude::*;
use stylist::{yew::styled_component, style};

use crate::router::Route;
use crate::components::atoms::navbar_item::NavbarItem;

#[styled_component(NavbarItems)]
pub fn navbar_items() -> Html {
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
        <div class={stylesheet}>
            <span class="inner">
                <NavbarItem route={Route::Home} label={"Home"}/>
                <NavbarItem route={Route::About} label={"About"}/>
            </span>
        </div>
    }
}