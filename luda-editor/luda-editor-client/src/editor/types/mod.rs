mod camera_angle;
pub use camera_angle::*;
use namui::prelude::*;
use std::{array::IntoIter, collections::HashMap};
mod pixel_size;
pub use pixel_size::*;

pub enum Track {
    Camera(CameraTrack),
    Subtitle(SubtitleTrack),
}
#[derive(Debug, Clone)]
pub struct CameraTrack {
    pub clips: Vec<CameraClip>,
}
#[derive(Debug, Clone)]
pub struct SubtitleTrack {
    pub clips: Vec<SubtitleClip>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    milliseconds: i64,
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

#[derive(Debug, Clone)]
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

pub struct SubtitlePlayDurationMeasurer {
    minimum_play_durations: HashMap<Language, Time>,
    play_duration_per_character: HashMap<Language, Time>,
}
impl SubtitlePlayDurationMeasurer {
    pub fn get_play_duration(&self, subtitle: &Subtitle, language: &Language) -> Time {
        let minimum_play_duration = self.minimum_play_durations.get(language).unwrap();
        let play_duration_per_character = self.play_duration_per_character.get(language).unwrap();
        let play_duration = Time::from_ms(
            (subtitle.language_text_map.get(language).unwrap().len() as f64
                * play_duration_per_character.milliseconds as f64)
                .ceil() as i64,
        );
        if play_duration < *minimum_play_duration {
            *minimum_play_duration
        } else {
            play_duration
        }
    }

    pub(crate) fn new() -> SubtitlePlayDurationMeasurer {
        SubtitlePlayDurationMeasurer {
            // TODO: Check minimum play duration
            minimum_play_durations: HashMap::<_, _>::from_iter(IntoIter::new([(
                Language::Ko,
                Time::from_ms(1000),
            )])),
            play_duration_per_character: HashMap::<_, _>::from_iter(IntoIter::new([(
                Language::Ko,
                Time::from_ms(100),
            )])),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubtitleClip {
    pub id: String,
    pub start_at: Time,
    pub subtitle: Subtitle,
}

#[derive(Debug, Clone)]
pub struct Subtitle {
    pub id: String,
    pub language_text_map: HashMap<Language, String>,
}

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

    pub(crate) fn update_camera_clip(&mut self, id: &str, selected_camera_clip: CameraClip) {
        for track in &mut self.tracks {
            match track {
                Track::Camera(track) => {
                    for clip in &mut track.clips {
                        if clip.id == id {
                            *clip = selected_camera_clip;
                            return;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circumscribed {
    pub center: Xy<f32>,
    pub radius: f32,
}

pub fn get_sample_sequence() -> Sequence {
    Sequence {
        tracks: vec![
            Track::Camera(CameraTrack {
                clips: vec![
                    CameraClip {
                        id: "1".to_string(),
                        start_at: Time::sec(0),
                        end_at: Time::sec(1),
                        camera_angle: CameraAngle {
                            character_pose_emotion: CharacterPoseEmotion(
                                "피디".to_string(),
                                "기본".to_string(),
                                "미소".to_string(),
                            ),
                            source_01_circumscribed: Circumscribed {
                                center: Xy { x: 0.25, y: 0.25 },
                                radius: 0.5259040471894634,
                            },
                            crop_screen_01_rect: LtrbRect {
                                left: 0.0,
                                top: 0.0,
                                right: 1.0,
                                bottom: 1.0,
                            },
                        },
                    },
                    CameraClip {
                        id: "2".to_string(),
                        start_at: Time::sec(1),
                        end_at: Time::sec(3),
                        camera_angle: CameraAngle {
                            character_pose_emotion: CharacterPoseEmotion(
                                "피디".to_string(),
                                "기본".to_string(),
                                "미소".to_string(),
                            ),
                            source_01_circumscribed: Circumscribed {
                                center: Xy { x: 0.25, y: 0.25 },
                                radius: 0.5259040471894634,
                            },
                            crop_screen_01_rect: LtrbRect {
                                left: 0.0,
                                top: 0.0,
                                right: 1.0,
                                bottom: 1.0,
                            },
                        },
                    },
                    CameraClip {
                        id: "3".to_string(),
                        start_at: Time::sec(3),
                        end_at: Time::sec(6),
                        camera_angle: CameraAngle {
                            character_pose_emotion: CharacterPoseEmotion(
                                "피디".to_string(),
                                "기본".to_string(),
                                "미소".to_string(),
                            ),
                            source_01_circumscribed: Circumscribed {
                                center: Xy { x: 0.25, y: 0.25 },
                                radius: 0.5259040471894634,
                            },
                            crop_screen_01_rect: LtrbRect {
                                left: 0.0,
                                top: 0.0,
                                right: 1.0,
                                bottom: 1.0,
                            },
                        },
                    },
                    CameraClip {
                        id: "4".to_string(),
                        start_at: Time::sec(6),
                        end_at: Time::sec(10),
                        camera_angle: CameraAngle {
                            character_pose_emotion: CharacterPoseEmotion(
                                "피디".to_string(),
                                "기본".to_string(),
                                "미소".to_string(),
                            ),
                            source_01_circumscribed: Circumscribed {
                                center: Xy { x: 0.25, y: 0.25 },
                                radius: 0.5259040471894634,
                            },
                            crop_screen_01_rect: LtrbRect {
                                left: 0.0,
                                top: 0.0,
                                right: 1.0,
                                bottom: 1.0,
                            },
                        },
                    },
                    CameraClip {
                        id: "5".to_string(),
                        start_at: Time::sec(10),
                        end_at: Time::sec(15),
                        camera_angle: CameraAngle {
                            character_pose_emotion: CharacterPoseEmotion(
                                "피디".to_string(),
                                "기본".to_string(),
                                "미소".to_string(),
                            ),
                            source_01_circumscribed: Circumscribed {
                                center: Xy { x: 0.5, y: 0.5 },
                                radius: 0.5259040471894634,
                            },
                            crop_screen_01_rect: LtrbRect {
                                left: 0.5,
                                top: 0.5,
                                right: 1.0,
                                bottom: 1.0,
                            },
                        },
                    },
                    CameraClip {
                        id: "6".to_string(),
                        start_at: Time::sec(15),
                        end_at: Time::sec(21),
                        camera_angle: CameraAngle {
                            character_pose_emotion: CharacterPoseEmotion(
                                "피디".to_string(),
                                "기본".to_string(),
                                "미소".to_string(),
                            ),
                            source_01_circumscribed: Circumscribed {
                                center: Xy { x: 0.25, y: 0.25 },
                                radius: 0.25,
                            },
                            crop_screen_01_rect: LtrbRect {
                                left: 0.2,
                                top: 0.4,
                                right: 1.0,
                                bottom: 1.0,
                            },
                        },
                    },
                ],
            }),
            Track::Subtitle(SubtitleTrack {
                clips: vec![
                    SubtitleClip {
                        id: "s-1-clip".to_string(),
                        start_at: Time::sec(0),
                        subtitle: Subtitle {
                            id: "s-1".to_string(),
                            language_text_map: HashMap::<_, _>::from_iter(IntoIter::new([(
                                Language::Ko,
                                "안녕하세요".to_string(),
                            )])),
                        },
                    },
                    SubtitleClip {
                        id: "s-2-clip".to_string(),
                        start_at: Time::sec(0),
                        subtitle: Subtitle {
                            id: "s-2".to_string(),
                            language_text_map: HashMap::<_, _>::from_iter(IntoIter::new([(
                                Language::Ko,
                                "세상!".to_string(),
                            )])),
                        },
                    },
                ],
            }),
        ],
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
