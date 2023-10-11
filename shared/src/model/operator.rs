use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::model::weapon::Weapon;
use crate::model::gadget::Gadget;

#[derive(Serialize, Deserialize)]
pub struct Operator {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    icon_url: String,
    image_url: String,
    attacker: bool,
    primaries: Vec<Weapon>,
    secondaries: Vec<Weapon>,
    gadgets: Vec<Gadget>,
}
