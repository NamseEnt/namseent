use crate::editor::{
    job::Job,
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

fn move_clip_at_last(clips: &mut Vec<CameraClip>, clip_id: &String) {
    let moving_clip_index = clips.iter().position(|clip| clip.id.eq(clip_id)).unwrap();
    let moving_clip = clips.remove(moving_clip_index);
    clips.push(moving_clip);
}

impl CameraTrackBody {
    pub fn render(props: &CameraTrackBodyProps) -> RenderingTree {
        let clips = match &props.timeline.job {
            Some(Job::MoveCameraClip(job)) => {
                let mut clips = props.clips.clone();
                let time_per_pixel = &props.timeline.time_per_pixel;
                job.order_clips_by_moving_clip(&mut clips, time_per_pixel, true);

                move_clip_at_last(&mut clips, &job.clip_id);

                clips
            }
            None => props.clips.to_vec(),
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
