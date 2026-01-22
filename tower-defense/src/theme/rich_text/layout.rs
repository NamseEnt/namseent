use super::inline_box::{InlineBox, LineBox, PositionedInlineBox};
use namui::*;

/// Layout configuration
#[derive(Debug, Clone, Copy)]
pub struct LayoutConfig {
    pub max_width: Option<Px>,
    pub text_align: TextAlign,
    pub line_height_percent: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            max_width: None,
            text_align: TextAlign::Left,
            line_height_percent: 1.3,
        }
    }
}

/// Layout engine for arranging inline boxes
pub struct LayoutEngine {
    config: LayoutConfig,
}

impl LayoutEngine {
    pub fn new(config: LayoutConfig) -> Self {
        Self { config }
    }

    /// Layout inline boxes into lines
    pub fn layout(&self, boxes: Vec<InlineBox>) -> Vec<LineBox> {
        if boxes.is_empty() {
            return vec![];
        }

        match self.config.max_width {
            Some(max_width) => self.layout_with_wrapping(boxes, max_width),
            None => vec![self.layout_single_line(boxes)],
        }
    }

    fn layout_single_line(&self, boxes: Vec<InlineBox>) -> LineBox {
        let mut x = 0.px();
        let mut max_ascent = 0.px();
        let mut max_descent = 0.px();
        let mut positioned = Vec::new();

        for inline_box in boxes {
            let baseline = inline_box.baseline();
            let height = inline_box.height();
            let width = inline_box.width();

            max_ascent = max_ascent.max(baseline);
            max_descent = max_descent.max(height - baseline);

            positioned.push(PositionedInlineBox {
                inline_box,
                x,
                y: 0.px(),
            });

            x += width;
        }

        let line_height = (max_ascent + max_descent) * self.config.line_height_percent;

        // Apply baseline alignment
        for positioned_box in &mut positioned {
            let box_baseline = positioned_box.inline_box.baseline();
            positioned_box.y = max_ascent - box_baseline;
        }

        LineBox {
            boxes: positioned,
            content_width: x,
            height: line_height,
            baseline: max_ascent,
        }
    }

    fn layout_with_wrapping(&self, boxes: Vec<InlineBox>, max_width: Px) -> Vec<LineBox> {
        let mut lines = Vec::new();
        let mut current_line = Vec::new();
        let mut current_width = 0.px();

        for inline_box in boxes {
            if inline_box.is_hard_break() {
                if !current_line.is_empty() {
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    current_width = 0.px();
                }
                continue;
            }

            let box_width = inline_box.width();

            if current_width + box_width <= max_width {
                current_width += box_width;
                current_line.push(inline_box);
            } else {
                // Try to find a break opportunity
                if let Some(break_idx) = self.find_last_break_opportunity(&current_line) {
                    let remaining: Vec<_> = current_line.drain(break_idx..).collect();

                    if !current_line.is_empty() {
                        lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    }

                    // Re-add remaining boxes
                    for remaining_box in remaining.into_iter().rev() {
                        current_line.insert(0, remaining_box);
                    }
                    current_line.push(inline_box);
                    current_width = current_line.iter().map(|b| b.width()).sum();
                } else if current_line.is_empty() {
                    // Single box wider than max_width - force it
                    current_line.push(inline_box);
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    current_width = 0.px();
                } else {
                    // Move box to next line
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    current_line.push(inline_box);
                    current_width = box_width;
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(self.layout_single_line(current_line));
        }

        lines
    }

    fn find_last_break_opportunity(&self, boxes: &[InlineBox]) -> Option<usize> {
        boxes
            .iter()
            .enumerate()
            .rev()
            .find(|(_, b)| b.is_break_opportunity())
            .map(|(i, _)| i)
    }
}
