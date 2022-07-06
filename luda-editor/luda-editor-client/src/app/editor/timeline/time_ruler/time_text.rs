use super::Gradation;
use namui::prelude::*;

pub struct TimeTextsProps<'a> {
    pub height: Px,
    pub time_per_px: TimePerPx,
    pub gradations: &'a Vec<Gradation>,
}

pub fn render_time_texts(props: TimeTextsProps) -> RenderingTree {
    const LEFT_MARGIN_PX: Px = px(5.0);
    const TEXT_SIZE: IntPx = int_px(10);
    RenderingTree::Children(
        props
            .gradations
            .iter()
            .map(|&Gradation { x, at }| {
                let total_milliseconds = at.as_millis() as i32;
                let total_seconds = total_milliseconds / 1000;

                let minutes = total_seconds / 60;
                let seconds = total_seconds % 60;
                let milliseconds = total_milliseconds % 1000;

                let text = format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds);
                namui::text(namui::TextParam {
                    x: (x + LEFT_MARGIN_PX).into(),
                    y: props.height / 2.0,
                    align: namui::TextAlign::Left,
                    baseline: namui::TextBaseline::Middle,
                    font_type: namui::FontType {
                        font_weight: namui::FontWeight::REGULAR,
                        language: namui::Language::Ko,
                        serif: false,
                        size: TEXT_SIZE,
                    },
                    style: namui::TextStyle {
                        color: namui::Color::grayscale_f01(0.5),
                        ..Default::default()
                    },
                    text: text,
                })
            })
            .collect::<Vec<_>>(),
    )
}
