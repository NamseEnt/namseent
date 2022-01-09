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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Time {
    pub milliseconds: i64,
}
impl Time {
    pub fn zero() -> Self {
        Self { milliseconds: 0 }
    }
    pub fn from_ms(milliseconds: i64) -> Self {
        Self { milliseconds }
    }
}
impl std::ops::Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Time) -> Self::Output {
        Time {
            milliseconds: self.milliseconds - rhs.milliseconds,
        }
    }
}
impl std::ops::Add for Time {
    type Output = Time;
    fn add(self, rhs: Time) -> Self::Output {
        Time {
            milliseconds: self.milliseconds + rhs.milliseconds,
        }
    }
}
impl std::ops::Div<TimePerPixel> for Time {
    type Output = PixelSize;
    fn div(self, rhs: TimePerPixel) -> Self::Output {
        let milliseconds = self.milliseconds as f64 / rhs.time.milliseconds as f64;
        PixelSize(milliseconds as f32 * rhs.pixel_size.0)
    }
}
impl std::ops::Div<i64> for Time {
    type Output = Time;
    fn div(self, rhs: i64) -> Self::Output {
        let milliseconds = self.milliseconds / rhs;
        Time { milliseconds }
    }
}
impl std::ops::Mul<TimePerPixel> for PixelSize {
    type Output = Time;
    fn mul(self, rhs: TimePerPixel) -> Self::Output {
        Time {
            milliseconds: ((self.0 / rhs.pixel_size.0) * (rhs.time.milliseconds as f32)) as i64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimePerPixel {
    time: Time,
    pixel_size: PixelSize,
}

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

impl Time {
    pub fn sec(seconds: i64) -> Time {
        Time {
            milliseconds: seconds * 1000,
        }
    }
    pub fn ms(milliseconds: i64) -> Time {
        Time { milliseconds }
    }
}
impl TimePerPixel {
    pub(crate) fn new(time: Time, pixel_size: PixelSize) -> TimePerPixel {
        TimePerPixel { time, pixel_size }
    }
}

#[derive(Debug, Clone)]
pub struct ImageFilenameObject {
    pub character: String,
    pub pose: String,
    pub emotion: String,
    pub url: String,
}

impl ImageFilenameObject {
    pub fn into_character_pose_emotion(&self) -> CharacterPoseEmotion {
        CharacterPoseEmotion(
            self.character.clone(),
            self.pose.clone(),
            self.emotion.clone(),
        )
    }
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
