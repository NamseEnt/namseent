use self::camera_clip_body::{CameraClipBody, CameraClipBodyProps};
use crate::app::{
    editor::{events::EditorEvent, job::Job, TimelineRenderContext},
    types::{CameraClip, CameraTrack, ClipReplacer, PixelSize},
};
use namui::prelude::*;
use std::collections::LinkedList;
pub mod camera_clip_body;

pub struct CameraTrackBody {}
pub struct CameraTrackBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub track: &'a CameraTrack,
    pub context: &'a TimelineRenderContext<'a>,
}

fn move_clip_at_last(track: &mut CameraTrack, clip_ids: Vec<&String>) {
    let mut clips = LinkedList::new();
    track.clips.iter().for_each(|clip| {
        if clip_ids.contains(&&clip.id) {
            clips.push_back(clip.clone());
        } else {
            clips.push_front(clip.clone());
        }
    });
    track.clips = clips.into_iter().collect::<Vec<_>>().into();
}

impl CameraTrackBody {
    pub fn render(props: &CameraTrackBodyProps) -> RenderingTree {
        let clips = match &props.context.job {
            Some(Job::MoveCameraClip(job)) => {
                let mut track = props.track.clone();

                let mut moving_clips = vec![];
                track
                    .clips
                    .iter()
                    .filter(|clip| job.clip_ids.contains(&clip.id))
                    .for_each(|clip| {
                        moving_clips.push(clip.clone());
                    });

                let moving_clip_ids = job.clip_ids.iter().collect::<Vec<_>>();
                track.move_clips_delta(&moving_clip_ids, job.get_delta_time());

                let delta_time = job.get_delta_time();

                let mut track = moving_clips.iter().fold(track, |track, moving_clip| {
                    track
                        .replace_clip(&moving_clip.id, |clip| {
                            Ok(CameraClip {
                                id: clip.id.clone(),
                                start_at: moving_clip.start_at + delta_time,
                                end_at: moving_clip.end_at + delta_time,
                                camera_angle: clip.camera_angle.clone(),
                            })
                        })
                        .unwrap()
                });

                move_clip_at_last(&mut track, moving_clip_ids);

                track.clips
            }
            Some(Job::ResizeCameraClip(job)) => {
                let mut track = props.track.clone();
                job.resize_clip_in_track(&mut track);
                track.clips
            }
            _ => props.track.clips.clone(),
        };

        let body_border = rect(RectParam {
            x: 0.0,
            y: 0.0,
            width: props.width,
            height: props.height,
            style: RectStyle {
                stroke: Some(RectStroke {
                    border_position: BorderPosition::Middle,
                    color: Color::BLACK,
                    width: 1.0,
                }),
                ..Default::default()
            },
        })
        .attach_event(move |builder| {
            let timeline_start_at = props.context.start_at;
            let time_per_pixel = props.context.time_per_pixel;
            builder.on_mouse_up(move |event| {
                if event.button == Some(MouseButton::Right) {
                    namui::event::send(EditorEvent::CameraTrackBodyRightClickEvent {
                        mouse_global_xy: event.global_xy,
                        mouse_position_in_time: timeline_start_at
                            + PixelSize(event.local_xy.x) * time_per_pixel,
                    })
                }
            })
        });
        render![
            body_border,
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
            ),
        ]
    }
}
