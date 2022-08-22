use namui::prelude::*;

pub struct PlaybackTimeViewProps {
    pub playback_time: Time,
    pub rect: namui::Rect<Px>,
}

pub(super) fn render_playback_time_view(props: &PlaybackTimeViewProps) -> namui::RenderingTree {
    let total_milliseconds = props.playback_time.as_millis() as u32;
    let total_seconds = total_milliseconds / 1000;

    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let milliseconds = total_milliseconds % 1000;

    let text = format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds);

    render([
        namui::rect(namui::RectParam {
            rect: Rect::Xywh {
                x: props.rect.x(),
                y: props.rect.y(),
                width: props.rect.width(),
                height: props.rect.height(),
            },
            style: namui::RectStyle {
                stroke: Some(namui::RectStroke {
                    border_position: namui::BorderPosition::Inside,
                    color: namui::Color::BLACK,
                    width: px(1.0),
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        namui::text(namui::TextParam {
            x: props.rect.x() + props.rect.width() / 2.0,
            y: props.rect.y() + props.rect.height() / 2.0,
            align: namui::TextAlign::Center,
            baseline: namui::TextBaseline::Middle,
            font_type: namui::FontType {
                font_weight: namui::FontWeight::REGULAR,
                language: namui::Language::Ko,
                serif: false,
                size: int_px(20),
            },
            style: namui::TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
            text,
        }),
    ])
}
