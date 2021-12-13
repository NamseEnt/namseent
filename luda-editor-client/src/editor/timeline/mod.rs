use ::namui::*;
use chrono::Duration;
mod playback_time_view;
use super::Clip;
use playback_time_view::*;
mod time_ruler;
use time_ruler::*;

pub struct Timeline {
    xywh: namui::XywhRect<f32>,
    header_width: f32,
    time_ruler_height: f32,
    selected_clip: Option<Clip>,
}

impl Timeline {
    pub fn new(xywh: namui::XywhRect<f32>) -> Self {
        Self {
            xywh,
            header_width: 200.0,
            time_ruler_height: 20.0,
            selected_clip: None,
        }
    }

    pub(crate) fn resize(&mut self, xywh: namui::XywhRect<f32>) {
        self.xywh = xywh;
    }
}

pub struct TimelineProps {
    pub playback_time: chrono::Duration,
}

impl namui::Entity for Timeline {
    type Props = TimelineProps;

    fn update(&mut self, event: &dyn std::any::Any) {}

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        render![
            namui::rect(namui::RectParam {
                x: self.xywh.x,
                y: self.xywh.y,
                width: self.xywh.width,
                height: self.xywh.height,
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
                self.xywh.x,
                self.xywh.y,
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
                            width: self.xywh.width - self.header_width,
                            height: self.time_ruler_height,
                        },
                        duration_per_pixel: DurationPerPixel::new(Duration::zero(), 1), // TODO
                        start_time: Duration::zero(),                                   // TODO
                    }),
                ]
            )
        ]
    }
}
