use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Gadget {
    name: String,
    icon_url: String
}
