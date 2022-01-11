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
use std::collections::HashMap;
mod time;
pub use time::*;
mod time_per_pixel;
pub use time_per_pixel::*;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraClip {
    pub id: String,
    pub start_at: Time,
    pub end_at: Time,
    pub camera_angle: CameraAngle,
}

#[derive(Debug)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Sequence {
    pub tracks: Vec<Track>,
}

impl Sequence {
    pub fn get_clip(&self, id: &str) -> Option<Clip> {
        for track in &self.tracks {
            match track {
                Track::Camera(track) => {
                    for clip in &track.clips {
                        if clip.id == id {
                            return Some(Clip::Camera(clip));
                        }
                    }
                }
                Track::Subtitle(track) => {
                    for clip in &track.clips {
                        if clip.id == id {
                            return Some(Clip::Subtitle(clip));
                        }
                    }
                }
            }
        }
        None
    }
    pub fn get_mut_clip<'a>(&'a mut self, id: &str) -> Option<MutableClip<'a>> {
        for track in &mut self.tracks {
            match track {
                Track::Camera(track) => {
                    for clip in &mut track.clips {
                        if clip.id == id {
                            return Some(MutableClip::Camera(clip));
                        }
                    }
                }
                Track::Subtitle(track) => {
                    for clip in &mut track.clips {
                        if clip.id == id {
                            return Some(MutableClip::Subtitle(clip));
                        }
                    }
                }
            }
        }
        None
    }
}

impl TryFrom<Vec<u8>> for Sequence {
    fn try_from(value: Vec<u8>) -> Result<Sequence, String> {
        match String::from_utf8(value) {
            Ok(string) => match serde_json::from_str::<Sequence>(&string) {
                Ok(sequence) => Ok(sequence),
                Err(error) => Err(error.to_string()),
            },
            Err(error) => Err(error.to_string()),
        }
    }

    type Error = String;
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
