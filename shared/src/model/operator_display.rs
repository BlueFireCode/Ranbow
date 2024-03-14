use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct OperatorDisplay {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub icon_url: String,
    pub selected: Option<bool>,
    pub attacker: bool,
    pub tdm: bool
}
