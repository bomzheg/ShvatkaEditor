use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Key {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key={}", &self.value)
    }
}
