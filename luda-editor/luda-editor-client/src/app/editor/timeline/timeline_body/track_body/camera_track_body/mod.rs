use self::camera_clip_body::{CameraClipBody, CameraClipBodyProps};
use crate::app::{
    editor::{job::Job, TimelineRenderContext},
    types::CameraTrack,
};
use namui::prelude::*;
use std::sync::Arc;
mod camera_clip_body;

pub struct CameraTrackBody {}
pub struct CameraTrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub track: &'a CameraTrack,
    pub context: &'a TimelineRenderContext<'a>,
}

fn move_clip_at_last(track: &mut CameraTrack, clip_id: &String) {
    let mut clips = track.clips.to_vec();
    let moving_clip_index = clips.iter().position(|clip| clip.id.eq(clip_id)).unwrap();
    clips[moving_clip_index..].rotate_left(1);
    track.clips = clips.into();
}

impl CameraTrackBody {
    pub fn render(props: &CameraTrackBodyProps) -> RenderingTree {
        let clips = match &props.context.job {
            Some(Job::MoveCameraClip(job)) => {
                let mut track = props.track.clone();

                job.order_clips_by_moving_clip(&mut track, true);

                move_clip_at_last(&mut track, &job.clip_id);

                track.clips
            }
            _ => props.track.clips.clone(),
        };

        RenderingTree::Children(
            clips
                .iter()
                .map(|clip| {
                    CameraClipBody::render(&CameraClipBodyProps {
                        track_body_wh: &Wh {
                            width: props.width,
                            height: props.height,
                        },
                        clip: clip,
                        context: props.context,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }
}
