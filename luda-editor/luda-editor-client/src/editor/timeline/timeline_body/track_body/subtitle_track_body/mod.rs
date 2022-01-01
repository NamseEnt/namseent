use crate::editor::{
    job::Job,
    timeline::timeline_body::track_body::subtitle_track_body::subtitle_clip_body::{
        SubtitleClipBody, SubtitleClipBodyProps,
    },
    types::{SubtitleClip, SubtitleTrack, Track},
    Timeline, TimelineRenderContext,
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

// fn move_clip_at_last(track: &mut SubtitleTrack, clip_id: &String) {
//     let clips = &mut track.clips;
//     let moving_clip_index = clips.iter().position(|clip| clip.id.eq(clip_id)).unwrap();
//     let moving_clip = clips.remove(moving_clip_index);
//     clips.push(moving_clip);
// }

impl SubtitleTrackBody {
    pub fn render(props: &SubtitleTrackBodyProps) -> RenderingTree {
        // TODO
        // let clips = match &props.context.job {
        //     Some(Job::MoveSubtitleClip(job)) => {
        //         let mut track = props.track.clone();

        //         let time_per_pixel = &props.context.time_per_pixel;
        //         job.order_clips_by_moving_clip(&mut track, time_per_pixel, true);

        //         move_clip_at_last(&mut track, &job.clip_id);

        //         track.clips
        //     }
        //     _ => props.track.clips.clone(),
        // };
        let clips = &props.track.clips;

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
