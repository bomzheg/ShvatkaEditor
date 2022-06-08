use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use super::{hint::Hint, key::Key};

#[derive(Serialize, Deserialize)]
pub(crate) struct Level {
    __level__: bool,
    id: String,
    #[serde(rename = "author_ID")]
    author: i64,
    keys: Vec<Key>,
    time_hints: Vec<Hint>,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, first key: {}", &self.id, &self.keys[0])
    }
}
