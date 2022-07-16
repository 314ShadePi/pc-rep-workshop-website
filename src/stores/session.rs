use serde::{Serialize, Deserialize};
use yewdux::prelude::*;
use crate::components::pages::games::game_list::GameList;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SessionStore {
    pub count: u32,
    pub game_list: GameList,
}

impl Persistent for SessionStore {
    fn area() -> Area {
        Area::Session
    }

    fn key() -> &'static str {
        "pc-rep-workshop-website-af62a145-6393-4ec8-b88d-dc752d6a952c"
    }
}
