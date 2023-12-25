use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct TextStyleBorder {
    pub width: Px,
    pub color: Color,
}
#[derive(Clone, Copy, Debug)]
pub struct TextStyleDropShadow {
    pub x: Px,
    pub y: Px,
    pub color: Option<Color>,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TextStyleBackground {
    pub color: Color,
    pub margin: Option<Ltrb<Px>>,
}
#[derive(Clone, Debug)]
pub struct TextStyle {
    pub border: Option<TextStyleBorder>,
    pub drop_shadow: Option<TextStyleDropShadow>,
    pub color: Color,
    pub background: Option<TextStyleBackground>,
    pub line_height_percent: Percent,
    pub underline: Option<Paint>,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            border: Default::default(),
            drop_shadow: Default::default(),
            color: Default::default(),
            background: Default::default(),
            line_height_percent: 110.percent(),
            underline: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextParam {
    pub text: String,
    pub x: Px,
    pub y: Px,
    pub align: TextAlign,
    pub baseline: TextBaseline,
    pub font: Font,
    pub style: TextStyle,
    pub max_width: Option<Px>,
}

pub fn text(param: TextParam) -> RenderingTree {
    crate::render([
        RenderingTree::Node(RenderingData {
            draw_calls: vec![DrawCall {
                commands: vec![
                    // draw_shadow(),
                    draw_border(&param, &param.font),
                    Some(draw_text(&param, &param.font)),
                ]
                .into_iter()
                .flatten()
                .collect(),
            }],
        }),
        draw_background(&param, &param.font),
    ])
}
// type TextHandleParam = TextParam & {
//   font: Font;
// };

// function drawShadow({
//   x,
//   y,
//   align,
//   baseline,
//   text,
//   style: { dropShadow, color },
//   font,
// }: TextHandleParam): TextDrawCommand | undefined {
//   if (!dropShadow) {
//     return;
//   }
//   const shadowPaint = new CanvasKit.Paint();
//   shadowPaint.setColor(dropShadow.color || color);
//   shadowPaint.setStyle(CanvasKit.PaintStyle.Fill);
//   shadowPaint.setAntiAlias(true);

//   return TextDrawCommand({
//     text,
//     font,
//     x: x + dropShadow.x,
//     y: y + dropShadow.y,
//     paint: shadowPaint,
//     align,
//     baseline,
//   });
// }
pub(crate) fn get_text_paint(color: Color) -> Paint {
    namui::Paint::new(color)
        .set_style(namui::PaintStyle::Fill)
        .set_anti_alias(true)
}
fn draw_text(param: &TextParam, font: &Font) -> DrawCommand {
    let text_paint = get_text_paint(param.style.color);

    DrawCommand::Text {
        command: {
            TextDrawCommand {
                text: param.text.clone(),
                font: font.clone(),
                x: param.x,
                y: param.y,
                paint: text_paint,
                align: param.align,
                baseline: param.baseline,
                max_width: param.max_width,
                line_height_percent: param.style.line_height_percent,
                underline: param.style.underline.clone(),
            }
        },
    }
}
fn draw_border(param: &TextParam, font: &Font) -> Option<DrawCommand> {
    let border = param.style.border?;

    let border_paint = namui::Paint::new(border.color)
        .set_style(namui::PaintStyle::Stroke)
        .set_stroke_width(border.width)
        .set_stroke_join(namui::StrokeJoin::Miter)
        .set_anti_alias(true);

    Some(DrawCommand::Text {
        command: TextDrawCommand {
            text: param.text.clone(),
            font: font.clone(),
            x: param.x,
            y: param.y,
            paint: border_paint,
            align: param.align,
            baseline: param.baseline,
            max_width: param.max_width,
            line_height_percent: param.style.line_height_percent,
            underline: None,
        },
    })
}

fn draw_background(param: &TextParam, font: &Font) -> RenderingTree {
    let style = &param.style;

    let background = &style.background;
    if background.is_none() {
        return RenderingTree::Empty;
    };
    let background = background.as_ref().unwrap();

    let paint = get_text_paint(param.style.color);

    let Some(font_metrics) = system::font::font_metrics(font) else {
        crate::log!("Font metrics not found for font: {:?}", font);
        return RenderingTree::Empty;
    };

    let width = system::font::group_glyph(font, &paint).width(&param.text);

    let height = param.line_height_px();
    let bottom_of_baseline = get_bottom_of_baseline(param.baseline, font_metrics);
    let top = param.y + bottom_of_baseline + font_metrics.descent + font_metrics.ascent;

    let margin = background.margin.unwrap_or_default();

    let final_x = -margin.left + get_left_in_align(param.x, param.align, width);
    let final_y = -margin.top + top;
    let final_width = width + margin.left + margin.right;
    let final_height = height + margin.top + margin.bottom;

    rect(RectParam {
        rect: Rect::Xywh {
            x: final_x,
            y: final_y,
            width: final_width,
            height: final_height,
        },
        style: RectStyle {
            fill: Some(RectFill {
                color: background.color,
            }),
            ..Default::default()
        },
    })
}

impl TextParam {
    pub fn line_height_px(&self) -> Px {
        self.font.size.into_px() * self.style.line_height_percent
    }
}
