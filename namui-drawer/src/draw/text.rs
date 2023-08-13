use crate::*;

impl Draw for TextDrawCommand {
    fn draw(self, ctx: &DrawContext) {
        if self.text.len() == 0 {
            return;
        }

        let group_glyph = ctx.skia.group_glyph(&self.font, &self.paint);

        let paragraph = Paragraph::new(&self.text, group_glyph.clone(), self.max_width);

        let line_height = line_height_px(&self);

        let multiline_y_baseline_offset =
            get_multiline_y_baseline_offset(self.baseline, line_height, paragraph.line_len());

        paragraph
            .iter_str()
            .enumerate()
            .map(|(index, line)| {
                (
                    self.y + multiline_y_baseline_offset + line_height * index,
                    line,
                )
            })
            .for_each(|(y, line)| {
                let glyph_groups = group_glyph.groups(&line);

                let total_width = glyph_groups.iter().map(|group| group.width).sum();

                let left = get_left_in_align(self.x, self.align, total_width);

                let mut x = left;

                for GlyphGroup {
                    glyphs,
                    font,
                    width,
                } in glyph_groups
                {
                    let font_metrics = ctx.skia.font_metrics(&font).unwrap();
                    let bottom = y + get_bottom_of_baseline(self.baseline, font_metrics);

                    let glyph_ids = glyphs.into_iter().map(|x| x.id).collect();

                    if let Some(underline_paint) = &self.underline {
                        ctx.canvas().draw_line(
                            Xy::new(x, bottom + 2.px()),
                            Xy::new(x + width, bottom + 2.px()),
                            &underline_paint,
                        );
                    }

                    ctx.canvas()
                        .draw_text_blob(glyph_ids, Xy::new(x, bottom), &font, &self.paint);

                    x += width;
                }
            });
    }
}

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// pub struct TextDrawCommand {
//     pub text: String,
//     pub font: Font,
//     pub x: Px,
//     pub y: Px,
//     pub paint: Paint,
//     pub align: TextAlign,
//     pub baseline: TextBaseline,
//     pub max_width: Option<Px>,
//     pub line_height_percent: Percent,
//     pub underline: Option<Paint>,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize, Copy, Clone, PartialEq)]
// pub enum TextAlign {
//     Left,
//     Center,
//     Right,
// }
// #[derive(Debug, serde::Serialize, serde::Deserialize, Copy, Clone, PartialEq)]
// pub enum TextBaseline {
//     Top,
//     Middle,
//     Bottom,
// }

// impl TextDrawCommand {
//     pub fn draw(&self) {

//     }
//     pub fn get_bounding_box(&self) -> Option<Rect<Px>> {
//         if self.text.len() == 0 {
//             return None;
//         }

//         let fonts = crate::font::with_fallbacks(self.font.clone());

//         let paint = self.paint.build();

//         let paragraph = Paragraph::new(&self.text, fonts, paint.clone(), self.max_width);

//         let line_height = self.line_height_px();

//         let multiline_y_baseline_offset =
//             get_multiline_y_baseline_offset(self.baseline, line_height, paragraph.line_len());

//         let font = &self.font;

//         paragraph
//             .iter_str()
//             .enumerate()
//             .map(|(index, line)| {
//                 (
//                     self.y + multiline_y_baseline_offset + line_height * index,
//                     line,
//                 )
//             })
//             .map(|(y, line)| {
//                 let glyph_ids = font.get_glyph_ids(line);

//                 let paint = self.paint.build();
//                 let glyph_bounds = font.get_glyph_bounds(glyph_ids.clone(), paint.as_ref());

//                 glyph_bounds
//                     .iter()
//                     .map(|bound| (bound.top(), bound.bottom()))
//                     .reduce(|acc, (top, bottom)| (acc.0.min(top), acc.1.max(bottom)))
//                     .and_then(|(top, bottom)| {
//                         let widths = font.get_glyph_widths(glyph_ids, paint.as_ref());
//                         let width = widths.iter().fold(px(0.0), |prev, curr| prev + curr);
//                         let x_axis_anchor = get_left_in_align(self.x, self.align, width);

//                         let metrics = font.metrics;
//                         let y_axis_anchor = y + get_bottom_of_baseline(self.baseline, metrics);

//                         Some(Rect::Ltrb {
//                             left: x_axis_anchor,
//                             top: top + y_axis_anchor,
//                             right: x_axis_anchor + width,
//                             bottom: bottom + y_axis_anchor,
//                         })
//                     })
//             })
//             .fold(None, |acc, rect| {
//                 if let Some(rect) = rect {
//                     if let Some(acc) = acc {
//                         Some(Rect::Ltrb {
//                             left: acc.left().min(rect.left()),
//                             top: acc.top().min(rect.top()),
//                             right: acc.right().max(rect.right()),
//                             bottom: acc.bottom().max(rect.bottom()),
//                         })
//                     } else {
//                         Some(rect)
//                     }
//                 } else {
//                     acc
//                 }
//             })
//     }

//     pub(crate) fn is_xy_in(&self, xy: Xy<Px>) -> bool {
//         self.get_bounding_box()
//             .map(|bound| bound.is_xy_inside(xy))
//             .unwrap_or(false)
//     }

//     fn line_height_px(&self) -> Px {
//         self.font.size.into_px() * self.line_height_percent
//     }
// }

fn line_height_px(command: &TextDrawCommand) -> Px {
    command.font.size.into_px() * command.line_height_percent
}
