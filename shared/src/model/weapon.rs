use serde::{Serialize, Deserialize};

use crate::model::attachment_options::AttachmentOptions;
use crate::model::attachments::Attachments;

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    name: String,
    icon_url: String,
    attachment_options: AttachmentOptions,
    attachments: Attachments,
}