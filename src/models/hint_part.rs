use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum HintPart {
    #[serde(rename = "text")]
    Text(TextHint),
    #[serde(rename = "gps")]
    GPS(GPSHint),
    #[serde(rename = "venue")]
    Venue,
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "animation")]
    Animation,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "video_note")]
    VideoNote,
    #[serde(rename = "contact")]
    Contact,
    #[serde(rename = "sticker")]
    Sticker,
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
