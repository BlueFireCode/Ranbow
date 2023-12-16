use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Attachments {
    pub sight: i64,
    pub muzzle: Vec<String>,
    pub grip: Vec<String>,
}
