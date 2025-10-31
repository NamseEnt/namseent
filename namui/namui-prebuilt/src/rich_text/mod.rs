mod parse;
#[cfg(test)]
mod tests;

use namui::*;
pub use parse::*;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum VerticalAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

pub enum Tag {
    Image { param: ImageParam },
    StyledText { font: Font, style: TextStyle },
    RenderingTree { rendering_tree: RenderingTree },
}

pub struct RegexHandler {
    pub regex: Regex,
    pub handler: Box<dyn Fn(&str) -> RenderingTree>,
}

impl RegexHandler {
    /// Create a new RegexHandler with a regex pattern
    pub fn new(
        pattern: &str,
        handler: Box<dyn Fn(&str) -> RenderingTree>,
    ) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(Self { regex, handler })
    }

    /// Find the first match in the text
    pub fn find_match(&self, text: &str) -> Option<(usize, usize)> {
        self.regex.find(text).map(|m| (m.start(), m.end()))
    }

    /// Get the matched text
    pub fn get_match<'a>(&self, text: &'a str) -> Option<&'a str> {
        self.regex.find(text).map(|m| m.as_str())
    }
}

/// Example usage documentation
impl<'a> RichText<'a> {
    /// Create a new RichText with default settings
    pub fn new(
        text: String,
        max_width: Option<Px>,
        default_font: Font,
        default_text_style: TextStyle,
        tag_map: &'a HashMap<String, Tag>,
    ) -> Self {
        Self {
            text,
            max_width,
            default_font,
            default_text_style,
            default_text_align: TextAlign::Left,
            default_vertical_align: VerticalAlign::default(),
            tag_map,
            regex_handlers: &[],
            on_parse_error: None,
        }
    }

    /// Create a new RichText with regex handlers
    #[allow(clippy::too_many_arguments)]
    pub fn with_regex_handlers(
        text: String,
        max_width: Option<Px>,
        default_font: Font,
        default_text_style: TextStyle,
        default_text_align: TextAlign,
        tag_map: &'a HashMap<String, Tag>,
        regex_handlers: &'a [RegexHandler],
    ) -> Self {
        Self {
            text,
            max_width,
            default_font,
            default_text_style,
            default_text_align,
            default_vertical_align: VerticalAlign::default(),
            tag_map,
            regex_handlers,
            on_parse_error: None,
        }
    }
}

pub struct RichText<'a> {
    pub text: String,
    pub max_width: Option<Px>,
    pub default_font: Font,
    pub default_text_style: TextStyle,
    pub default_text_align: TextAlign,
    pub default_vertical_align: VerticalAlign,
    pub tag_map: &'a HashMap<String, Tag>,
    pub regex_handlers: &'a [RegexHandler],
    pub on_parse_error: Option<&'a dyn Fn(ParseError)>,
}

impl Component for RichText<'_> {
    fn render(self, ctx: &RenderCtx) {
        let text = ctx.track_eq(&self.text);
        let tokens = ctx.memo(move || {
            let text = text.as_str();
            parse::parse(text).unwrap_or_else(|err| {
                if let Some(on_parse_error) = &self.on_parse_error {
                    on_parse_error(err);
                }
                vec![]
            })
        });

        // Check for invalid text alignment without max_width and warn user
        let effective_text_align = if self.max_width.is_none()
            && (self.default_text_align == TextAlign::Center
                || self.default_text_align == TextAlign::Right)
        {
            eprintln!(
                "Warning: RichText with text_align {:?} requires max_width to be set. Falling back to Left alignment.",
                self.default_text_align
            );
            TextAlign::Left
        } else {
            self.default_text_align
        };

        let mut processor = Processor::new(
            self.max_width,
            self.regex_handlers,
            self.default_vertical_align,
        );
        processor.current_text_align = effective_text_align;

        for token in tokens.iter() {
            match token {
                Token::DefaultText { text } => {
                    processor.process_text(
                        ctx,
                        text,
                        self.default_font.clone(),
                        self.default_text_style.clone(),
                        effective_text_align,
                    );
                }
                Token::Image { tag } => {
                    let Some(tag) = self.tag_map.get(tag) else {
                        continue;
                    };
                    let Tag::Image { param } = tag else {
                        continue;
                    };

                    processor.add(ctx, namui::image(param.clone()));
                }
                Token::StyledText { tag, text } => {
                    let Some(tag) = self.tag_map.get(tag) else {
                        continue;
                    };
                    let Tag::StyledText { font, style } = tag else {
                        continue;
                    };

                    processor.process_text(
                        ctx,
                        text,
                        font.clone(),
                        style.clone(),
                        effective_text_align,
                    );
                }
                Token::RenderingTree { tag } => {
                    let Some(tag) = self.tag_map.get(tag) else {
                        continue;
                    };
                    let Tag::RenderingTree { rendering_tree } = tag else {
                        continue;
                    };

                    processor.add(ctx, rendering_tree.clone());
                }
            };
        }

        // Flush any remaining items in the current line
        processor.finish(ctx);
    }
}

pub(crate) struct Processor<'a> {
    max_width: Option<Px>,
    cursor_x: Px,
    cursor_y: Px,
    line_height: Px,
    is_first_in_line: bool,
    regex_handlers: &'a [RegexHandler],
    current_line_items: Vec<LineItem>,
    current_text_align: TextAlign,
    default_vertical_align: VerticalAlign,
}

struct LineItem {
    rendering_tree: RenderingTree,
    width: Px,
    height: Px,
}

struct TextProcessParams {
    font: Font,
    style: TextStyle,
    text_align: TextAlign,
}

impl<'a> Processor<'a> {
    fn new(
        max_width: Option<Px>,
        regex_handlers: &'a [RegexHandler],
        default_vertical_align: VerticalAlign,
    ) -> Self {
        Self {
            max_width,
            cursor_x: 0.px(),
            cursor_y: 0.px(),
            line_height: 0.px(),
            is_first_in_line: true,
            regex_handlers,
            current_line_items: Vec::new(),
            current_text_align: TextAlign::Left,
            default_vertical_align,
        }
    }

    fn add(&mut self, ctx: &RenderCtx, rendering_tree: RenderingTree) {
        let Some(bounding_box) = rendering_tree.bounding_box() else {
            return;
        };

        let item_width = bounding_box.right();

        // Check if we need to break the line before adding this item
        if !self.is_first_in_line
            && let Some(max_width) = self.max_width
            && self.cursor_x + item_width > max_width
        {
            self.flush_current_line(ctx);
            self.break_line();
        }

        // Add item to current line
        self.current_line_items.push(LineItem {
            rendering_tree,
            width: item_width,
            height: bounding_box.height(),
        });

        self.line_height = self.line_height.max(bounding_box.height());
        self.cursor_x += item_width;
        self.is_first_in_line = false;
    }

    fn flush_current_line(&mut self, ctx: &RenderCtx) {
        if self.current_line_items.is_empty() {
            return;
        }

        let total_width: Px = self.current_line_items.iter().map(|item| item.width).sum();

        let start_x = match self.current_text_align {
            TextAlign::Left => 0.px(),
            TextAlign::Center => {
                (self
                    .max_width
                    .expect("Put max_width to use TextAlign::Center")
                    - total_width)
                    / 2.0
            }
            TextAlign::Right => {
                self.max_width
                    .expect("Put max_width to use TextAlign::Right")
                    - total_width
            }
        };

        // Calculate the maximum height of items in this line for vertical alignment
        let line_height = self
            .current_line_items
            .iter()
            .map(|item| item.height)
            .fold(0.px(), |max_height, height| max_height.max(height));

        let mut current_x = start_x;

        for item in &self.current_line_items {
            // Calculate vertical offset based on alignment
            let vertical_offset = match self.default_vertical_align {
                VerticalAlign::Top => 0.px(),
                VerticalAlign::Center => (line_height - item.height) / 2.0,
                VerticalAlign::Bottom => line_height - item.height,
            };

            ctx.compose(|ctx| {
                ctx.translate((current_x, self.cursor_y + vertical_offset))
                    .add(item.rendering_tree.clone());
            });
            current_x += item.width;
        }

        self.current_line_items.clear();
    }

    fn finish(&mut self, ctx: &RenderCtx) {
        self.flush_current_line(ctx);
    }
    fn process_text(
        &mut self,
        ctx: &RenderCtx,
        text: &str,
        font: Font,
        style: TextStyle,
        text_align: TextAlign,
    ) {
        if text.is_empty() {
            return;
        }

        // Check for regex matches first
        for handler in self.regex_handlers {
            if let Some((start, end)) = handler.find_match(text) {
                let match_text = &text[start..end];

                // Process text before the match
                if start > 0 {
                    let before_text = &text[..start];
                    self.process_text_simple(
                        ctx,
                        before_text,
                        font.clone(),
                        style.clone(),
                        text_align,
                    );
                }

                // Process the matched text with the handler
                let rendered_tree = (handler.handler)(match_text);
                self.add(ctx, rendered_tree);

                // Process text after the match
                if end < text.len() {
                    let after_text = &text[end..];
                    self.process_text(ctx, after_text, font, style, text_align);
                }

                return;
            }
        }

        // If no regex matched, process as normal text
        self.process_text_simple(ctx, text, font, style, text_align);
    }

    fn can_put_bounding_box(&self, bounding_box: Rect<Px>) -> bool {
        match self.max_width {
            Some(max_width) => self.cursor_x + bounding_box.right() <= max_width,
            None => true,
        }
    }

    fn get_rendering_tree(&self, text: &str, font: Font, style: TextStyle) -> RenderingTree {
        namui::text(TextParam {
            text: text.to_string(),
            x: 0.px(),
            y: 0.px(),
            align: TextAlign::Left, // Always use Left for individual text pieces
            baseline: TextBaseline::Top, // Use Top baseline for consistent behavior
            font,
            style,
            max_width: self.max_width.map(|max_width| max_width - self.cursor_x),
        })
    }

    fn process_text_simple(
        &mut self,
        ctx: &RenderCtx,
        text: &str,
        font: Font,
        style: TextStyle,
        text_align: TextAlign,
    ) {
        if text.is_empty() {
            return;
        }

        let rendering_tree = self.get_rendering_tree(text, font.clone(), style.clone());
        let Some(bounding_box) = rendering_tree.bounding_box() else {
            return;
        };

        if self.can_put_bounding_box(bounding_box) {
            return self.add(ctx, rendering_tree);
        };

        // Find the best break point considering word boundaries
        if let Some(break_point) = self.find_best_break_point(text, font.clone(), style.clone()) {
            let (left_text, right_text) = self.split_text_at_break_point(text, break_point);

            if !left_text.is_empty() {
                // Force add left_text to current line without line break check
                let left_rendering_tree =
                    self.get_rendering_tree(&left_text, font.clone(), style.clone());
                if let Some(bounding_box) = left_rendering_tree.bounding_box() {
                    self.current_line_items.push(LineItem {
                        rendering_tree: left_rendering_tree,
                        width: bounding_box.right(),
                        height: bounding_box.height(),
                    });
                    self.line_height = self.line_height.max(bounding_box.height());
                    self.cursor_x += bounding_box.right();
                    self.is_first_in_line = false;
                }
            }

            if !right_text.is_empty() {
                // Force a line break before processing the right part
                self.flush_current_line(ctx);
                self.break_line();
                self.process_text(ctx, &right_text, font.clone(), style.clone(), text_align);
            }
        } else {
            // Fallback to character-based splitting if no good word boundary found
            let params = TextProcessParams {
                font: font.clone(),
                style: style.clone(),
                text_align,
            };
            self.fallback_character_split(ctx, text, params);
        }
    }

    /// Find the best break point considering word boundaries
    fn find_best_break_point(&self, text: &str, font: Font, style: TextStyle) -> Option<usize> {
        // Find all potential word boundaries (spaces and punctuation)
        let mut word_boundaries = vec![0];

        for (i, char) in text.char_indices() {
            if char.is_whitespace() || char.is_ascii_punctuation() {
                // Add the position after the whitespace/punctuation
                if let Some(next_pos) = text.char_indices().nth(text[..i].chars().count() + 1) {
                    word_boundaries.push(next_pos.0);
                }
            }
        }
        word_boundaries.push(text.len());
        word_boundaries.sort_unstable();
        word_boundaries.dedup();

        // Convert byte indices to character indices
        let char_boundaries: Vec<usize> = word_boundaries
            .into_iter()
            .map(|byte_idx| text[..byte_idx].chars().count())
            .collect();

        // Find the largest boundary that fits within the available width
        let mut best_boundary = None;

        for &boundary in char_boundaries.iter().rev() {
            if boundary == 0 {
                continue;
            }

            let test_text: String = text.chars().take(boundary).collect();
            let rendering_tree = self.get_rendering_tree(&test_text, font.clone(), style.clone());

            if let Some(bounding_box) = rendering_tree.bounding_box()
                && self.can_put_bounding_box(bounding_box)
            {
                best_boundary = Some(boundary);
                break;
            }
        }

        best_boundary
    }

    /// Split text at the given character position, trimming whitespace appropriately
    fn split_text_at_break_point(&self, text: &str, break_point: usize) -> (String, String) {
        let left_text: String = text.chars().take(break_point).collect();
        let right_text: String = text.chars().skip(break_point).collect();

        // Trim trailing whitespace from left part and leading whitespace from right part
        let left_trimmed = left_text.trim_end().to_string();
        let right_trimmed = right_text.trim_start().to_string();

        (left_trimmed, right_trimmed)
    }

    /// Fallback to character-based splitting when word boundary approach fails
    fn fallback_character_split(&mut self, ctx: &RenderCtx, text: &str, params: TextProcessParams) {
        let max_width = self.max_width.unwrap();
        let mut low = 0;
        let mut high = text.chars().count();

        loop {
            let middle_point = (low + high).div_ceil(2);

            let left_text = text.chars().take(middle_point).collect::<String>();
            let right_text = text.chars().skip(middle_point).collect::<String>();

            if middle_point == low || middle_point == high {
                let left_rendering_tree =
                    self.get_rendering_tree(&left_text, params.font.clone(), params.style.clone());
                if let Some(bounding_box) = left_rendering_tree.bounding_box() {
                    // Only add left_text if it fits OR if it's the first item in line
                    // This prevents character separation within words
                    if self.is_first_in_line || self.can_put_bounding_box(bounding_box) {
                        self.current_line_items.push(LineItem {
                            rendering_tree: left_rendering_tree,
                            width: bounding_box.right(),
                            height: bounding_box.height(),
                        });
                        self.line_height = self.line_height.max(bounding_box.height());
                        self.cursor_x += bounding_box.right();
                        self.is_first_in_line = false;

                        if !right_text.is_empty() {
                            self.flush_current_line(ctx);
                            self.break_line();
                            return self.process_text(
                                ctx,
                                &right_text,
                                params.font,
                                params.style,
                                params.text_align,
                            );
                        }
                    } else {
                        // If left_text doesn't fit and it's not first in line,
                        // move to next line and process entire text there
                        self.flush_current_line(ctx);
                        self.break_line();
                        return self.process_text(
                            ctx,
                            text,
                            params.font,
                            params.style,
                            params.text_align,
                        );
                    }
                } else if !right_text.is_empty() {
                    self.flush_current_line(ctx);
                    self.break_line();
                    return self.process_text(
                        ctx,
                        &right_text,
                        params.font,
                        params.style,
                        params.text_align,
                    );
                }

                return;
            }

            let left_rendering_tree =
                self.get_rendering_tree(&left_text, params.font.clone(), params.style.clone());
            let Some(left_bounding_box) = left_rendering_tree.bounding_box() else {
                if !right_text.is_empty() {
                    // Force a line break before processing the right part
                    self.flush_current_line(ctx);
                    self.break_line();
                }
                return self.process_text(
                    ctx,
                    &right_text,
                    params.font,
                    params.style,
                    params.text_align,
                );
            };

            match (self.cursor_x + left_bounding_box.right())
                .partial_cmp(&max_width)
                .unwrap()
            {
                Ordering::Equal => {
                    // Add left_text if it fits exactly
                    self.current_line_items.push(LineItem {
                        rendering_tree: left_rendering_tree,
                        width: left_bounding_box.right(),
                        height: left_bounding_box.height(),
                    });
                    self.line_height = self.line_height.max(left_bounding_box.height());
                    self.cursor_x += left_bounding_box.right();
                    self.is_first_in_line = false;

                    if !right_text.is_empty() {
                        self.flush_current_line(ctx);
                        self.break_line();
                        return self.process_text(
                            ctx,
                            &right_text,
                            params.font,
                            params.style,
                            params.text_align,
                        );
                    }
                    return;
                }
                Ordering::Less => {
                    low = middle_point;
                }
                Ordering::Greater => {
                    high = middle_point;
                }
            }
        }
    }

    fn break_line(&mut self) {
        self.cursor_x = 0.px();
        self.cursor_y += self.line_height;
        self.line_height = 0.px();
        self.is_first_in_line = true;
        // Line items are already flushed by the caller, so no need to clear here
    }
}
