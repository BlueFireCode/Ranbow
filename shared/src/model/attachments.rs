use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Attachments {
    sight: i64,
    muzzle: Vec<String>,
    grip: Vec<String>,
}
