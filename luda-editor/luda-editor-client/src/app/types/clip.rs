use super::{CameraAngle, Time};
use namui::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraClip {
    pub id: String,
    pub start_at: Time,
    pub end_at: Time,
    pub camera_angle: CameraAngle,
}

pub enum Clip<'a> {
    Camera(&'a CameraClip),
    Subtitle(&'a SubtitleClip),
}
pub enum MutableClip<'a> {
    Camera(&'a mut CameraClip),
    Subtitle(&'a mut SubtitleClip),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleClip {
    pub id: String,
    pub start_at: Time,
    pub subtitle: Subtitle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub id: String,
    pub language_text_map: HashMap<Language, String>,
}
