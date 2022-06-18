use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <hr id="footer-line" />
            <div class="footer-text-container">
                <span class="inner">
                    <p>{"© 2022 - 314ShadePi"}</p>
                </span>
            </div>
        </footer>
    }
}