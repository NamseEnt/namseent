use self::camera_clip_body::{CameraClipBody, CameraClipBodyProps};
use crate::app::{
    editor::{job::Job, TimelineRenderContext},
    types::{CameraClip, CameraTrack, ClipReplacer},
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

                track.move_clip_delta(&job.clip_id, job.get_delta_time());

                let mut track = track
                    .replace_clip(&job.clip_id, |clip| {
                        Ok(CameraClip {
                            id: clip.id.clone(),
                            start_at: clip.start_at + job.get_delta_time(),
                            end_at: clip.end_at + job.get_delta_time(),
                            camera_angle: clip.camera_angle.clone(),
                        })
                    })
                    .unwrap();

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
