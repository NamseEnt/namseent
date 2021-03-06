use crate::{
    draw::text::{get_bottom_of_baseline, get_left_in_align},
    namui::{self, *},
};

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
#[derive(Clone, Copy, Debug, Default)]
pub struct TextStyle {
    pub border: Option<TextStyleBorder>,
    pub drop_shadow: Option<TextStyleDropShadow>,
    pub color: Color,
    pub background: Option<TextStyleBackground>,
}

#[derive(Clone, Debug)]
pub struct TextParam {
    pub text: String,
    pub x: Px,
    pub y: Px,
    pub align: TextAlign,
    pub baseline: TextBaseline,
    pub font_type: FontType,
    pub style: TextStyle,
}

pub fn text(param: TextParam) -> RenderingTree {
    let font = namui::font::get_font(param.font_type);
    match font {
        None => {
            crate::log!(
                "Font not found: {}",
                serde_json::to_string(&param.font_type).unwrap()
            );
            RenderingTree::Empty
        }
        Some(font) => {
            crate::render![
                draw_background(&param, font.as_ref()),
                namui::RenderingData {
                    draw_calls: vec![namui::DrawCall {
                        commands: vec![
                            // draw_shadow(),
                            draw_border(&param, font.clone()),
                            Some(draw_text(&param, font.clone())),
                        ]
                        .into_iter()
                        .filter_map(|command| command)
                        .collect(),
                    }],
                    ..Default::default()
                }
            ]
        }
    }

    //   return [
    //     drawBackground(textHandleParam),
    //     {
    //       drawCalls: [
    //         {
    //           commands: [
    //             drawShadow(textHandleParam),
    //             drawText(textHandleParam),
    //             drawBorder(textHandleParam),
    //           ].filter((x): x is TextDrawCommand => !!x),
    //         },
    //       ],
    //     },
    //   ];
    // }
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
fn draw_text(param: &TextParam, font: Arc<Font>) -> DrawCommand {
    let text_paint = namui::PaintBuilder::new()
        .set_color(param.style.color)
        .set_style(namui::PaintStyle::Fill)
        .set_anti_alias(true);

    DrawCommand::Text(TextDrawCommand {
        text: param.text.clone(),
        font,
        x: param.x,
        y: param.y,
        paint_builder: text_paint,
        align: param.align,
        baseline: param.baseline,
    })
}
fn draw_border(param: &TextParam, font: Arc<Font>) -> Option<DrawCommand> {
    let border = param.style.border?;

    let border_paint = namui::PaintBuilder::new()
        .set_color(border.color)
        .set_style(namui::PaintStyle::Stroke)
        .set_stroke_width(border.width)
        .set_stroke_join(namui::StrokeJoin::Miter)
        .set_anti_alias(true);

    Some(DrawCommand::Text(TextDrawCommand {
        text: param.text.clone(),
        font,
        x: param.x,
        y: param.y,
        paint_builder: border_paint,
        align: param.align,
        baseline: param.baseline,
    }))
}

fn draw_background(param: &TextParam, font: &Font) -> RenderingTree {
    let style = &param.style;

    let background = &style.background;
    if background.is_none() {
        return RenderingTree::Empty;
    };
    let background = background.as_ref().unwrap();

    let width = get_text_width_internal(
        font,
        &param.text,
        param.style.drop_shadow.map(|drop_shadow| drop_shadow.x),
    );

    let font_metrics = font.metrics;

    let height = -font_metrics.ascent + font_metrics.descent;
    let bottom_of_baseline = get_bottom_of_baseline(&param.baseline, &font_metrics);
    let top = param.y + bottom_of_baseline + font_metrics.ascent;

    let margin = background.margin.unwrap_or(Ltrb::default());

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
        ..Default::default()
    })
}

pub(crate) fn get_text_width_internal(font: &Font, text: &str, drop_shadow_x: Option<Px>) -> Px {
    let glyph_ids = font.get_glyph_ids(text);
    let glyph_widths = font.get_glyph_widths(glyph_ids, Option::None);
    glyph_widths.iter().fold(px(0.0), |acc, cur| acc + cur) + drop_shadow_x.unwrap_or(px(0.0))
}

pub fn get_text_width(text: &str, font_type: FontType, drop_shadow_x: Option<Px>) -> Option<Px> {
    let font = namui::font::get_font(font_type);
    font.map(|font| {
        let glyph_ids = font.get_glyph_ids(text);
        let glyph_widths = font.get_glyph_widths(glyph_ids, Option::None);
        glyph_widths.iter().fold(px(0.0), |acc, cur| acc + cur) + drop_shadow_x.unwrap_or(px(0.0))
    })
}
