pub enum Track {
    Camera(Vec<CameraClip>),
    Subtitle(Vec<SubtitleClip>),
}

pub struct CameraClip {
    pub id: String,
    pub start_ms: u64,
    pub end_ms: u64,
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
            start_ms: 0,
            end_ms: 1000,
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
