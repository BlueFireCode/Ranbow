use serde::{Serialize, Deserialize};

use crate::model::attachment_options::AttachmentOptions;
use crate::model::attachments::Attachments;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Weapon {
    pub name: String,
    pub icon_url: String,
    pub attachment_options: AttachmentOptions,
    pub attachments: Attachments,
}