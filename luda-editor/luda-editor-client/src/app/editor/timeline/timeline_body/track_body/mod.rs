use crate::app::{editor::TimelineRenderContext, types::Track};

use self::camera_track_body::{CameraTrackBody, CameraTrackBodyProps};
use namui::prelude::*;
mod camera_track_body;
use self::subtitle_track_body::{SubtitleTrackBody, SubtitleTrackBodyProps};
mod subtitle_track_body;

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
            Track::Subtitle(subtitle_track) => SubtitleTrackBody::render(&SubtitleTrackBodyProps {
                width: props.width,
                height: props.height,
                track: subtitle_track,
                context: props.context,
            }),
        }
    }
}
