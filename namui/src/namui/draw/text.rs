use super::{TextAlign, TextBaseline, TextDrawCommand};
use crate::namui::{skia::*, NamuiContext};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

pub fn draw_text(namui_context: &NamuiContext, command: &TextDrawCommand) {
    if command.text.len() == 0 {
        return;
    }

    let paint = command.paint_builder.build();

    let fonts = std::iter::once(command.font.clone()).chain(
        std::iter::once_with(|| get_fallback_fonts(namui_context, command.font.size)).flatten(),
    );

    let glyph_groups = get_glyph_groups(&command.text, fonts, &paint);

    let total_width = glyph_groups.iter().map(|group| group.width).sum();

    let left = get_left_in_align(command.x as f32, command.align, total_width);

    let mut bottom_of_fonts: HashMap<String, f32> = HashMap::new();

    let mut x = left;

    for GlyphGroup {
        glyph_ids,
        width,
        end_index: _,
        font,
    } in glyph_groups
    {
        let bottom = bottom_of_fonts
            .get(&font.id)
            .map(|bottom| bottom + font.size as f32)
            .unwrap_or_else(|| {
                let metrics = font.get_metrics();
                let bottom = command.y as f32 + get_bottom_of_baseline(&command.baseline, &metrics);
                bottom_of_fonts.insert(font.id.clone(), bottom);
                bottom
            });

        let text_blob = TextBlob::from_glyph_ids(&glyph_ids, &font);

        namui_context
            .surface
            .canvas()
            .draw_text_blob(&text_blob, x, bottom, &paint);

        x += width;
    }
}

#[derive(Debug)]
struct GlyphGroup {
    glyph_ids: Vec<u16>,
    end_index: usize,
    width: f32,
    font: Arc<Font>,
}
fn get_glyph_groups(
    text: &str,
    fonts: impl Iterator<Item = Arc<Font>>,
    paint: &Arc<Paint>,
) -> Vec<GlyphGroup> {
    let mut groups: Vec<GlyphGroup> = vec![];
    let mut non_calculated_char_and_indexes: Vec<(char, usize)> = text
        .chars()
        .enumerate()
        .map(|(index, char)| (char, index))
        .collect();
    let mut fonts = fonts.peekable();

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

        let available_glyph_id_and_index_and_width: Vec<(u16, usize, f32)> = {
            let available_glyph_ids: Vec<_> = available_glyph_id_and_indexes
                .iter()
                .map(|(glyph_id, _)| *glyph_id)
                .collect();

            let widths = font.get_glyph_widths(&available_glyph_ids, Option::Some(paint));

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
pub fn get_left_in_align(x: f32, align: TextAlign, width: f32) -> f32 {
    match align {
        TextAlign::Left => x,
        TextAlign::Right => x - width,
        TextAlign::Center => x - width / 2.0,
    }
}
pub fn get_bottom_of_baseline(baseline: &TextBaseline, font_metrics: &FontMetrics) -> f32 {
    match baseline {
        TextBaseline::Top => -font_metrics.ascent,
        TextBaseline::Bottom => -font_metrics.descent,
        TextBaseline::Middle => (-font_metrics.ascent - font_metrics.descent) / 2.0,
    }
}
fn get_fallback_fonts(namui_context: &NamuiContext, font_size: i16) -> VecDeque<Arc<Font>> {
    let mut managers = crate::managers();
    namui_context
        .fallback_font_typefaces
        .iter()
        .map(|typeface| {
            managers
                .font_manager
                .get_font_of_typeface(typeface.clone(), font_size)
        })
        .collect()
}
impl TextDrawCommand {
    pub fn get_bounding_box(&self) -> Option<LtrbRect> {
        if self.text.len() == 0 {
            return None;
        }

        let font = &self.font;

        let glyph_ids = font.get_glyph_ids(&self.text);

        let paint = self.paint_builder.build();
        let glyph_bounds = font.get_glyph_bounds(&glyph_ids, Some(&paint));

        glyph_bounds
            .iter()
            .map(|bound| (bound.top, bound.bottom))
            .reduce(|acc, (top, bottom)| (acc.0.min(top), acc.1.max(bottom)))
            .and_then(|(top, bottom)| {
                let widths = font.get_glyph_widths(&glyph_ids, Option::Some(&paint));
                let width = widths.iter().fold(0.0, |prev, curr| prev + curr);
                let x_axis_anchor = get_left_in_align(self.x as f32, self.align, width);

                let metrics = font.get_metrics();
                let y_axis_anchor =
                    self.y as f32 + get_bottom_of_baseline(&self.baseline, &metrics);

                Some(LtrbRect {
                    left: x_axis_anchor,
                    top: top + y_axis_anchor,
                    right: x_axis_anchor + width,
                    bottom: bottom + y_axis_anchor,
                })
            })
    }
}
