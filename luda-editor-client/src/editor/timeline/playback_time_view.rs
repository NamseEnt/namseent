use ::namui::*;
use chrono::Duration;

pub struct PlaybackTimeView {}
pub struct PlaybackTimeViewProps {
    pub playback_time: Duration,
    pub xywh: namui::XywhRect<f32>,
}

impl PlaybackTimeView {
    pub fn new() -> Self {
        PlaybackTimeView {}
    }
}

impl namui::Entity for PlaybackTimeView {
    type Props = PlaybackTimeViewProps;
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let num_seconds = props.playback_time.num_seconds();

        let seconds = num_seconds % 60;
        let minutes = (num_seconds / 60) % 60;
        let milliseconds = props.playback_time.num_milliseconds() % 1000;

        let text = format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds);

        render![
            namui::rect(namui::RectParam {
                x: props.xywh.x,
                y: props.xywh.y,
                width: props.xywh.width,
                height: props.xywh.height,
                style: namui::RectStyle {
                    stroke: Some(namui::RectStroke {
                        border_position: namui::BorderPosition::Inside,
                        color: namui::Color::BLACK,
                        width: 1.0,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            namui::text(namui::TextParam {
                x: props.xywh.x + props.xywh.width / 2.0,
                y: props.xywh.y + props.xywh.height / 2.0,
                align: namui::TextAlign::Center,
                baseline: namui::TextBaseline::Middle,
                font_type: namui::FontType {
                    font_weight: namui::FontWeight::REGULAR,
                    language: namui::Language::Ko,
                    serif: false,
                    size: 20,
                },
                style: namui::TextStyle {
                    color: namui::Color::BLACK,
                    ..Default::default()
                },
                text: text,
            })
        ]
    }
}
