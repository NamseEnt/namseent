use crate::app::types::Time;
use namui::prelude::*;

pub struct PlaybackTimeViewProps<'a> {
    pub playback_time: &'a Time,
    pub xywh: namui::XywhRect<f32>,
}

pub(super) fn render_playback_time_view(props: &PlaybackTimeViewProps) -> namui::RenderingTree {
    let total_milliseconds = props.playback_time.get_total_milliseconds() as u32;
    let total_seconds = total_milliseconds / 1000;

    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let milliseconds = total_milliseconds % 1000;

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
