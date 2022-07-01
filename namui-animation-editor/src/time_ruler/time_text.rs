use super::*;

pub struct TimeTextsProps<'a> {
    pub height: f32,
    pub time_per_px: TimePerPx,
    pub gradations: &'a Vec<Gradation>,
}

pub fn render_time_texts(props: &TimeTextsProps) -> RenderingTree {
    let left_margin_px: Px = Px::from(5.0);
    const TEXT_SIZE: i16 = 10;
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
                    x: (x + left_margin_px).into(),
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
                        color: namui::Color::WHITE,
                        ..Default::default()
                    },
                    text,
                })
            })
            .collect::<Vec<_>>(),
    )
}
