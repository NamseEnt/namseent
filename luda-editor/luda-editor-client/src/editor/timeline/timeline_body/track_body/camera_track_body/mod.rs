use std::rc::Rc;

use crate::editor::{
    job::Job,
    timeline::timeline_body::track_body::camera_track_body::camera_clip_body::{
        CameraClipBody, CameraClipBodyProps,
    },
    types::{CameraClip, CameraTrack, Track},
    Timeline, TimelineRenderContext,
};
use namui::prelude::*;
mod camera_clip_body;

pub struct CameraTrackBody {}
pub struct CameraTrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub track: &'a CameraTrack,
    pub context: &'a TimelineRenderContext<'a>,
}

fn move_clip_at_last(track: &mut CameraTrack, clip_id: &String) {
    let clips = &mut track.clips;
    let moving_clip_index = clips.iter().position(|clip| clip.id.eq(clip_id)).unwrap();
    let moving_clip = clips.remove(moving_clip_index);
    clips.push(moving_clip);
}

impl CameraTrackBody {
    pub fn render(props: &CameraTrackBodyProps) -> RenderingTree {
        let clips = match &props.context.job {
            Some(Job::MoveCameraClip(job)) => {
                let mut track = props.track.clone();

                let time_per_pixel = &props.context.time_per_pixel;
                job.order_clips_by_moving_clip(&mut track, time_per_pixel, true);

                move_clip_at_last(&mut track, &job.clip_id);

                track.clips
            }
            _ => props.track.clips.clone(),
        };

        render![
            // TODO : rect
            render![clips
                .iter()
                .map(|clip| {
                    CameraClipBody::render(&CameraClipBodyProps {
                        width: props.width,
                        height: props.height,
                        clip: clip,
                        context: props.context,
                    })
                })
                .collect::<Vec<_>>()]
        ]
    }
}
