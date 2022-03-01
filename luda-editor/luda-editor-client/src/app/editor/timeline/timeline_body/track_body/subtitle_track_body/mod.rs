use self::subtitle_clip_body::{SubtitleClipBody, SubtitleClipBodyProps};
use crate::app::{
    editor::{job::Job, TimelineRenderContext},
    types::SubtitleTrack,
};
use namui::prelude::*;
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
            Some(Job::MoveClip(job)) => {
                let track = job.move_clip_in_track(props.track.clone());
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
