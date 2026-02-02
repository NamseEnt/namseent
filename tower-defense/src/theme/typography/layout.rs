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
        let mut max_font_line_height = 0.px();
        let mut positioned = Vec::new();

        for inline_box in boxes {
            let baseline = inline_box.baseline();
            let height = inline_box.height();
            let width = inline_box.width();

            max_ascent = max_ascent.max(baseline);
            max_descent = max_descent.max(height - baseline);

            // Calculate font-based line height for text boxes
            if let InlineBox::Text(shaped) = &inline_box {
                let font_size: Px = shaped.font.size.into();
                let font_line_height = font_size * self.config.line_height_percent;
                max_font_line_height = max_font_line_height.max(font_line_height);
            }

            positioned.push(PositionedInlineBox {
                inline_box,
                x,
                y: 0.px(),
                text_baseline: TextBaseline::Top,
            });

            x += width;
        }

        // Line height is the maximum of content height and font line height
        let content_height = (max_ascent + max_descent) * self.config.line_height_percent;
        let line_height = content_height.max(max_font_line_height);

        // Apply vertical alignment
        for positioned_box in &mut positioned {
            let vertical_align = positioned_box.inline_box.vertical_align();

            match &positioned_box.inline_box {
                InlineBox::Text(_) => {
                    // For text boxes, use TextBaseline and adjust y accordingly
                    match vertical_align {
                        super::style::VerticalAlign::Top => {
                            positioned_box.y = 0.px();
                            positioned_box.text_baseline = TextBaseline::Top;
                        }
                        super::style::VerticalAlign::Middle => {
                            positioned_box.y = line_height / 2.0;
                            positioned_box.text_baseline = TextBaseline::Middle;
                        }
                        super::style::VerticalAlign::Bottom => {
                            positioned_box.y = line_height;
                            positioned_box.text_baseline = TextBaseline::Bottom;
                        }
                    }
                }
                InlineBox::Atomic { height, .. } => {
                    // For atomic boxes, calculate y position based on box height
                    match vertical_align {
                        super::style::VerticalAlign::Top => {
                            positioned_box.y = 0.px();
                        }
                        super::style::VerticalAlign::Middle => {
                            positioned_box.y = (line_height - *height) / 2.0;
                        }
                        super::style::VerticalAlign::Bottom => {
                            positioned_box.y = line_height - *height;
                        }
                    }
                }
                _ => {
                    // For other boxes (breaks, spaces), use top alignment
                    positioned_box.y = 0.px();
                }
            }
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
        let mut box_queue: Vec<InlineBox> = boxes;

        while let Some(inline_box) = box_queue.first().cloned() {
            box_queue.remove(0);

            if inline_box.is_hard_break() {
                if !current_line.is_empty() {
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    current_width = 0.px();
                }
                continue;
            }

            let box_width = inline_box.width();

            if current_width + box_width <= max_width {
                // Box fits in current line
                current_width += box_width;
                current_line.push(inline_box);
            } else if current_width == 0.px() {
                // Current line is empty - try to split the box
                if let Some((fits, remaining)) = inline_box.split_to_fit(max_width) {
                    current_line.push(fits);
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    current_width = 0.px();
                    box_queue.insert(0, remaining);
                } else {
                    // Can't split single box - force it
                    current_line.push(inline_box);
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    current_width = 0.px();
                }
            } else {
                // Current line is not empty and box doesn't fit
                // Try to find a break opportunity
                if let Some(break_idx) = self.find_last_break_opportunity(&current_line) {
                    // Found break opportunity
                    let remaining: Vec<_> = current_line.drain(break_idx..).collect();

                    if !current_line.is_empty() {
                        lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    }

                    // Re-add removed boxes and current box to queue
                    box_queue.insert(0, inline_box);
                    for remaining_box in remaining.into_iter().rev() {
                        box_queue.insert(0, remaining_box);
                    }
                    current_width = 0.px();
                } else {
                    // No break opportunity - move current line to output
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line)));
                    box_queue.insert(0, inline_box);
                    current_width = 0.px();
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
