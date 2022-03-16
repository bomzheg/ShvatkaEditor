use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum HintPart {
    Text(TextHint),
    GPS(GPSHint),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TextHint {
    text: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GPSHint {
    latitude: f64,
    longitude: f64,
}
