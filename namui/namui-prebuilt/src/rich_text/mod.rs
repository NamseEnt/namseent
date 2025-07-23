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
    /// Create a new RichText with regex handlers
    pub fn with_regex_handlers(
        text: String,
        max_width: Option<Px>,
        default_font: Font,
        default_text_style: TextStyle,
        tag_map: &'a HashMap<String, Tag>,
        regex_handlers: &'a [RegexHandler],
    ) -> Self {
        Self {
            text,
            max_width,
            default_font,
            default_text_style,
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
    pub tag_map: &'a HashMap<String, Tag>,
    pub regex_handlers: &'a [RegexHandler],
    pub on_parse_error: Option<&'a dyn Fn(ParseError)>,
}

impl Component for RichText<'_> {
    fn render(self, ctx: &RenderCtx) {
        let tokens = ctx.memo(|| {
            parse::parse(self.text).unwrap_or_else(|err| {
                if let Some(on_parse_error) = &self.on_parse_error {
                    on_parse_error(err);
                }
                vec![]
            })
        });

        let max_width = self.max_width.unwrap_or(f32::INFINITY.px());

        let mut processor = Processor {
            max_width,
            cursor_x: 0.px(),
            cursor_y: 0.px(),
            line_height: 0.px(),
            is_first_in_line: true,
            regex_handlers: self.regex_handlers,
        };

        for token in tokens.iter() {
            match token {
                Token::DefaultText { text } => {
                    processor.process_text(
                        ctx,
                        text,
                        self.default_font.clone(),
                        self.default_text_style.clone(),
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

                    processor.process_text(ctx, text, font.clone(), style.clone());
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
    }
}

struct Processor<'a> {
    max_width: Px,
    cursor_x: Px,
    cursor_y: Px,
    line_height: Px,
    is_first_in_line: bool,
    regex_handlers: &'a [RegexHandler],
}
impl<'a> Processor<'a> {
    fn add(&mut self, ctx: &RenderCtx, rendering_tree: RenderingTree) {
        let Some(bounding_box) = namui::bounding_box(&rendering_tree) else {
            return;
        };
        if !self.is_first_in_line && self.cursor_x + bounding_box.right() > self.max_width {
            self.break_line();
        }

        self.line_height = self.line_height.max(bounding_box.height());

        ctx.compose(|ctx| {
            ctx.translate((self.cursor_x, self.cursor_y))
                .add(rendering_tree);
        });

        self.is_first_in_line = false;
        self.cursor_x += bounding_box.right();
    }
    fn process_text(&mut self, ctx: &RenderCtx, text: &str, font: Font, style: TextStyle) {
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
                    self.process_text_simple(ctx, before_text, font.clone(), style.clone());
                }

                // Process the matched text with the handler
                let rendered_tree = (handler.handler)(match_text);
                self.add(ctx, rendered_tree);

                // Process text after the match
                if end < text.len() {
                    let after_text = &text[end..];
                    self.process_text(ctx, after_text, font, style);
                }

                return;
            }
        }

        // If no regex matched, process as normal text
        self.process_text_simple(ctx, text, font, style);
    }

    fn process_text_simple(&mut self, ctx: &RenderCtx, text: &str, font: Font, style: TextStyle) {
        if text.is_empty() {
            return;
        }
        let get_rendering_tree = |text: &str| {
            namui::text(TextParam {
                text: text.to_string(),
                x: 0.px(),
                y: 0.px(),
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                font: font.clone(),
                style: style.clone(),
                max_width: None,
            })
        };

        {
            let rendering_tree = get_rendering_tree(text);
            let Some(bounding_box) = namui::bounding_box(&rendering_tree) else {
                return;
            };

            if self.cursor_x + bounding_box.right() < self.max_width {
                return self.add(ctx, rendering_tree);
            }
        }

        let mut low = 0;
        let mut high = text.len();

        loop {
            let middle_point = (low + high).div_ceil(2);

            let left_text = &text[..middle_point];
            let right_text = &text[middle_point..];

            if middle_point == low || middle_point == high {
                self.add(ctx, get_rendering_tree(left_text));
                return self.process_text(ctx, right_text, font, style);
            }

            let left_rendering_tree = get_rendering_tree(left_text);
            let Some(left_bounding_box) = namui::bounding_box(&left_rendering_tree) else {
                return self.process_text(ctx, right_text, font, style);
            };

            match (self.cursor_x + left_bounding_box.right())
                .partial_cmp(&self.max_width)
                .unwrap()
            {
                Ordering::Equal => {
                    self.add(ctx, left_rendering_tree);
                    return self.process_text(ctx, right_text, font, style);
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
}
