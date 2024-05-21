use crate::*;

impl Draw for &TextDrawCommand {
    fn draw(self, skia: &mut impl SkSkia) {
        if self.text.is_empty() {
            return;
        }

        let group_glyph = skia.group_glyph(&self.font, &self.paint);

        let paragraph = Paragraph::new(&self.text, group_glyph.clone(), self.max_width);

        let line_height = line_height_px(self);

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
                    let font_metrics = skia.font_metrics(&font).unwrap();
                    let bottom = y + get_bottom_of_baseline(self.baseline, font_metrics);

                    let glyph_ids = glyphs.into_iter().map(|x| x.id).collect();

                    if let Some(underline_paint) = &self.underline {
                        skia.surface().canvas().draw_line(
                            Xy::new(x, bottom + 2.px()),
                            Xy::new(x + width, bottom + 2.px()),
                            underline_paint,
                        );
                    }

                    skia.surface().canvas().draw_text_blob(
                        glyph_ids,
                        Xy::new(x, bottom),
                        &font,
                        &self.paint,
                    );

                    x += width;
                }
            });
    }
}

fn line_height_px(command: &TextDrawCommand) -> Px {
    command.font.size.into_px() * command.line_height_percent
}
