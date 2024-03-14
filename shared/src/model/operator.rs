use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::model::weapon::Weapon;
use crate::model::gadget::Gadget;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Operator {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub icon_url: String,
    pub image_url: String,
    pub attacker: bool,
    pub primaries: Vec<Weapon>,
    pub selected_primary: Option<Weapon>,
    pub secondaries: Vec<Weapon>,
    pub selected_secondary: Option<Weapon>,
    pub gadgets: Vec<Gadget>,
    pub selected_gadget: Option<Gadget>,
    pub tdm: bool
}
