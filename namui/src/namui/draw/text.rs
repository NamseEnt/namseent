use super::*;
use crate::{namui::skia::*, system::graphics, *};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

#[derive(Serialize, Clone, Debug)]
pub struct TextDrawCommand {
    pub text: String,
    #[serde(skip_serializing)]
    pub font: Arc<Font>,
    pub x: Px,
    pub y: Px,
    #[serde(skip_serializing)]
    pub paint_builder: PaintBuilder,
    pub align: TextAlign,
    pub baseline: TextBaseline,
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

        let line_texts = self.text.split("\n").collect::<Vec<_>>();

        let line_height = (self.font.size as f32).px() * 110.percent();

        let paint = self.paint_builder.build();

        let fonts = std::iter::once(self.font.clone())
            .chain(std::iter::once_with(|| get_fallback_fonts(self.font.size)).flatten())
            .collect::<Vec<_>>();

        let mut bottom_of_fonts: HashMap<String, Px> = HashMap::new();

        let multiline_y_baseline_offset = match self.baseline {
            TextBaseline::Top => 0.px(),
            TextBaseline::Middle => -line_height * 0.5 * (line_texts.len() - 1),
            TextBaseline::Bottom => -line_height * (line_texts.len() - 1),
        };

        line_texts
            .iter()
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

                    let text_blob = TextBlob::from_glyph_ids(&glyph_ids, &font);

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

        let line_texts = self.text.split("\n").collect::<Vec<_>>();

        let line_height = (self.font.size as f32).px() * 110.percent();

        let multiline_y_baseline_offset = match self.baseline {
            TextBaseline::Top => 0.px(),
            TextBaseline::Middle => -line_height * 0.5 * (line_texts.len() - 1),
            TextBaseline::Bottom => -line_height * (line_texts.len() - 1),
        };

        let font = &self.font;

        line_texts
            .iter()
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
}

#[derive(Debug)]
struct GlyphGroup {
    glyph_ids: Vec<u16>,
    end_index: usize,
    width: Px,
    font: Arc<Font>,
}
fn get_glyph_groups(text: &str, fonts: &Vec<Arc<Font>>, paint: &Arc<Paint>) -> Vec<GlyphGroup> {
    let mut groups: Vec<GlyphGroup> = vec![];
    let mut non_calculated_char_and_indexes: Vec<(char, usize)> = text
        .chars()
        .enumerate()
        .map(|(index, char)| (char, index))
        .collect();
    let mut fonts = fonts.iter().peekable();

    while !non_calculated_char_and_indexes.is_empty() && fonts.peek().is_some() {
        let font = fonts.next().unwrap();

        let text = non_calculated_char_and_indexes
            .iter()
            .map(|(char, _)| char)
            .collect::<String>();

        let glyph_ids = font.get_glyph_ids(&text);

        let mut available_glyph_id_and_indexes = vec![];
        for (index, glyph_id) in glyph_ids.iter().enumerate() {
            if *glyph_id != 0 {
                available_glyph_id_and_indexes.push((*glyph_id, index));
                non_calculated_char_and_indexes.retain(|(_, index2)| index != *index2);
            }
        }

        if available_glyph_id_and_indexes.is_empty() {
            continue;
        }

        let available_glyph_id_and_index_and_width: Vec<(u16, usize, Px)> = {
            let available_glyph_ids: Vec<_> = available_glyph_id_and_indexes
                .iter()
                .map(|(glyph_id, _)| *glyph_id)
                .collect();

            let widths = font.get_glyph_widths(available_glyph_ids.into(), Option::Some(paint));

            widths
                .into_iter()
                .zip(available_glyph_id_and_indexes.into_iter())
                .map(|(width, (glyph_id, index))| (glyph_id, index, width))
                .collect()
        };

        for (glyph_id, index, width) in available_glyph_id_and_index_and_width {
            if let Some(last_group) = groups.last_mut() {
                if last_group.end_index + 1 == index {
                    last_group.glyph_ids.push(glyph_id);
                    last_group.width += width;
                    last_group.end_index = index;
                    continue;
                }
            }
            groups.push(GlyphGroup {
                glyph_ids: vec![glyph_id],
                end_index: index,
                width,
                font: font.clone(),
            });
        }
    }
    groups.sort_by(|a, b| a.end_index.cmp(&b.end_index));
    groups
}
pub fn get_left_in_align(x: Px, align: TextAlign, width: Px) -> Px {
    match align {
        TextAlign::Left => x,
        TextAlign::Right => x - width,
        TextAlign::Center => x - width / 2.0,
    }
}
pub fn get_bottom_of_baseline(baseline: TextBaseline, font_metrics: FontMetrics) -> Px {
    match baseline {
        TextBaseline::Top => -font_metrics.ascent,
        TextBaseline::Bottom => -font_metrics.descent,
        TextBaseline::Middle => (-font_metrics.ascent - font_metrics.descent) / 2.0,
    }
}
fn get_fallback_fonts(font_size: i16) -> VecDeque<Arc<Font>> {
    crate::typeface::get_fallback_font_typefaces()
        .map(|typeface| crate::font::get_font_of_typeface(typeface.clone(), font_size))
        .collect()
}
