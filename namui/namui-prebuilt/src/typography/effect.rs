use namui::*;

#[allow(clippy::too_many_arguments)]
pub fn glow(
    text: impl AsRef<str>,
    font: Font,
    xy: Xy<Px>,
    paint: Paint,
    align: TextAlign,
    baseline: TextBaseline,
    blur_style: BlurStyle,
    sigma: f32,
    glow_thick: Px,
    glow_color: Color,
) -> RenderingTree {
    let gen = |paint: Paint| DrawCommand::Text {
        command: TextDrawCommand {
            text: text.as_ref().to_string(),
            font: font.clone(),
            x: xy.x,
            y: xy.y,
            paint,
            align,
            baseline,
            max_width: None,
            line_height_percent: 100.percent(),
            underline: None,
        }
        .into(),
    };
    let front = gen(paint.clone().set_blend_mode(BlendMode::HardLight));
    let back_stroke = gen(paint
        .clone()
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(glow_thick)
        .set_color(glow_color)
        .set_mask_filter(MaskFilter::Blur { blur_style, sigma }));
    let back_fill = gen(paint
        .clone()
        .set_style(PaintStyle::Fill)
        .set_color(glow_color)
        .set_mask_filter(MaskFilter::Blur { blur_style, sigma }));
    namui::render([back_fill, back_stroke, front].map(RenderingTree::Node))
}
