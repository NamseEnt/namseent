use super::*;
use crate::{namui::skia::*, system::graphics, text::*, *};
use std::{collections::HashMap, sync::Arc};

#[derive(Serialize, Clone, Debug)]
pub struct TextDrawCommand {
    pub text: String,
    pub font: Arc<Font>,
    pub x: Px,
    pub y: Px,
    pub paint_builder: PaintBuilder,
    pub align: TextAlign,
    pub baseline: TextBaseline,
    pub max_width: Option<Px>,
    pub line_height_percent: Percent,
}

#[derive(Debug, Serialize, Copy, Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
#[derive(Debug, Serialize, Copy, Clone)]
pub enum TextBaseline {
    Top,
    Middle,
    Bottom,
}

impl TextDrawCommand {
    pub fn draw(&self) {
        if self.text.len() == 0 {
            return;
        }

        let fonts = std::iter::once(self.font.clone())
            .chain(std::iter::once_with(|| get_fallback_fonts(self.font.size)).flatten())
            .collect::<Vec<_>>();

        let paint = self.paint_builder.build();

        let line_texts = LineTexts::new(&self.text, &fonts, &paint, self.max_width);

        let line_height = self.line_height_px();

        let mut bottom_of_fonts: HashMap<String, Px> = HashMap::new();

        let multiline_y_baseline_offset =
            get_multiline_y_baseline_offset(self.baseline, line_height, line_texts.line_len());

        line_texts
            .iter_str()
            .enumerate()
            .map(|(index, line_text)| {
                (
                    self.y + multiline_y_baseline_offset + line_height * index,
                    line_text,
                )
            })
            .for_each(|(y, line_text)| {
                let glyph_groups = get_glyph_groups(&line_text, &fonts, &paint);

                let total_width = glyph_groups.iter().map(|group| group.width).sum();

                let left = get_left_in_align(self.x, self.align, total_width);

                let mut x = left;

                for GlyphGroup {
                    glyph_ids,
                    width,
                    end_index: _,
                    font,
                } in glyph_groups
                {
                    let bottom = y + bottom_of_fonts
                        .get(&font.id)
                        .map(|bottom| *bottom)
                        .unwrap_or_else(|| {
                            let metrics = font.metrics;
                            let bottom = get_bottom_of_baseline(self.baseline, metrics);
                            bottom_of_fonts.insert(font.id.clone(), bottom);
                            bottom
                        });

                    let text_blob = TextBlob::from_glyph_ids(glyph_ids.into_boxed_slice(), &font);

                    graphics::surface()
                        .canvas()
                        .draw_text_blob(&text_blob, x, bottom, &paint);

                    x += width;
                }
            });
    }
    pub fn get_bounding_box(&self) -> Option<Rect<Px>> {
        if self.text.len() == 0 {
            return None;
        }

        let fonts = std::iter::once(self.font.clone())
            .chain(std::iter::once_with(|| get_fallback_fonts(self.font.size)).flatten())
            .collect::<Vec<_>>();

        let paint = self.paint_builder.build();

        let line_texts = LineTexts::new(&self.text, &fonts, &paint, self.max_width);

        let line_height = self.line_height_px();

        let multiline_y_baseline_offset =
            get_multiline_y_baseline_offset(self.baseline, line_height, line_texts.line_len());

        let font = &self.font;

        line_texts
            .iter_str()
            .enumerate()
            .map(|(index, line_text)| {
                (
                    self.y + multiline_y_baseline_offset + line_height * index,
                    line_text,
                )
            })
            .map(|(y, line_text)| {
                let glyph_ids = font.get_glyph_ids(line_text);

                let paint = self.paint_builder.build();
                let glyph_bounds = font.get_glyph_bounds(glyph_ids.clone(), Some(&paint));

                glyph_bounds
                    .iter()
                    .map(|bound| (bound.top(), bound.bottom()))
                    .reduce(|acc, (top, bottom)| (acc.0.min(top), acc.1.max(bottom)))
                    .and_then(|(top, bottom)| {
                        let widths = font.get_glyph_widths(glyph_ids, Option::Some(&paint));
                        let width = widths.iter().fold(px(0.0), |prev, curr| prev + curr);
                        let x_axis_anchor = get_left_in_align(self.x, self.align, width);

                        let metrics = font.metrics;
                        let y_axis_anchor = y + get_bottom_of_baseline(self.baseline, metrics);

                        Some(Rect::Ltrb {
                            left: x_axis_anchor,
                            top: top + y_axis_anchor,
                            right: x_axis_anchor + width,
                            bottom: bottom + y_axis_anchor,
                        })
                    })
            })
            .fold(None, |acc, rect| {
                if let Some(rect) = rect {
                    if let Some(acc) = acc {
                        Some(Rect::Ltrb {
                            left: acc.left().min(rect.left()),
                            top: acc.top().min(rect.top()),
                            right: acc.right().max(rect.right()),
                            bottom: acc.bottom().max(rect.bottom()),
                        })
                    } else {
                        Some(rect)
                    }
                } else {
                    acc
                }
            })
    }

    pub(crate) fn is_xy_in(&self, xy: Xy<Px>) -> bool {
        self.get_bounding_box()
            .map(|bound| bound.is_xy_inside(xy))
            .unwrap_or(false)
    }

    fn line_height_px(&self) -> Px {
        self.font.size.into_px() * self.line_height_percent
    }
}
