mod page;
mod pixel_size;
mod router_context;
pub use clip::*;
use namui::prelude::*;
pub use page::*;
pub use pixel_size::*;
pub use router_context::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
mod time;
pub use time::*;
mod time_per_pixel;
pub use time_per_pixel::*;
mod subtitle_play_duration_measure;
pub use subtitle_play_duration_measure::*;
mod sequence;
pub use sequence::*;
mod track;
pub use track::*;
pub mod google_spreadsheet;
pub use google_spreadsheet::Sheet;
pub mod meta;
pub use meta::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub id: String,
    #[serde(default)] // TODO: Remove this attribute after sync data.
    pub speaker: String,
    pub language_text_map: HashMap<Language, String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Circumscribed {
    pub center: Xy<f32>,
    pub radius: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterPoseEmotion(pub String, pub String, pub String);
impl CharacterPoseEmotion {
    pub(crate) fn to_url(&self) -> String {
        format!("/{}/{}/{}.png", self.0, self.1, self.2)
    }
}
