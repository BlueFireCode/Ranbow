use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct AttachmentOptions {
    pub sight: bool,
    pub muzzle: bool,
    pub grip: bool,
    pub laser: bool,
}