use namui::prelude::*;
mod playback_time_view;
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
    types::{PixelSize, Sequence, SubtitlePlayDurationMeasurer, Time, TimePerPixel},
};
use playback_time_view::*;
mod time_ruler;
use time_ruler::*;
mod play_head;
mod timeline_body;
mod timeline_header;
mod track_header;

pub struct Timeline {
    header_width: f32,
    time_ruler_height: f32,
    pub start_at: Time,
    pub time_per_pixel: TimePerPixel,
}
impl Timeline {
    pub fn new() -> Self {
        Self {
            header_width: 200.0,
            time_ruler_height: 20.0,
            time_per_pixel: TimePerPixel::new(Time::from_ms(50.0), PixelSize(1.0)),
            start_at: Time::from_sec(0.0),
        }
    }
}
pub struct TimelineProps<'a> {
    pub xywh: namui::XywhRect<f32>,
    pub playback_time: &'a Time,
    pub job: &'a Option<Job>,
    pub selected_clip_id: &'a Option<String>,
    pub sequence: &'a Sequence,
    pub subtitle_play_duration_measurer: &'a SubtitlePlayDurationMeasurer,
}
pub struct TimelineRenderContext<'a> {
    pub time_per_pixel: TimePerPixel,
    pub job: &'a Option<Job>,
    pub selected_clip_id: &'a Option<String>,
    start_at: Time,
    pub subtitle_play_duration_measurer: &'a SubtitlePlayDurationMeasurer,
    pub language: Language, // TODO : Set this from setting page
}
impl Timeline {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::TimelineMoveEvent { pixel } => {
                    self.start_at += pixel * self.time_per_pixel;
                }
                EditorEvent::TimelineZoomEvent {
                    delta,
                    anchor_x_in_timeline,
                } => {
                    let zoom_by_wheel = |target: &f32, delta: &f32| -> f32 {
                        const STEP: f32 = 400.0;
                        const MIN: f32 = 10.0;
                        const MAX: f32 = 1000.0;

                        let wheel = STEP * (target / 10.0).log2();

                        let next_wheel = wheel + delta;

                        let zoomed = num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);
                        zoomed
                    };
                    let time_of_mouse_position =
                        self.start_at + anchor_x_in_timeline * self.time_per_pixel;

                    let next_ms_per_pixel =
                        zoom_by_wheel(&self.time_per_pixel.ms_per_pixel(), delta);
                    let next_time_per_pixel = TimePerPixel::from_ms_per_pixel(&next_ms_per_pixel);

                    let next_start_at =
                        time_of_mouse_position - anchor_x_in_timeline * next_time_per_pixel;

                    self.time_per_pixel = next_time_per_pixel;
                    self.start_at = next_start_at;
                }
                _ => {}
            }
        }
    }

    pub fn render(&self, props: &TimelineProps) -> namui::RenderingTree {
        let context = TimelineRenderContext {
            time_per_pixel: self.time_per_pixel,
            job: props.job,
            selected_clip_id: &props.selected_clip_id,
            start_at: self.start_at,
            subtitle_play_duration_measurer: props.subtitle_play_duration_measurer,
            language: Language::Ko,
        };
        let xywh = props.xywh;
        let body_width = xywh.width - self.header_width;
        let track_body_height = xywh.height - self.time_ruler_height;
        render![
            namui::rect(namui::RectParam {
                x: xywh.x,
                y: xywh.y,
                width: xywh.width,
                height: xywh.height,
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
                xywh.x,
                xywh.y,
                render![
                    render_playback_time_view(&PlaybackTimeViewProps {
                        xywh: namui::XywhRect {
                            x: 0.0,
                            y: 0.0,
                            width: self.header_width,
                            height: self.time_ruler_height,
                        },
                        playback_time: props.playback_time,
                    }),
                    render_time_ruler(&TimeRulerProps {
                        xywh: namui::XywhRect {
                            x: self.header_width,
                            y: 0.0,
                            width: xywh.width - self.header_width,
                            height: self.time_ruler_height,
                        },
                        start_at: self.start_at,
                        time_per_pixel: self.time_per_pixel,
                    }),
                    namui::translate(
                        0.0,
                        self.time_ruler_height,
                        render![
                            TimelineHeader::render(&TimelineHeaderProps {
                                width: self.header_width,
                                height: track_body_height,
                                tracks: &props.sequence.tracks,
                            }),
                            namui::translate(
                                self.header_width,
                                0.0,
                                TimelineBody::render(&TimelineBodyProps {
                                    width: body_width,
                                    height: track_body_height,
                                    tracks: &props.sequence.tracks,
                                    context: &context,
                                })
                            ),
                        ]
                    ),
                    namui::translate(
                        self.header_width,
                        0.0,
                        namui::clip(
                            namui::PathBuilder::new().add_rect(&LtrbRect {
                                left: 0.0,
                                top: 0.0,
                                right: body_width,
                                bottom: xywh.height,
                            }),
                            namui::ClipOp::Intersect,
                            render_play_head(&PlayHeadProps {
                                start_at: &context.start_at,
                                time_per_pixel: &context.time_per_pixel,
                                time_ruler_height: self.time_ruler_height,
                                track_body_height,
                                playback_time: props.playback_time,
                            })
                        ),
                    ),
                ]
            )
        ]
    }
}
