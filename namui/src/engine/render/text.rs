use crate::engine::{self, *};

#[derive(Clone, Copy, Debug)]
pub struct TextStyleBorder {
    pub width: f32,
    pub color: Color,
}
#[derive(Clone, Copy, Debug)]
pub struct TextStyleDropShadow {
    pub x: f32,
    pub y: f32,
    pub color: Option<Color>,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TextStyleBackground {
    pub color: Color,
    pub margin: Option<LtrbRect>,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TextStyle {
    pub border: Option<TextStyleBorder>,
    pub drop_shadow: Option<TextStyleDropShadow>,
    pub color: Color,
    pub background: Option<TextStyleBackground>,
}

pub struct TextParam {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub align: TextAlign,
    pub baseline: TextBaseline,
    pub font_type: FontType,
    pub style: TextStyle,
}

pub fn text(param: TextParam) -> RenderingTree {
    let font = engine::managers().font_manager.get_font(&param.font_type);
    match font {
        None => {
            engine::log(format!(
                "Font not found: {}",
                serde_json::to_string(&param.font_type).unwrap()
            ));
            RenderingTree::Empty
        }
        Some(font) => {
            render![engine::RenderingData {
                draw_calls: vec![engine::DrawCall {
                    commands: vec![
                        // draw_shadow(),
                        draw_text(param, font),
                        // draw_border(),

                        // commands: vec![engine::DrawCommand::Text(engine::TextDrawCommand {
                        //     text: param.text,
                        //     x: 100,
                        //     y: 100,
                        //     align: engine::TextAlign::Left,
                        //     baseline: engine::TextBaseline::Top,
                        //     font,
                        //     paint: param.paint,
                        // })],
                    ]
                }],
                on_click: None,
                id: None,
            }]
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
// function drawText({
//   text,
//   font,
//   x,
//   y,
//   align,
//   baseline,
//   style,
// }: TextHandleParam): TextDrawCommand | undefined {
fn draw_text(param: TextParam, font: Arc<Font>) -> DrawCommand {
    let text_paint = engine::Paint::new();
    text_paint.set_color(&param.style.color);
    text_paint.set_style(&engine::PaintStyle::Fill);
    text_paint.set_anti_alias(true);

    DrawCommand::Text(TextDrawCommand {
        text: param.text,
        font,
        x: param.x,
        y: param.y,
        paint: text_paint,
        align: param.align,
        baseline: param.baseline,
    })
}
// function drawBorder({
//   style: { border },
//   text,
//   font,
//   x,
//   y,
//   align,
//   baseline,
// }: TextHandleParam): TextDrawCommand | undefined {
//   if (!border) {
//     return;
//   }
//   const borderPaint = new CanvasKit.Paint();
//   borderPaint.setColor(border.color);
//   borderPaint.setStyle(CanvasKit.PaintStyle.Stroke);
//   borderPaint.setStrokeWidth(border.width);
//   borderPaint.setStrokeJoin(CanvasKit.StrokeJoin.Miter);
//   borderPaint.setAntiAlias(true);

//   return TextDrawCommand({
//     text,
//     font,
//     x,
//     y,
//     paint: borderPaint,
//     align,
//     baseline,
//   });
// }

// function drawBackground({
//   x,
//   y,
//   align,
//   baseline,
//   text,
//   font,
//   style: { background, dropShadow },
// }: TextHandleParam): RenderingTree {
// fn draw_background(param: &TextParam, font: &Font) -> RenderingTree {
//     let style = &param.style;

//     let background = &style.background;
//     if background.is_none() {
//         return RenderingTree::Empty;
//     };
//     let background = background.as_ref().unwrap();

//     let width = get_text_width(
//         font,
//         param.text,
//         param.style.drop_shadow.map(|drop_shadow| drop_shadow.x),
//     );

//     let font_metrics = font.get_metrics();
//     let glyph_ids = font.get_glyph_ids(param.text);
//     let glyphs_top_bottom = get_glyphs_top_bottom(font, &glyph_ids);

//     let (height, top) = match glyphs_top_bottom {
//         Some((top, bottom)) => {
//             let height = bottom - top
//                 + if let Some(drop_shadow) = param.style.drop_shadow {
//                     drop_shadow.y
//                 } else {
//                     0.0
//                 };
//             let top = param.y + get_bottom_of_baseline(param.baseline, font_metrics) + top;
//             (height, top)
//         }
//         Option::None => (
//             -font_metrics.ascent + font_metrics.descent,
//             param.y + get_bottom_of_baseline(param.baseline, font_metrics) + font_metrics.ascent,
//         ),
//     };

//     let margin = background.margin.unwrap_or(LtrbRect::default());

//     let final_x = margin.left + get_left_in_align(param.x, param.align, width);
//     let final_y = margin.top + top;
//     let final_width = width + margin.left + margin.right;
//     let final_height = height + margin.top + margin.bottom;

//     // TODO
//     // rect(RectParam {
//     //     x: final_x,
//     //     y: final_y,
//     //     width: final_width,
//     //     height: final_height,
//     //     style: RectStyle {
//     //         fill: Some(RectFill {
//     //             color: background.color,
//     //         }),
//     //         ..Default::default()
//     //     },
//     //     ..Default::default()
//     // })
//     RenderingTree::Empty
// }

pub(crate) fn get_text_width(font: &Font, text: &str, drop_shadow_x: Option<f32>) -> f32 {
    let glyph_ids = font.get_glyph_ids(text);
    let glyph_widths = font.get_glyph_widths(&glyph_ids, Option::None);
    glyph_widths.iter().fold(0.0, |acc, cur| acc + cur) + drop_shadow_x.unwrap_or(0.0)
}

// // export function getGlyphsTopBottom(
// //   font: Font,
// //   glyphIds: ReturnType<Font["getGlyphIDs"]>,
// // ):
// //   | {
// //       glyphsTop: number;
// //       glyphsBottom: number;
// //     }
// //   | undefined {
// fn get_glyphs_top_bottom(font: &Font, glyph_ids: &GlyphIds) -> Option<(f32, f32)> {
//     if glyph_ids.len() == 0 {
//         return None;
//     }
//     let glyph_bounds = font.get_glyph_bounds(glyph_ids, Option::None);

//     let glyphs_top = glyph_bounds
//         .iter()
//         .reduce(|prev, cur| if prev.top < cur.top { prev } else { cur })
//         .unwrap()
//         .top;

//     let glyphs_bottom = glyph_bounds
//         .iter()
//         .reduce(|prev, cur| if prev.bottom > cur.bottom { prev } else { cur })
//         .unwrap()
//         .bottom;

//     Some((glyphs_top, glyphs_bottom))
// }
