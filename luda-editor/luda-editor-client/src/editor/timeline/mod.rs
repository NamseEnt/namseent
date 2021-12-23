use namui::prelude::*;
mod playback_time_view;
use super::{
    job::Job,
    types::{PixelSize, Sequence, Time, TimePerPixel},
    Clip,
};
use crate::editor::timeline::{
    timeline_body::{TimelineBody, TimelineBodyProps},
    timeline_header::{TimelineHeader, TimelineHeaderProps},
};
use playback_time_view::*;
mod time_ruler;
use time_ruler::*;
mod timeline_body;
mod timeline_header;
mod track_header;

pub struct Timeline {
    header_width: f32,
    time_ruler_height: f32,
    pub selected_clip_id: Option<String>,
    pub sequence: Sequence,
    start_at: Time,
    pub time_per_pixel: TimePerPixel,
    pub job: Option<Job>,
}

impl Timeline {
    pub fn new(sequence: Sequence) -> Self {
        Self {
            header_width: 200.0,
            time_ruler_height: 20.0,
            selected_clip_id: None,
            sequence,
            time_per_pixel: TimePerPixel::new(Time::ms(50), PixelSize(1.0)),
            start_at: Time::sec(0),
            job: None,
        }
    }
}

pub struct TimelineProps {
    pub xywh: namui::XywhRect<f32>,
    pub playback_time: chrono::Duration,
}

impl namui::Entity for Timeline {
    type Props = TimelineProps;

    fn update(&mut self, event: &dyn std::any::Any) {}

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let xywh = props.xywh;
        let body_width = xywh.width - self.header_width;
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
                    PlaybackTimeView::new().render(&PlaybackTimeViewProps {
                        xywh: namui::XywhRect {
                            x: 0.0,
                            y: 0.0,
                            width: self.header_width,
                            height: self.time_ruler_height,
                        },
                        playback_time: props.playback_time,
                    }),
                    TimeRuler::new().render(&TimeRulerProps {
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
                                height: xywh.height,
                                tracks: &self.sequence.tracks,
                            }),
                            namui::translate(
                                self.header_width,
                                0.0,
                                TimelineBody::render(&TimelineBodyProps {
                                    width: body_width,
                                    height: xywh.height,
                                    tracks: &self.sequence.tracks,
                                    timeline: self,
                                })
                            ),
                        ]
                    )
                ]
            )
        ]
    }
}
