mod parse;

use namui::*;
pub use parse::*;
use std::{cmp::Ordering, collections::HashMap};

pub enum Tag {
    Image { param: ImageParam },
    StyledText { font: Font, style: TextStyle },
}

pub struct RichText<'a> {
    pub text: String,
    pub max_width: Option<Px>,
    pub default_font: Font,
    pub default_text_style: TextStyle,
    pub tag_map: &'a HashMap<String, Tag>,
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
            };
        }
    }
}

struct Processor {
    max_width: Px,
    cursor_x: Px,
    cursor_y: Px,
    line_height: Px,
    is_first_in_line: bool,
}
impl Processor {
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
            let middle_point = (low + high + 1) / 2;

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
