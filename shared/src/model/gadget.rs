use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Gadget {
    pub name: String,
    pub icon_url: String
}
