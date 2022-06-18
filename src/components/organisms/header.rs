use yew::prelude::*;

use super::super::molecules::navbar::Navbar;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <Navbar/>
            <hr id="header-line" />
        </header>
    }
}