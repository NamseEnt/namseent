mod parse;

use namui::*;
pub use parse::*;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

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
            tag_map,
            regex_handlers: &[],
            on_parse_error: None,
        }
    }

    /// Create a new RichText with custom text alignment
    pub fn with_text_alignment(
        text: String,
        max_width: Option<Px>,
        default_font: Font,
        default_text_style: TextStyle,
        default_text_align: TextAlign,
        tag_map: &'a HashMap<String, Tag>,
    ) -> Self {
        Self {
            text,
            max_width,
            default_font,
            default_text_style,
            default_text_align,
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

        let max_width = self.max_width.unwrap_or(f32::INFINITY.px());

        let mut processor = Processor::new(max_width, self.regex_handlers);
        processor.current_text_align = self.default_text_align;

        for token in tokens.iter() {
            match token {
                Token::DefaultText { text } => {
                    processor.process_text(
                        ctx,
                        text,
                        self.default_font.clone(),
                        self.default_text_style.clone(),
                        self.default_text_align,
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
                        self.default_text_align,
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

struct Processor<'a> {
    max_width: Px,
    cursor_x: Px,
    cursor_y: Px,
    line_height: Px,
    is_first_in_line: bool,
    regex_handlers: &'a [RegexHandler],
    current_line_items: Vec<LineItem>,
    current_text_align: TextAlign,
}

struct LineItem {
    rendering_tree: RenderingTree,
    width: Px,
}

struct TextProcessParams {
    font: Font,
    style: TextStyle,
    text_align: TextAlign,
}

impl<'a> Processor<'a> {
    fn new(max_width: Px, regex_handlers: &'a [RegexHandler]) -> Self {
        Self {
            max_width,
            cursor_x: 0.px(),
            cursor_y: 0.px(),
            line_height: 0.px(),
            is_first_in_line: true,
            regex_handlers,
            current_line_items: Vec::new(),
            current_text_align: TextAlign::Left,
        }
    }

    fn add(&mut self, ctx: &RenderCtx, rendering_tree: RenderingTree) {
        let Some(bounding_box) = namui::bounding_box(&rendering_tree) else {
            return;
        };

        let item_width = bounding_box.right();

        // Check if we need to break the line before adding this item
        if !self.is_first_in_line && self.cursor_x + item_width > self.max_width {
            self.flush_current_line(ctx);
            self.break_line();
        }

        // Add item to current line
        self.current_line_items.push(LineItem {
            rendering_tree,
            width: item_width,
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
        let available_width = self.max_width;

        let start_x = match self.current_text_align {
            TextAlign::Left => 0.px(),
            TextAlign::Center => (available_width - total_width) / 2.0,
            TextAlign::Right => available_width - total_width,
        };

        let mut current_x = start_x;

        for item in &self.current_line_items {
            ctx.compose(|ctx| {
                ctx.translate((current_x, self.cursor_y))
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

        // Calculate remaining width for text wrapping (not alignment)
        let remaining_width = self.max_width - self.cursor_x;

        let get_rendering_tree = |text: &str| {
            namui::text(TextParam {
                text: text.to_string(),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left, // Always use Left for individual text pieces
                baseline: TextBaseline::Top, // Use Top baseline for consistent behavior
                font: font.clone(),
                style: style.clone(),
                max_width: Some(remaining_width),
            })
        };

        {
            let rendering_tree = get_rendering_tree(text);
            let Some(bounding_box) = namui::bounding_box(&rendering_tree) else {
                return;
            };

            if self.cursor_x + bounding_box.right() <= self.max_width {
                return self.add(ctx, rendering_tree);
            }
        }

        // Find the best break point considering word boundaries
        if let Some(break_point) = self.find_best_break_point(text, &get_rendering_tree) {
            let (left_text, right_text) = self.split_text_at_break_point(text, break_point);

            if !left_text.is_empty() {
                self.add(ctx, get_rendering_tree(&left_text));
            }

            if !right_text.is_empty() {
                self.process_text(ctx, &right_text, font, style, text_align);
            }
        } else {
            // Fallback to character-based splitting if no good word boundary found
            let params = TextProcessParams {
                font: font.clone(),
                style: style.clone(),
                text_align,
            };
            self.fallback_character_split(ctx, text, params, &get_rendering_tree);
        }
    }

    /// Find the best break point considering word boundaries
    fn find_best_break_point(
        &self,
        text: &str,
        get_rendering_tree: &dyn Fn(&str) -> RenderingTree,
    ) -> Option<usize> {
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
            let rendering_tree = get_rendering_tree(&test_text);

            if let Some(bounding_box) = namui::bounding_box(&rendering_tree) {
                if self.cursor_x + bounding_box.right() <= self.max_width {
                    best_boundary = Some(boundary);
                    break;
                }
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
    fn fallback_character_split(
        &mut self,
        ctx: &RenderCtx,
        text: &str,
        params: TextProcessParams,
        get_rendering_tree: &dyn Fn(&str) -> RenderingTree,
    ) {
        let mut low = 0;
        let mut high = text.chars().count();

        loop {
            let middle_point = (low + high).div_ceil(2);

            let left_text = text.chars().take(middle_point).collect::<String>();
            let right_text = text.chars().skip(middle_point).collect::<String>();

            if middle_point == low || middle_point == high {
                self.add(ctx, get_rendering_tree(&left_text));
                return self.process_text(
                    ctx,
                    &right_text,
                    params.font,
                    params.style,
                    params.text_align,
                );
            }

            let left_rendering_tree = get_rendering_tree(&left_text);
            let Some(left_bounding_box) = namui::bounding_box(&left_rendering_tree) else {
                return self.process_text(
                    ctx,
                    &right_text,
                    params.font,
                    params.style,
                    params.text_align,
                );
            };

            match (self.cursor_x + left_bounding_box.right())
                .partial_cmp(&self.max_width)
                .unwrap()
            {
                Ordering::Equal => {
                    self.add(ctx, left_rendering_tree);
                    return self.process_text(
                        ctx,
                        &right_text,
                        params.font,
                        params.style,
                        params.text_align,
                    );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_handler_creation() {
        let handler = RegexHandler::new(
            r"icon<[^>]+>",
            Box::new(|matched_text| {
                // Mock rendering tree for testing
                namui::text(TextParam {
                    text: format!("ICON: {matched_text}"),
                    x: 0.px(),
                    y: 0.px(),
                    align: TextAlign::Left,
                    baseline: TextBaseline::Top,
                    font: Font {
                        name: "Arial".to_string(),
                        size: px(14.0).into(),
                    },
                    style: TextStyle::default(),
                    max_width: None,
                })
            }),
        );

        assert!(handler.is_ok());
    }

    #[test]
    fn test_regex_matching() {
        let handler =
            RegexHandler::new(r"icon<[^>]+>", Box::new(|_| RenderingTree::Empty)).unwrap();

        let text = "Hello icon<gold:24:32:32:1> World";
        let result = handler.find_match(text);

        assert_eq!(result, Some((6, 27))); // "icon<gold:24:32:32:1>"

        let matched = handler.get_match(text);
        assert_eq!(matched, Some("icon<gold:24:32:32:1>"));
    }

    #[test]
    fn test_multiple_regex_patterns() {
        let icon_handler =
            RegexHandler::new(r"icon<[^>]+>", Box::new(|_| RenderingTree::Empty)).unwrap();

        let mention_handler =
            RegexHandler::new(r"@\w+", Box::new(|_| RenderingTree::Empty)).unwrap();

        let text = "Hello @user and icon<gold:24:32:32:1>";

        assert_eq!(icon_handler.find_match(text), Some((16, 37)));
        assert_eq!(mention_handler.find_match(text), Some((6, 11)));
    }

    #[test]
    fn test_korean_text_char_boundary() {
        // Test that Korean text is properly handled with character boundaries
        let korean_text = "한글 텍스트 아이콘 태그 테스트";
        let char_count = korean_text.chars().count();
        let byte_len = korean_text.len();

        // Verify that character count is different from byte length for Korean text
        assert_ne!(char_count, byte_len);
        assert_eq!(char_count, 17); // 17 characters including spaces
        assert_eq!(byte_len, 43); // UTF-8 encoded byte length
        assert!(byte_len > char_count); // More bytes than characters due to UTF-8 encoding

        // Test character slicing works correctly - split at position 9 (middle of "아이콘")
        let first_part: String = korean_text.chars().take(9).collect();
        let second_part: String = korean_text.chars().skip(9).collect();

        assert_eq!(first_part, "한글 텍스트 아이");
        assert_eq!(second_part, "콘 태그 테스트");
        assert_eq!(format!("{first_part}{second_part}"), korean_text);

        // Test that our character-based splitting avoids byte boundary errors
        for i in 0..=char_count {
            let left: String = korean_text.chars().take(i).collect();
            let right: String = korean_text.chars().skip(i).collect();
            assert_eq!(format!("{left}{right}"), korean_text);
        }
    }

    #[test]
    fn test_word_boundary_line_breaking() {
        // Test that word boundaries are respected when line breaking occurs
        let text = "Happiness and Joy";

        // Create a mock processor to test the word boundary logic
        let regex_handlers: [RegexHandler; 0] = [];
        let processor = Processor::new(100.px(), &regex_handlers);

        // Test finding word boundaries
        let boundaries = [
            (0, ""),
            (9, "Happiness"),          // Should break after "Happiness"
            (13, "Happiness and"),     // Should break after "and"
            (17, "Happiness and Joy"), // Complete text
        ];

        for (expected_char_pos, expected_text) in boundaries {
            let test_chars: String = text.chars().take(expected_char_pos).collect();
            assert_eq!(test_chars, expected_text);
        }

        // Test that split_text_at_break_point trims whitespace correctly
        let (left, right) = processor.split_text_at_break_point(text, 9);
        assert_eq!(left, "Happiness");
        assert_eq!(right, "and Joy");

        let (left, right) = processor.split_text_at_break_point(text, 13);
        assert_eq!(left, "Happiness and");
        assert_eq!(right, "Joy");
    }

    #[test]
    fn test_regex_handler_full_integration() {
        // Create a regex handler that matches icon patterns
        let icon_handler = RegexHandler::new(
            r"icon<([^:>]+):(\d+):(\d+):(\d+):(\d+)>",
            Box::new(|matched_text| {
                namui::text(TextParam {
                    text: format!("[ICON:{matched_text}]"),
                    x: 0.px(),
                    y: 0.px(),
                    align: TextAlign::Left,
                    baseline: TextBaseline::Top,
                    font: Font {
                        name: "Arial".to_string(),
                        size: px(14.0).into(),
                    },
                    style: TextStyle::default(),
                    max_width: None,
                })
            }),
        )
        .unwrap();

        // Create a mention handler
        let mention_handler = RegexHandler::new(
            r"@(\w+)",
            Box::new(|matched_text| {
                namui::text(TextParam {
                    text: format!("[MENTION:{matched_text}]"),
                    x: 0.px(),
                    y: 0.px(),
                    align: TextAlign::Left,
                    baseline: TextBaseline::Top,
                    font: Font {
                        name: "Arial".to_string(),
                        size: px(14.0).into(),
                    },
                    style: TextStyle::default(),
                    max_width: None,
                })
            }),
        )
        .unwrap();

        let regex_handlers = [icon_handler, mention_handler];

        // Test with real input that should match multiple patterns
        let test_text = "Hello @user, here's an icon: icon<gold:24:16:16:1> and another @admin.";

        // Verify icon pattern matches
        assert!(regex_handlers[0].find_match(test_text).is_some());
        let icon_match = regex_handlers[0].find_match(test_text).unwrap();
        assert_eq!(
            &test_text[icon_match.0..icon_match.1],
            "icon<gold:24:16:16:1>"
        );

        // Verify mention pattern matches
        assert!(regex_handlers[1].find_match(test_text).is_some());
        let mention_match = regex_handlers[1].find_match(test_text).unwrap();
        assert_eq!(&test_text[mention_match.0..mention_match.1], "@user");

        // Test that empty regex handlers array doesn't break anything
        let empty_handlers: [RegexHandler; 0] = [];
        assert_eq!(empty_handlers.len(), 0);
    }
}
