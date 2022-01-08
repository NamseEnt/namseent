use namui::prelude::*;

use crate::app::{
    editor::{job::Job, TimelineRenderContext},
    types::SubtitleTrack,
};

use self::subtitle_clip_body::{SubtitleClipBody, SubtitleClipBodyProps};
mod subtitle_clip_body;

pub struct SubtitleTrackBody {}
pub struct SubtitleTrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub track: &'a SubtitleTrack,
    pub context: &'a TimelineRenderContext<'a>,
}

impl SubtitleTrackBody {
    pub fn render(props: &SubtitleTrackBodyProps) -> RenderingTree {
        let clips = match &props.context.job {
            Some(Job::MoveSubtitleClip(job)) => {
                let mut track = props.track.clone();

                let time_per_pixel = &props.context.time_per_pixel;
                let moving_clip = track
                    .clips
                    .iter_mut()
                    .find(|clip| clip.id.eq(&job.clip_id))
                    .unwrap();
                job.move_subtitle_clip_by_job(moving_clip, time_per_pixel);

                track.clips
            }
            _ => props.track.clips.clone(),
        };

        RenderingTree::Children(
            clips
                .iter()
                .map(|clip| {
                    SubtitleClipBody::render(&SubtitleClipBodyProps {
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
