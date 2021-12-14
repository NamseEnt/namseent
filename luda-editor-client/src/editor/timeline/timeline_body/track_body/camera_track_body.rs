use crate::editor::{
    timeline::timeline_body::track_body::camera_track_body::camera_clip_body::{
        CameraClipBody, CameraClipBodyProps,
    },
    types::{CameraClip, Track},
    Timeline,
};
use ::namui::*;
mod camera_clip_body;

pub struct CameraTrackBody {}
pub struct CameraTrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub clips: &'a Vec<CameraClip>,
    pub timeline: &'a Timeline,
}
impl CameraTrackBody {
    pub fn render(props: &CameraTrackBodyProps) -> RenderingTree {
        render![
            // TODO : rect
            render![props
                .clips
                .iter()
                .map(|clip| {
                    CameraClipBody::render(&CameraClipBodyProps {
                        width: props.width,
                        height: props.height,
                        clip: clip,
                        timeline: props.timeline,
                    })
                })
                .collect::<Vec<_>>()]
        ]
    }
}
