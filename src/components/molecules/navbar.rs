use yew::prelude::*;

use super::navbar_items::NavbarItems;
use super::super::atoms::navbar_logo::NavbarLogo;
use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class="container">
            <div class="row align-items-center justify-content-between">
                <div class="logo">
                    <NavbarLogo route={Route::Home} label={"PC Rep Workshop"}/>
                </div>
                <input type="checkbox" id="nav-check" />
                <label for="nav-check" class="nav-toggler">
                    <span></span>
                </label>
                <nav class="nav">
                    <NavbarItems/>
                </nav>
            </div>
        </div>
    }
}