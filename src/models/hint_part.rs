use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum HintPart {
    #[serde(rename = "text")]
    Text(hint_impl::TextHint),
    #[serde(rename = "gps")]
    GPS(hint_impl::GPSHint),
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

pub(crate) mod hint_impl {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub(crate) struct TextHint {
        __hint__: bool,
        text: String,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct GPSHint {
        __hint__: bool,
        latitude: f64,
        longitude: f64,
    }
}
