use crate::*;
use namui_type::*;

impl BoundingBox for &DrawCommand {
    fn bounding_box(self) -> Option<Rect<Px>> {
        match self {
            DrawCommand::Path { command } => command.bounding_box(),
            DrawCommand::Text { command } => command.bounding_box(),
            DrawCommand::Image { command } => command.bounding_box(),
        }
    }
}

impl BoundingBox for &PathDrawCommand {
    fn bounding_box(self) -> Option<Rect<Px>> {
        NativePath::get(&self.path).bounding_box(Some(&self.paint))
    }
}
impl BoundingBox for &TextDrawCommand {
    fn bounding_box(self) -> Option<Rect<Px>> {
        if self.text.is_empty() {
            return None;
        }

        let paragraph = Paragraph::new(
            &self.text,
            self.font.clone(),
            self.paint.clone(),
            self.max_width,
        );

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
                self.font
                    .bounds(&line_text, &self.paint)
                    .iter()
                    .map(|bound| (bound.top(), bound.bottom()))
                    .reduce(|acc, (top, bottom)| (acc.0.min(top), acc.1.max(bottom)))
                    .map(|(top, bottom)| {
                        let widths = self.font.widths(&line_text, &self.paint);
                        let width = widths.iter().fold(px(0.0), |prev, curr| prev + curr);
                        let x_axis_anchor = get_left_in_align(self.x, self.align, width);

                        let metrics = self.font.font_metrics();
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
    fn bounding_box(self) -> Option<Rect<Px>> {
        match &self.paint {
            Some(paint) => {
                NativePath::get(&Path::new().add_rect(self.rect)).bounding_box(Some(paint))
            }
            _ => Some(self.rect),
        }
    }
}
