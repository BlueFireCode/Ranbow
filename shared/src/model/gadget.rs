use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Gadget {
    pub name: String,
    pub icon_url: String
}
