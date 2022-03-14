use serde::{Deserialize, Serialize};
use super::hint_part::HintPart;

#[derive(Serialize, Deserialize)]
pub(crate) struct Hint {
    time: i32,
    parts: HintPart,
}
