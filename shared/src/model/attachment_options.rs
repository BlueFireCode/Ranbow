use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AttachmentOptions {
    sight: bool,
    muzzle: bool,
    grip: bool,
    laser: bool,
}