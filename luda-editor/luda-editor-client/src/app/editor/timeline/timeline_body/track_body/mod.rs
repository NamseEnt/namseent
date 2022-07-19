pub mod camera_track_body;
mod resizable_clip_body;
mod subtitle_track_body;

use self::camera_track_body::{CameraTrackBody, CameraTrackBodyProps};
use self::subtitle_track_body::{SubtitleTrackBody, SubtitleTrackBodyProps};
use crate::app::types::CameraAngleImageLoader;
use crate::app::{editor::TimelineRenderContext, types::Track};
use namui::prelude::*;
pub use resizable_clip_body::*;
use std::sync::Arc;

pub struct TrackBody {}
pub struct TrackBodyProps<'a> {
    pub width: Px,
    pub height: Px,
    pub track: &'a Track,
    pub context: &'a TimelineRenderContext<'a>,
    pub camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
}
impl TrackBody {
    pub fn render(props: &TrackBodyProps) -> RenderingTree {
        match props.track {
            Track::Camera(camera_track) => CameraTrackBody::render(&CameraTrackBodyProps {
                width: props.width,
                height: props.height,
                track: camera_track,
                context: props.context,
                camera_angle_image_loader: props.camera_angle_image_loader.clone(),
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
