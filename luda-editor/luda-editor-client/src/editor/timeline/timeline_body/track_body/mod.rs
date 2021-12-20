use crate::editor::{types::Track, Timeline};
use namui::prelude::*;

use self::camera_track_body::{CameraTrackBody, CameraTrackBodyProps};
mod camera_track_body;

pub struct TrackBody {}
pub struct TrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub track: Track,
    pub timeline: &'a Timeline,
}
impl TrackBody {
    pub fn render(props: &TrackBodyProps) -> RenderingTree {
        match props.track {
            Track::Camera(camera_track) => CameraTrackBody::render(&CameraTrackBodyProps {
                width: props.width,
                height: props.height,
                track: camera_track,
                timeline: props.timeline,
            }),
            Track::Subtitle(_) => todo!(),
        }
    }
}
