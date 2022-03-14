use serde::{Deserialize, Serialize};

use super::{hint::Hint, key::Key};

#[derive(Serialize, Deserialize)]
pub(crate) struct Level {
    id: String,
    author: i64,
    keys: Vec<Key>,
    hints: Vec<Hint>,
}
