use namui::prelude::*;
mod playback_time_view;
use super::job::Job;
use crate::app::{
    editor::timeline::{
        play_head::{render_play_head, PlayHeadProps},
        timeline_body::{TimelineBody, TimelineBodyProps},
        timeline_header::{TimelineHeader, TimelineHeaderProps},
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
    pub fn update(&mut self, event: &dyn std::any::Any) {}

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
