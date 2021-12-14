pub enum Track {
    Camera(Vec<CameraClip>),
    Subtitle(Vec<SubtitleClip>),
}
#[derive(Debug, Clone, Copy)]
pub struct Time {
    milliseconds: i64,
}
impl std::ops::Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Time) -> Self::Output {
        Time {
            milliseconds: self.milliseconds - rhs.milliseconds,
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

#[derive(Debug, Clone, Copy)]
pub struct PixelSize(pub f32);

#[derive(Debug, Clone, Copy)]
pub struct TimePerPixel {
    time: Time,
    pixel_size: PixelSize,
}

pub struct CameraClip {
    pub id: String,
    pub start_at: Time,
    pub end_at: Time,
    pub camera_angle: CameraAngle,
}

pub enum Clip {
    Camera(CameraClip),
    Subtitle(SubtitleClip),
}

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

pub struct PointSize {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

pub fn get_sample_sequence() -> Sequence {
    Sequence {
        tracks: vec![Track::Camera(vec![CameraClip {
            id: "1".to_string(),
            start_at: Time::sec(0),
            end_at: Time::sec(5),
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
        }])],
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
