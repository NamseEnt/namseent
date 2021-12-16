use crate::editor::{types::Track, Timeline};
use ::namui::*;

use self::camera_track_body::{CameraTrackBody, CameraTrackBodyProps};
mod camera_track_body;

pub struct TrackBody {}
pub struct TrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub track: &'a Track,
    pub timeline: &'a Timeline,
}
impl TrackBody {
    pub fn render(props: &TrackBodyProps) -> RenderingTree {
        match props.track {
            Track::Camera(camera_track) => CameraTrackBody::render(&CameraTrackBodyProps {
                width: props.width,
                height: props.height,
                clips: &camera_track.0,
                timeline: props.timeline,
            }),
            Track::Subtitle(_) => todo!(),
        }
    }
}
