use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Attachments {
    pub sight: u8,
    pub selected_sight: Option<u8>,
    pub muzzle: Vec<String>,
    pub selected_muzzle: Option<String>,
    pub grip: Vec<String>,
    pub selected_grip: Option<String>,
    pub laser_selected: Option<bool>
}
