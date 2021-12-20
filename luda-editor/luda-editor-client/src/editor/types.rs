use std::rc::Rc;

pub enum Track {
    Camera(CameraTrack),
    Subtitle(Vec<SubtitleClip>),
}
#[derive(Debug, Clone)]
pub struct CameraTrack {
    pub clips: Vec<CameraClip>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    milliseconds: i64,
}
impl Time {
    pub fn zero() -> Self {
        Self {
            milliseconds: 0,
        }
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
        let milliseconds = self.milliseconds as f64
            / rhs
                .time
                .milliseconds as f64;
        PixelSize(
            milliseconds as f32
                * rhs
                    .pixel_size
                    .0,
        )
    }
}
impl std::ops::Div<i64> for Time {
    type Output = Time;
    fn div(self, rhs: i64) -> Self::Output {
        let milliseconds = self.milliseconds / rhs;
        Time {
            milliseconds,
        }
    }
}
impl std::ops::Mul<TimePerPixel> for PixelSize {
    type Output = Time;
    fn mul(self, rhs: TimePerPixel) -> Self::Output {
        Time {
            milliseconds: ((self.0
                / rhs
                    .pixel_size
                    .0)
                * (rhs
                    .time
                    .milliseconds as f32)) as i64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PixelSize(pub f32);

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
    Subtitle(SubtitleClip),
}

#[derive(Debug, Clone)]
pub struct CameraAngle {
    pub image_source_url: String,
    pub source_point_size: PointSize,
    pub dest_point_size: PointSize,
}

pub struct SubtitleClip {
    pub id: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub subtitle: Subtitle,
}

pub struct Subtitle {
    pub text: String,
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
                    todo!()
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct PointSize {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

pub fn get_sample_sequence() -> Sequence {
    Sequence {
        tracks: vec![Track::Camera(CameraTrack {
            clips: vec![
                CameraClip {
                    id: "1".to_string(),
                    start_at: Time::sec(0),
                    end_at: Time::sec(1),
                    camera_angle: CameraAngle {
                        image_source_url: "resources/images/피디-기본-미소.png".to_string(),
                        source_point_size: PointSize {
                            x: 0.25,
                            y: 0.25,
                            size: 0.5259040471894634,
                        },
                        dest_point_size: PointSize {
                            x: 0.0,
                            y: 0.0,
                            size: 1.0,
                        },
                    },
                },
                CameraClip {
                    id: "2".to_string(),
                    start_at: Time::sec(1),
                    end_at: Time::sec(3),
                    camera_angle: CameraAngle {
                        image_source_url: "resources/images/피디-기본-미소.png".to_string(),
                        source_point_size: PointSize {
                            x: 0.25,
                            y: 0.25,
                            size: 0.5259040471894634,
                        },
                        dest_point_size: PointSize {
                            x: 0.0,
                            y: 0.0,
                            size: 1.0,
                        },
                    },
                },
                CameraClip {
                    id: "3".to_string(),
                    start_at: Time::sec(3),
                    end_at: Time::sec(6),
                    camera_angle: CameraAngle {
                        image_source_url: "resources/images/피디-기본-미소.png".to_string(),
                        source_point_size: PointSize {
                            x: 0.25,
                            y: 0.25,
                            size: 0.5259040471894634,
                        },
                        dest_point_size: PointSize {
                            x: 0.0,
                            y: 0.0,
                            size: 1.0,
                        },
                    },
                },
                CameraClip {
                    id: "4".to_string(),
                    start_at: Time::sec(6),
                    end_at: Time::sec(10),
                    camera_angle: CameraAngle {
                        image_source_url: "resources/images/피디-기본-미소.png".to_string(),
                        source_point_size: PointSize {
                            x: 0.25,
                            y: 0.25,
                            size: 0.5259040471894634,
                        },
                        dest_point_size: PointSize {
                            x: 0.0,
                            y: 0.0,
                            size: 1.0,
                        },
                    },
                },
                CameraClip {
                    id: "5".to_string(),
                    start_at: Time::sec(10),
                    end_at: Time::sec(15),
                    camera_angle: CameraAngle {
                        image_source_url: "resources/images/피디-기본-미소.png".to_string(),
                        source_point_size: PointSize {
                            x: 0.25,
                            y: 0.25,
                            size: 0.5259040471894634,
                        },
                        dest_point_size: PointSize {
                            x: 0.0,
                            y: 0.0,
                            size: 1.0,
                        },
                    },
                },
                CameraClip {
                    id: "6".to_string(),
                    start_at: Time::sec(15),
                    end_at: Time::sec(21),
                    camera_angle: CameraAngle {
                        image_source_url: "resources/images/피디-기본-미소.png".to_string(),
                        source_point_size: PointSize {
                            x: 0.25,
                            y: 0.25,
                            size: 0.5259040471894634,
                        },
                        dest_point_size: PointSize {
                            x: 0.0,
                            y: 0.0,
                            size: 1.0,
                        },
                    },
                },
            ],
        })],
    }
}
impl Time {
    pub fn sec(seconds: i64) -> Time {
        Time {
            milliseconds: seconds * 1000,
        }
    }
    pub fn ms(milliseconds: i64) -> Time {
        Time {
            milliseconds,
        }
    }
}
impl TimePerPixel {
    pub(crate) fn new(time: Time, pixel_size: PixelSize) -> TimePerPixel {
        TimePerPixel {
            time,
            pixel_size,
        }
    }
}
