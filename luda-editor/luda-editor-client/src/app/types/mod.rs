mod camera_angle;
mod page;
mod pixel_size;
mod router_context;
pub use camera_angle::*;
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
mod camera_track;
pub use camera_track::*;
pub mod google_spreadsheet;
pub use google_spreadsheet::Sheet;
pub mod meta;
pub use meta::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub id: String,
    pub language_text_map: HashMap<Language, String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Circumscribed {
    pub center: Xy<f32>,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub struct ImageFilenameObject {
    pub character: String,
    pub pose: String,
    pub emotion: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterPoseEmotion(pub String, pub String, pub String);

impl CharacterPoseEmotion {
    pub fn get_url(&self, image_filename_objects: &Vec<ImageFilenameObject>) -> Option<String> {
        for image_filename_object in image_filename_objects {
            if image_filename_object.character == self.0
                && image_filename_object.pose == self.1
                && image_filename_object.emotion == self.2
            {
                return Some(image_filename_object.url.clone());
            }
        }
        None
    }
}
