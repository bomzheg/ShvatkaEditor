use serde::{Deserialize, Serialize};

use crate::Level;

#[derive(Serialize, Deserialize)]
pub(crate) struct Game {
    pub(crate) id: String,
    #[serde(rename = "author_ID")]
    pub(crate) author: i64,
    pub(crate) name: String,
    pub(crate) levels: Vec<Level>,
}
