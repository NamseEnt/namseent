use namui::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn glow(
    text: impl AsRef<str>,
    font: Font,
    xy: Xy<Px>,
    paint: Paint,
    align: TextAlign,
    baseline: TextBaseline,
    blur: Blur,
    glow_thick: Px,
    glow_color: Color,
) -> RenderingTree {
    let front = DrawCommand::Text {
        command: TextDrawCommand {
            text: text.as_ref().to_string(),
            font: font.clone(),
            x: xy.x,
            y: xy.y,
            paint: paint.clone().set_blend_mode(BlendMode::HardLight),
            align,
            baseline,
            max_width: None,
            line_height_percent: 100.percent(),
            underline: None,
        },
    };
    let back = DrawCommand::Text {
        command: TextDrawCommand {
            text: text.as_ref().to_string(),
            font,
            x: xy.x,
            y: xy.y,
            paint: paint
                .set_style(PaintStyle::StrokeAndFill)
                .set_stroke_width(glow_thick)
                .set_color(glow_color)
                .set_mask_filter(MaskFilter::Blur { blur }),
            align,
            baseline,
            max_width: None,
            line_height_percent: 100.percent(),
            underline: None,
        },
    };
    RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![back, front],
        }],
    })
}
