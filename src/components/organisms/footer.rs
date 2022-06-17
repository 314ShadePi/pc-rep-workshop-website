use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <hr id="line" size="1" color="white" />
            <div class="footer-text-container">
                <span class="inner">
                    <p>{"© 2022 - 314ShadePi - PC Rep Workshop"}</p>
                </span>
            </div>
        </footer>
    }
}