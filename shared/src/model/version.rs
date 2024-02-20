use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub version_number: String
}