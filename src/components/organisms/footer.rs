use yew::prelude::*;
use stylist::{yew::styled_component, style};

#[styled_component(Footer)]
pub fn footer() -> Html {
    let stylesheet = style!(
        r#"
        position: fixed;
        bottom: 0;
        width: 99%;

        .inner {
            display: flex;
            justify-content: center;
            gap: 5rem;
            margin-top: 1.25rem;
            margin-left: 2rem;
            font-family: 'CascadiaCodePL';
        }

        #line {
            width: 100%;
        }
        "#
    ).unwrap();
    html! {
        <footer class={stylesheet}>
            <hr id="line" size="1" color="white" />
            <div>
                <span class="inner">
                    <p>{"© 2022 - 314ShadePi - PC Rep Workshop"}</p>
                </span>
            </div>
        </footer>
    }
}