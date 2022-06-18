mod stores;
mod router;
mod components;
use yew::prelude::*;
use yew_router::prelude::*;
use components::{organisms::header::Header, atoms::footer::Footer};

use crate::router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <div class="main-container">
                <Switch<Route> render={Switch::render(switch)} />
            </div>
            <Footer />
        </BrowserRouter>
    }
}
