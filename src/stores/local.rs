use serde::{Serialize, Deserialize};
use yewdux::prelude::*;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LocalState {
    pub username: String,
    pub fav_lang: String,
    pub count: u32,
}

impl Persistent for LocalState {
    fn area() -> Area {
        Area::Local
    }

    fn key() -> &'static str {
        "pc-rep-workshop-website-be15bdfc-2f11-4944-b315-2f711c96b509"
    }
}
