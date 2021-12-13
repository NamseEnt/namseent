use chrono::Duration;

use crate::{engine, render};

pub struct PlaybackTimeView {}
pub struct PlaybackTimeViewProps {
    pub playback_time: Duration,
    pub xywh: engine::XywhRect<f32>,
}

impl PlaybackTimeView {
    pub fn new() -> Self {
        PlaybackTimeView {}
    }
}

impl engine::Entity for PlaybackTimeView {
    type Props = PlaybackTimeViewProps;
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, props: &Self::Props) -> engine::RenderingTree {
        let num_seconds = props.playback_time.num_seconds();

        let seconds = num_seconds % 60;
        let minutes = (num_seconds / 60) % 60;
        let milliseconds = props.playback_time.num_milliseconds() % 1000;

        let text = format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds);

        render![
            engine::rect(engine::RectParam {
                x: props.xywh.x,
                y: props.xywh.y,
                width: props.xywh.width,
                height: props.xywh.height,
                style: engine::RectStyle {
                    stroke: Some(engine::RectStroke {
                        border_position: engine::BorderPosition::Inside,
                        color: engine::Color::BLACK,
                        width: 1.0,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            engine::text(engine::TextParam {
                x: props.xywh.x + props.xywh.width / 2.0,
                y: props.xywh.y + props.xywh.height / 2.0,
                align: engine::TextAlign::Center,
                baseline: engine::TextBaseline::Middle,
                font_type: engine::FontType {
                    font_weight: engine::FontWeight::REGULAR,
                    language: engine::Language::Ko,
                    serif: false,
                    size: 20,
                },
                style: engine::TextStyle {
                    color: engine::Color::BLACK,
                    ..Default::default()
                },
                text: text,
            })
        ]
    }
}
