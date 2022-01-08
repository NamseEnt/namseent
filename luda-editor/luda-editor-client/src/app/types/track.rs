use serde::{Deserialize, Serialize};

use super::{CameraClip, SubtitleClip};

#[derive(Serialize, Deserialize, Clone)]
pub enum Track {
    Camera(CameraTrack),
    Subtitle(SubtitleTrack),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTrack {
    pub clips: Vec<CameraClip>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub clips: Vec<SubtitleClip>,
}
