use super::*;

pub struct GradationsProps<'a> {
    pub wh: Wh<Px>,
    pub gap_px: Px,
    pub gradations: &'a Vec<Gradation>,
}
pub const SUB_GRADATION_FREQUENCY: i32 = 5;

pub fn render_gradations(props: &GradationsProps) -> RenderingTree {
    let gradation_height = props.wh.height * 2.0 / 3.0;
    let sub_gradation_height = gradation_height / 3.0;

    let gradation_paint = PaintBuilder::new()
        .set_color(Color::grayscale_f01(0.5))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(px(2.0));

    let sub_gradation_paint = PaintBuilder::new()
        .set_color(Color::grayscale_f01(0.5))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(px(1.0));

    struct GradationProperty {
        x: Px,
        is_big: bool,
    }
    let mut gradation_properties = Vec::<GradationProperty>::new();

    for &Gradation { x, .. } in props.gradations {
        gradation_properties.push(GradationProperty { x, is_big: true });

        for i in 1..SUB_GRADATION_FREQUENCY {
            gradation_properties.push(GradationProperty {
                x: x + i * props.gap_px / SUB_GRADATION_FREQUENCY,
                is_big: false,
            });
        }
    }

    RenderingTree::Children(
        gradation_properties
            .iter()
            .map(move |GradationProperty { is_big, x }| {
                let gradation_height = if *is_big {
                    gradation_height
                } else {
                    sub_gradation_height
                };
                let path = PathBuilder::new()
                    .move_to(*x, (props.wh.height - gradation_height) / 2.0)
                    .line_to(*x, props.wh.height);
                namui::path(
                    path,
                    if *is_big {
                        gradation_paint.clone()
                    } else {
                        sub_gradation_paint.clone()
                    },
                )
            })
            .collect(),
    )
}
