mod camera_angle;
pub mod google_spreadsheet;
pub mod meta;
mod page;
mod router_context;
mod sequence;
mod subtitle_play_duration_measure;
mod track;

pub use camera_angle::*;
pub use clip::*;
pub use google_spreadsheet::Sheet;
pub use meta::*;
use namui::prelude::*;
pub use page::*;
pub use router_context::*;
pub use sequence::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
pub use subtitle_play_duration_measure::*;
pub use track::*;

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
