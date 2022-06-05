use yew::prelude::*;
use stylist::{yew::styled_component, style};

use crate::router::Route;
use crate::components::{molecules::navbar_items::NavbarItems, atoms::navbar_logo::NavbarLogo};

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let stylesheet = style!(
        r#"
        display: flex;
        justify-content: flex-start;
        gap: 5rem;
        margin-top: 1.25rem;
        margin-left: 2rem;

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
                    <NavbarLogo route={Route::Home} label={"PC Rep Workshop"}/>
                    <span class="inner">
                        <NavbarItems/>
                    </span>
                </div>
            </nav>
            <hr style="width:100%" size="1" color="white" />
        </header>
    }
}