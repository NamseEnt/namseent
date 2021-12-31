use crate::editor::{types::Track, Timeline, TimelineRenderContext};
use namui::prelude::*;

use self::camera_track_body::{CameraTrackBody, CameraTrackBodyProps};
mod camera_track_body;

pub struct TrackBody {}
pub struct TrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub track: &'a Track,
    pub context: &'a TimelineRenderContext<'a>,
}
impl TrackBody {
    pub fn render(props: &TrackBodyProps) -> RenderingTree {
        match props.track {
            Track::Camera(camera_track) => CameraTrackBody::render(&CameraTrackBodyProps {
                width: props.width,
                height: props.height,
                track: camera_track,
                context: props.context,
            }),
            Track::Subtitle(_) => todo!(),
        }
    }
}
