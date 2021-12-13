use crate::{engine, render};
mod playback_time_view;
use super::Clip;
use playback_time_view::*;
mod time_ruler;
use time_ruler::*;

pub struct Timeline {
    xywh: engine::XywhRect<f32>,
    header_width: f32,
    time_ruler_height: f32,
    selected_clip: Option<Clip>,
}

impl Timeline {
    pub fn new(xywh: engine::XywhRect<f32>) -> Self {
        Self {
            xywh,
            header_width: 200.0,
            time_ruler_height: 20.0,
            selected_clip: None,
        }
    }

    pub(crate) fn resize(&mut self, xywh: engine::XywhRect<f32>) {
        self.xywh = xywh;
    }
}

pub struct TimelineProps {
    pub playback_time: chrono::Duration,
}

impl engine::Entity for Timeline {
    type Props = TimelineProps;

    fn update(&mut self, event: &dyn std::any::Any) {}

    fn render(&self, props: &Self::Props) -> engine::RenderingTree {
        render![
            engine::rect(engine::RectParam {
                x: self.xywh.x,
                y: self.xywh.y,
                width: self.xywh.width,
                height: self.xywh.height,
                style: engine::RectStyle {
                    fill: Some(engine::RectFill {
                        color: engine::Color::TRANSPARENT,
                    }),
                    ..Default::default()
                },
                // TODO: id: state.timelineBorderId,
                ..Default::default()
            }),
            engine::translate(
                self.xywh.x,
                self.xywh.y,
                render![
                    PlaybackTimeView::new().render(&PlaybackTimeViewProps {
                        xywh: engine::XywhRect {
                            x: 0.0,
                            y: 0.0,
                            width: self.header_width,
                            height: self.time_ruler_height,
                        },
                        playback_time: props.playback_time,
                    }),
                    engine::translate(self.header_width, 0.0, render![])
                ]
            )
        ]
    }
}
