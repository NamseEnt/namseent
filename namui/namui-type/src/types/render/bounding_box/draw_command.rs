use crate::*;

impl BoundingBox for &DrawCommand {
    fn bounding_box(self, calculator: &dyn SkCalculate) -> Option<Rect<Px>> {
        match self {
            DrawCommand::Path { command } => command.bounding_box(calculator),
            DrawCommand::Text { command } => command.bounding_box(calculator),
            DrawCommand::Image { command } => command.bounding_box(calculator),
        }
    }
}

impl BoundingBox for &PathDrawCommand {
    fn bounding_box(self, calculator: &dyn SkCalculate) -> Option<Rect<Px>> {
        calculator.path_bounding_box(&self.path, Some(&self.paint))
    }
}
impl BoundingBox for &TextDrawCommand {
    fn bounding_box(self, calculator: &dyn SkCalculate) -> Option<Rect<Px>> {
        if self.text.is_empty() {
            return None;
        }

        let group_glyph = calculator.group_glyph(&self.font, &self.paint);
        let paragraph = Paragraph::new(&self.text, group_glyph.clone(), self.max_width);

        let line_height = self.line_height_px();

        let multiline_y_baseline_offset =
            get_multiline_y_baseline_offset(self.baseline, line_height, paragraph.line_len());

        paragraph
            .iter_str()
            .enumerate()
            .map(|(index, line_text)| {
                (
                    self.y + multiline_y_baseline_offset + line_height * index,
                    line_text,
                )
            })
            .map(|(y, line_text)| {
                group_glyph
                    .bounds(&line_text)
                    .iter()
                    .map(|bound| (bound.top(), bound.bottom()))
                    .reduce(|acc, (top, bottom)| (acc.0.min(top), acc.1.max(bottom)))
                    .map(|(top, bottom)| {
                        let widths = group_glyph.widths(&self.text);
                        let width = widths.iter().fold(px(0.0), |prev, curr| prev + curr);
                        let x_axis_anchor = get_left_in_align(self.x, self.align, width);

                        let metrics = group_glyph.font_metrics();
                        let y_axis_anchor = y + get_bottom_of_baseline(self.baseline, metrics);

                        Rect::Ltrb {
                            left: x_axis_anchor,
                            top: top + y_axis_anchor,
                            right: x_axis_anchor + width,
                            bottom: bottom + y_axis_anchor,
                        }
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
}

impl BoundingBox for &ImageDrawCommand {
    fn bounding_box(self, calculator: &dyn SkCalculate) -> Option<Rect<Px>> {
        match &self.paint {
            Some(paint) => {
                calculator.path_bounding_box(&Path::new().add_rect(self.rect), Some(paint))
            }
            _ => Some(self.rect),
        }
    }
}
