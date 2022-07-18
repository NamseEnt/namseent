mod play_head;
mod playback_time_view;
mod time_ruler;
pub mod timeline_body;
mod timeline_header;
mod track_header;

use super::job::Job;
use crate::app::{
    editor::{
        events::EditorEvent,
        timeline::{
            play_head::{render_play_head, PlayHeadProps},
            timeline_body::{TimelineBody, TimelineBodyProps},
            timeline_header::{TimelineHeader, TimelineHeaderProps},
        },
    },
    storage::GithubStorage,
    types::{Sequence, SubtitlePlayDurationMeasure},
};
use namui::prelude::*;
use playback_time_view::*;
use std::sync::Arc;
use time_ruler::*;

pub struct Timeline {
    header_width: Px,
    time_ruler_height: Px,
    pub start_at: Time,
    pub time_per_px: TimePerPx,
    timeline_body: TimelineBody,
}
impl Timeline {
    pub fn new() -> Self {
        Self {
            header_width: px(200.0),
            time_ruler_height: px(20.0),
            time_per_px: Time::Ms(50.0) / px(1.0),
            start_at: Time::Sec(0.0),
            timeline_body: TimelineBody::new(),
        }
    }
}
pub struct TimelineProps<'a> {
    pub rect: namui::Rect<Px>,
    pub playback_time: Time,
    pub job: &'a Option<Job>,
    pub selected_clip_ids: &'a [&'a String],
    pub sequence: &'a Sequence,
    pub subtitle_play_duration_measurer: &'a dyn SubtitlePlayDurationMeasure,
    pub storage: Arc<dyn GithubStorage>,
}
pub struct TimelineRenderContext<'a> {
    pub time_per_px: TimePerPx,
    pub job: &'a Option<Job>,
    pub selected_clip_ids: &'a [&'a String],
    start_at: Time,
    pub subtitle_play_duration_measurer: &'a dyn SubtitlePlayDurationMeasure,
    pub language: Language, // TODO : Set this from setting page
}
impl Timeline {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::TimelineMoveEvent { px } => {
                    self.start_at += px * self.time_per_px;
                }
                EditorEvent::TimelineZoomEvent {
                    delta,
                    anchor_x_in_timeline,
                } => {
                    let zoom_by_wheel = |target: f32, delta: f32| -> f32 {
                        const STEP: f32 = 400.0;
                        const MIN: f32 = 10.0;
                        const MAX: f32 = 1000.0;

                        let wheel = STEP * (target / 10.0).log2();

                        let next_wheel = wheel + delta;

                        let zoomed = num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);
                        zoomed
                    };
                    let time_of_mouse_position =
                        self.start_at + anchor_x_in_timeline * self.time_per_px;

                    let next_ms_per_px =
                        zoom_by_wheel((self.time_per_px * px(1.0)).as_millis(), *delta);
                    let next_time_per_px = Time::Ms(next_ms_per_px) / px(1.0);

                    let next_start_at =
                        time_of_mouse_position - anchor_x_in_timeline * next_time_per_px;

                    self.time_per_px = next_time_per_px;
                    self.start_at = next_start_at;
                }
                _ => {}
            }
        }

        self.timeline_body.update(event);
    }

    pub fn render(&self, props: TimelineProps) -> namui::RenderingTree {
        let context = TimelineRenderContext {
            time_per_px: self.time_per_px,
            job: props.job,
            selected_clip_ids: &props.selected_clip_ids,
            start_at: self.start_at,
            subtitle_play_duration_measurer: props.subtitle_play_duration_measurer,
            language: Language::Ko,
        };
        let rect = props.rect;
        let body_width = rect.width() - self.header_width;
        let track_body_height = rect.height() - self.time_ruler_height;
        render([
            namui::rect(namui::RectParam {
                rect: Rect::Xywh {
                    x: rect.x(),
                    y: rect.y(),
                    width: rect.width(),
                    height: rect.height(),
                },
                style: namui::RectStyle {
                    fill: Some(namui::RectFill {
                        color: namui::Color::TRANSPARENT,
                    }),
                    ..Default::default()
                },
                // TODO: id: state.timelineBorderId,
                ..Default::default()
            }),
            namui::translate(
                rect.x(),
                rect.y(),
                render([
                    render_playback_time_view(&PlaybackTimeViewProps {
                        rect: namui::Rect::Xywh {
                            x: px(0.0),
                            y: px(0.0),
                            width: self.header_width,
                            height: self.time_ruler_height,
                        },
                        playback_time: props.playback_time,
                    }),
                    render_time_ruler(TimeRulerProps {
                        rect: namui::Rect::Xywh {
                            x: self.header_width,
                            y: px(0.0),
                            width: rect.width() - self.header_width,
                            height: self.time_ruler_height,
                        },
                        start_at: self.start_at,
                        time_per_px: self.time_per_px,
                    }),
                    namui::translate(
                        px(0.0),
                        self.time_ruler_height,
                        render([
                            TimelineHeader::render(TimelineHeaderProps {
                                width: self.header_width,
                                height: track_body_height,
                                tracks: &props.sequence.tracks,
                            }),
                            namui::translate(
                                self.header_width,
                                px(0.0),
                                TimelineBody::render(TimelineBodyProps {
                                    width: body_width,
                                    height: track_body_height,
                                    tracks: &props.sequence.tracks,
                                    context: &context,
                                    storage: props.storage,
                                }),
                            ),
                        ]),
                    ),
                    namui::translate(
                        self.header_width,
                        px(0.0),
                        namui::clip(
                            namui::PathBuilder::new().add_rect(Rect::Ltrb {
                                left: px(0.0),
                                top: px(0.0),
                                right: body_width,
                                bottom: rect.height(),
                            }),
                            namui::ClipOp::Intersect,
                            render_play_head(&PlayHeadProps {
                                start_at: context.start_at,
                                time_per_px: context.time_per_px,
                                time_ruler_height: self.time_ruler_height,
                                track_body_height,
                                playback_time: props.playback_time,
                            }),
                        ),
                    ),
                ]),
            ),
        ])
    }
}
