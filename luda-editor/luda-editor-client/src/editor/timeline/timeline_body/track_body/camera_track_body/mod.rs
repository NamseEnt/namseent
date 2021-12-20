use std::rc::Rc;

use crate::editor::{
    job::Job,
    timeline::timeline_body::track_body::camera_track_body::camera_clip_body::{
        CameraClipBody, CameraClipBodyProps,
    },
    types::{CameraClip, CameraTrack, Track},
    Timeline,
};
use namui::prelude::*;
mod camera_clip_body;

pub struct CameraTrackBody {}
pub struct CameraTrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub track: &'a CameraTrack,
    pub timeline: &'a Timeline,
}

fn move_clip_at_last(track: &CameraTrack, clip_id: &String) -> CameraTrack {
    let mut new_track = track.clone();
    let clips = new_track.clips;
    let moving_clip_index = clips
        .iter()
        .position(|clip| {
            clip.id
                .eq(clip_id)
        })
        .unwrap();
    let moving_clip = clips.remove(moving_clip_index);
    clips.push(moving_clip);
    new_track
}

impl CameraTrackBody {
    pub fn render(props: &CameraTrackBodyProps) -> RenderingTree {
        let clips = match &props
            .timeline
            .job
        {
            Some(Job::MoveCameraClip(job)) => {
                let track = props
                    .track
                    .clone();

                let time_per_pixel = &props
                    .timeline
                    .time_per_pixel;
                job.order_clips_by_moving_clip(&mut track, time_per_pixel, true);

                let track = move_clip_at_last(&mut track, &job.clip_id);

                track.clips
            }
            None => {
                props
                    .track
                    .clips
            }
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
                        timeline: props.timeline,
                    })
                })
                .collect::<Vec<_>>()]
        ]
    }
}
