use crate::app::types::{PixelSize, Time, TimePerPixel};
use namui::prelude::*;

pub struct GradationsProps {
    pub wh: Wh<f32>,
    pub start_px: PixelSize,
    pub gap_px: PixelSize,
}

pub const BIG_GRADATION_INTERVAL: i32 = 5;

pub fn render_gradations(props: &GradationsProps) -> RenderingTree {
    let big_gradation_height = props.wh.height * 2.0 / 3.0;
    let small_gradation_height = big_gradation_height / 3.0;

    let big_gradation_paint = PaintBuilder::new()
        .set_color(Color::gary_scale_f01(0.5))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.0);

    let small_gradation_paint = PaintBuilder::new()
        .set_color(Color::gary_scale_f01(0.5))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(1.0);

    let big_gradation_xs =
        get_big_gradation_xs(PixelSize(props.wh.width), props.start_px, props.gap_px);

    struct GradationProperty {
        x: PixelSize,
        is_big: bool,
    }
    let mut gradation_properties = Vec::<GradationProperty>::new();

    for big_gradation_x in big_gradation_xs {
        gradation_properties.push(GradationProperty {
            x: big_gradation_x,
            is_big: true,
        });
        for i in 1..5 {
            gradation_properties.push(GradationProperty {
                x: big_gradation_x + i * props.gap_px,
                is_big: false,
            });
        }
    }

    RenderingTree::Children(
        gradation_properties
            .iter()
            .map(move |GradationProperty { is_big, x }| {
                let gradation_height = if *is_big {
                    big_gradation_height
                } else {
                    small_gradation_height
                };
                let path = PathBuilder::new()
                    .move_to(x.into(), (props.wh.height - gradation_height) / 2.0)
                    .line_to(x.into(), props.wh.height);
                namui::path(
                    path,
                    if *is_big {
                        big_gradation_paint.clone()
                    } else {
                        small_gradation_paint.clone()
                    },
                )
            })
            .collect(),
    )
}
fn get_big_gradation_xs(
    time_ruler_width: PixelSize,
    gradation_start_px: PixelSize,
    gradation_gap_px: PixelSize,
) -> Vec<PixelSize> {
    let mut big_gradation_xs = vec![];
    let mut x = gradation_start_px;
    while x < time_ruler_width {
        big_gradation_xs.push(x);
        x += gradation_gap_px * BIG_GRADATION_INTERVAL as f32;
    }
    return big_gradation_xs;
}
