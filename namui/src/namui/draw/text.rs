use std::borrow::Borrow;

use crate::namui::{self, skia::*, NamuiContext};

use super::{TextAlign, TextBaseline, TextDrawCommand};

pub fn draw_text(namui_context: &NamuiContext, command: &TextDrawCommand) {
    if command.text.len() == 0 {
        return;
    }

    let font = &command.font;

    let glyph_ids = font.get_glyph_ids(&command.text);

    let paint = command.paint_builder.build();

    let widths = font.get_glyph_widths(&glyph_ids, Option::Some(&paint));

    let width = widths.iter().fold(0.0, |prev, curr| prev + curr);

    let metrics = font.get_metrics();

    let bottom = command.y as f32 + get_bottom_of_baseline(&command.baseline, &metrics);

    let left = get_left_in_align(command.x as f32, command.align, width);

    let text_blob = TextBlob::from_text(&command.text, font.borrow());

    namui_context
        .surface
        .canvas()
        .draw_text_blob(&text_blob, left, bottom, &paint);
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
