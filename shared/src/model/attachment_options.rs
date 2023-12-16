use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AttachmentOptions {
    pub sight: bool,
    pub muzzle: bool,
    pub grip: bool,
    pub laser: bool,
}