use std::ops::Deref;

use crate::stores::session::SessionStore;

use super::{game, game_list};
use reqwasm::http::Request;
use yew::prelude::*;
use gloo::console::log;
use yewdux::prelude::*;
use yewdux_functional::*;

#[derive(Clone)]
pub struct ResponseData {
    pub data: Option<game_list::GameList>,
}

#[function_component(Games)]
pub fn page() -> Html {
    let state = use_state(|| {
        ResponseData {
            data: None
        }
    });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get("http://127.0.0.1:8080/games/")
                    .send()
                    .await
                    .unwrap()
                    .json::<game_list::GameList>()
                    .await
                    .unwrap();
                
                let mut response_data = state.deref().clone();
                response_data.data = Some(res);
                state.set(response_data)
            });
        })
    };
    /*let game_list: game_list::GameList = serde_json::from_str(string_to_static_str(game_list)).unwrap();
    let game: game_list::Game = game_list.games.iter().find(|g| g.id == "one").unwrap().clone();*/
    html! {
        <button onclick={onclick}>{"game.name"}</button>
    }
}