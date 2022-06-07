use yew::prelude::*;

use super::super::molecules::navbar::Navbar;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <Navbar/>
            <hr style="width:100%" size="1" color="white" />
        </header>
    }
}