// Tower Defense Rich Text Implementation
// Token-based rendering using tokens from namui-prebuilt
// Optimized for game UI text rendering

use namui::*;
use namui_prebuilt::rich_text::Token;
use std::collections::HashMap;

/// Inline layout configuration
#[derive(Debug, Clone, Copy)]
pub struct InlineLayoutConfig {
    pub max_width: Option<Px>,
    pub text_align: TextAlign,
    pub vertical_align: VerticalAlign,
    pub line_height: LineHeight,
}

#[derive(Debug, Clone, Copy)]
pub enum LineHeight {
    Normal,
    Px(Px),
    Percent(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticalAlign {
    Baseline,
    Top,
    Middle,
    Bottom,
}

/// Text with measured dimensions
#[derive(Debug, Clone)]
pub struct ShapedText {
    pub text: String,
    pub font: Font,
    pub style: TextStyle,
    pub width: Px,
    pub height: Px,
    pub ascent: Px,
    pub descent: Px,
}

impl ShapedText {
    pub fn shape(text: String, font: Font, style: TextStyle) -> Self {
        let rendering_tree = namui::text(TextParam {
            text: text.clone(),
            x: 0.px(),
            y: 0.px(),
            align: TextAlign::Left,
            baseline: TextBaseline::Top,
            font: font.clone(),
            style: style.clone(),
            max_width: None,
        });

        let (width, height) = rendering_tree
            .bounding_box()
            .map(|bb| (bb.width(), bb.height()))
            .unwrap_or((0.px(), 0.px()));

        let font_size: Px = font.size.into();
        let ascent = font_size * 0.8;
        let descent = font_size * 0.2;

        Self {
            text,
            font,
            style,
            width,
            height,
            ascent,
            descent,
        }
    }
}

/// Inline box types
#[derive(Debug, Clone)]
pub enum InlineBox {
    Text(ShapedText),
    Atomic {
        content: RenderingTree,
        width: Px,
        height: Px,
        baseline: Px,
    },
    SoftBreak,
    HardBreak,
    Space {
        width: Px,
        font: Font,
    },
}

impl InlineBox {
    pub fn width(&self) -> Px {
        match self {
            InlineBox::Text(shaped) => shaped.width,
            InlineBox::Atomic { width, .. } => *width,
            InlineBox::SoftBreak | InlineBox::HardBreak => 0.px(),
            InlineBox::Space { width, .. } => *width,
        }
    }

    pub fn height(&self) -> Px {
        match self {
            InlineBox::Text(shaped) => shaped.height,
            InlineBox::Atomic { height, .. } => *height,
            InlineBox::SoftBreak | InlineBox::HardBreak => 0.px(),
            InlineBox::Space { font, .. } => {
                let size: Px = font.size.into();
                size
            }
        }
    }

    pub fn baseline(&self) -> Px {
        match self {
            InlineBox::Text(shaped) => shaped.ascent,
            InlineBox::Atomic { baseline, .. } => *baseline,
            InlineBox::SoftBreak | InlineBox::HardBreak => 0.px(),
            InlineBox::Space { font, .. } => {
                let size: Px = font.size.into();
                size * 0.8
            }
        }
    }

    pub fn is_break_opportunity(&self) -> bool {
        matches!(
            self,
            InlineBox::SoftBreak | InlineBox::HardBreak | InlineBox::Space { .. }
        )
    }

    pub fn is_hard_break(&self) -> bool {
        matches!(self, InlineBox::HardBreak)
    }
}

/// Positioned inline box in a line
#[derive(Debug, Clone)]
pub struct PositionedInlineBox {
    pub inline_box: InlineBox,
    pub x: Px,
    pub y: Px,
}

/// Line box containing positioned inline boxes
#[derive(Debug, Clone)]
pub struct LineBox {
    pub boxes: Vec<PositionedInlineBox>,
    pub content_width: Px,
    pub height: Px,
    pub baseline: Px,
    pub leading: Px,
}

/// Inline layout engine
pub struct InlineLayoutEngine {
    config: InlineLayoutConfig,
}

impl InlineLayoutEngine {
    pub fn new(config: InlineLayoutConfig) -> Self {
        Self { config }
    }

    pub fn layout(&self, inline_boxes: Vec<InlineBox>) -> Vec<LineBox> {
        if inline_boxes.is_empty() {
            return vec![];
        }

        match self.config.max_width {
            Some(max_width) => self.layout_with_wrapping(inline_boxes, max_width),
            None => vec![self.layout_single_line(inline_boxes)],
        }
    }

    fn layout_single_line(&self, inline_boxes: Vec<InlineBox>) -> LineBox {
        let mut x = 0.px();
        let mut max_ascent = 0.px();
        let mut max_descent = 0.px();
        let mut positioned_boxes = Vec::new();

        for inline_box in inline_boxes {
            let width = inline_box.width();
            let height = inline_box.height();
            let baseline = inline_box.baseline();

            max_ascent = max_ascent.max(baseline);
            max_descent = max_descent.max(height - baseline);

            positioned_boxes.push(PositionedInlineBox {
                inline_box,
                x,
                y: 0.px(),
            });

            x += width;
        }

        let baseline_pos = max_ascent;
        let line_height = self.compute_line_height(max_ascent + max_descent);
        let leading = (line_height - (max_ascent + max_descent)).max(0.px());

        for positioned in &mut positioned_boxes {
            positioned.y = self.compute_vertical_offset(
                positioned.inline_box.baseline(),
                positioned.inline_box.height(),
                baseline_pos,
            );
        }

        LineBox {
            boxes: positioned_boxes,
            content_width: x,
            height: line_height,
            baseline: baseline_pos + leading / 2.0,
            leading,
        }
    }

    fn layout_with_wrapping(&self, inline_boxes: Vec<InlineBox>, max_width: Px) -> Vec<LineBox> {
        let mut lines = Vec::new();
        let mut current_line_boxes = Vec::new();
        let mut current_width = 0.px();

        for inline_box in inline_boxes {
            if inline_box.is_hard_break() {
                if !current_line_boxes.is_empty() {
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line_boxes)));
                    current_width = 0.px();
                }
                continue;
            }

            let box_width = inline_box.width();

            if current_width + box_width <= max_width {
                current_width += box_width;
                current_line_boxes.push(inline_box);
            } else {
                if let Some(break_index) = self.find_break_opportunity(&current_line_boxes) {
                    let remaining: Vec<_> = current_line_boxes.drain(break_index..).collect();

                    if !current_line_boxes.is_empty() {
                        lines
                            .push(self.layout_single_line(std::mem::take(&mut current_line_boxes)));
                    }

                    for box_item in remaining.into_iter().rev() {
                        current_line_boxes.insert(0, box_item);
                    }
                    current_line_boxes.push(inline_box);
                    current_width = current_line_boxes.iter().map(|b| b.width()).sum();
                } else if current_line_boxes.is_empty() {
                    current_line_boxes.push(inline_box);
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line_boxes)));
                    current_width = 0.px();
                } else {
                    lines.push(self.layout_single_line(std::mem::take(&mut current_line_boxes)));
                    current_line_boxes.push(inline_box);
                    current_width = box_width;
                }
            }
        }

        if !current_line_boxes.is_empty() {
            lines.push(self.layout_single_line(current_line_boxes));
        }

        lines
    }

    fn find_break_opportunity(&self, boxes: &[InlineBox]) -> Option<usize> {
        boxes
            .iter()
            .enumerate()
            .rev()
            .find(|(_, b)| b.is_break_opportunity())
            .map(|(i, _)| i)
    }

    fn compute_line_height(&self, content_height: Px) -> Px {
        match self.config.line_height {
            LineHeight::Normal => content_height,
            LineHeight::Px(height) => height,
            LineHeight::Percent(percent) => content_height * percent,
        }
    }

    fn compute_vertical_offset(&self, box_baseline: Px, box_height: Px, line_baseline: Px) -> Px {
        match self.config.vertical_align {
            VerticalAlign::Baseline => line_baseline - box_baseline,
            VerticalAlign::Top => 0.px(),
            VerticalAlign::Middle => line_baseline - box_height / 2.0,
            VerticalAlign::Bottom => line_baseline - box_height,
        }
    }
}

/// Text segmenter for tokenization
pub struct TextSegmenter {
    font: Font,
    style: TextStyle,
}

impl TextSegmenter {
    pub fn new(font: Font, style: TextStyle) -> Self {
        Self { font, style }
    }

    pub fn segment(&self, text: &str) -> Vec<InlineBox> {
        let mut boxes = Vec::new();
        let mut current_word = String::new();

        for ch in text.chars() {
            match ch {
                ' ' => {
                    if !current_word.is_empty() {
                        boxes.push(InlineBox::Text(ShapedText::shape(
                            current_word.clone(),
                            self.font.clone(),
                            self.style.clone(),
                        )));
                        current_word.clear();
                    }
                    let space_shaped =
                        ShapedText::shape(" ".to_string(), self.font.clone(), self.style.clone());
                    boxes.push(InlineBox::Space {
                        width: space_shaped.width,
                        font: self.font.clone(),
                    });
                    boxes.push(InlineBox::SoftBreak);
                }
                '\n' => {
                    if !current_word.is_empty() {
                        boxes.push(InlineBox::Text(ShapedText::shape(
                            current_word.clone(),
                            self.font.clone(),
                            self.style.clone(),
                        )));
                        current_word.clear();
                    }
                    boxes.push(InlineBox::HardBreak);
                }
                _ => {
                    current_word.push(ch);
                }
            }
        }

        if !current_word.is_empty() {
            boxes.push(InlineBox::Text(ShapedText::shape(
                current_word,
                self.font.clone(),
                self.style.clone(),
            )));
        }

        boxes
    }
}

/// Rich text renderer configuration
pub struct RichTextConfig {
    pub max_width: Option<Px>,
    pub font: Font,
    pub text_style: TextStyle,
    pub text_align: TextAlign,
    pub vertical_align: VerticalAlign,
    pub line_height: LineHeight,
    pub tag_styles: HashMap<String, (Option<Font>, Option<TextStyle>)>,
    pub content_providers: HashMap<String, Box<dyn Fn() -> RenderingTree>>,
}

impl Default for RichTextConfig {
    fn default() -> Self {
        Self {
            max_width: None,
            font: Font {
                name: "Arial".to_string(),
                size: int_px(14),
            },
            text_style: TextStyle::default(),
            text_align: TextAlign::Left,
            vertical_align: VerticalAlign::Baseline,
            line_height: LineHeight::Normal,
            tag_styles: HashMap::new(),
            content_providers: HashMap::new(),
        }
    }
}

/// Rich text renderer using tokens directly
pub struct RichTextRenderer {
    config: RichTextConfig,
}

impl RichTextRenderer {
    pub fn new(config: RichTextConfig) -> Self {
        Self { config }
    }

    /// Render from Token vector (skip parsing phase)
    pub fn render_from_tokens(&self, tokens: Vec<Token>, ctx: &RenderCtx) {
        let inline_boxes = self.tokens_to_inline_boxes(tokens);

        let layout_config = InlineLayoutConfig {
            max_width: self.config.max_width,
            text_align: self.config.text_align,
            vertical_align: self.config.vertical_align,
            line_height: self.config.line_height,
        };

        let engine = InlineLayoutEngine::new(layout_config);
        let lines = engine.layout(inline_boxes);

        self.render_lines(ctx, lines);
    }

    fn tokens_to_inline_boxes(&self, tokens: Vec<Token>) -> Vec<InlineBox> {
        let mut boxes = Vec::new();

        for token in tokens {
            match token {
                Token::DefaultText { text } => {
                    let segmenter = TextSegmenter::new(
                        self.config.font.clone(),
                        self.config.text_style.clone(),
                    );
                    boxes.extend(segmenter.segment(&text));
                }
                Token::StyledText { tag, text } => {
                    let (font, style) = self
                        .config
                        .tag_styles
                        .get(&tag)
                        .cloned()
                        .unwrap_or((None, None));

                    let font = font.unwrap_or_else(|| self.config.font.clone());
                    let style = style.unwrap_or_else(|| self.config.text_style.clone());

                    let segmenter = TextSegmenter::new(font, style);
                    boxes.extend(segmenter.segment(&text));
                }
                Token::Image { tag } | Token::RenderingTree { tag } => {
                    if let Some(provider) = self.config.content_providers.get(&tag) {
                        let content = provider();
                        let (width, height): (Px, Px) = content
                            .bounding_box()
                            .map(|bb| (bb.width(), bb.height()))
                            .unwrap_or((0.px(), 0.px()));

                        boxes.push(InlineBox::Atomic {
                            content,
                            width,
                            height,
                            baseline: height * 0.8,
                        });
                    }
                }
            }
        }

        boxes
    }

    fn render_lines(&self, ctx: &RenderCtx, lines: Vec<LineBox>) {
        let mut y = 0.px();

        for line in lines {
            let x_offset = self.compute_line_offset(&line);

            for positioned in line.boxes {
                let x = x_offset + positioned.x;
                let box_y = y + positioned.y;

                match positioned.inline_box {
                    InlineBox::Text(shaped) => {
                        ctx.add(namui::text(TextParam {
                            text: shaped.text,
                            x,
                            y: box_y,
                            align: TextAlign::Left,
                            baseline: TextBaseline::Top,
                            font: shaped.font,
                            style: shaped.style,
                            max_width: None,
                        }));
                    }
                    InlineBox::Atomic { content, .. } => {
                        ctx.compose(|ctx| {
                            ctx.translate((x, box_y)).add(content);
                        });
                    }
                    InlineBox::Space { .. } | InlineBox::SoftBreak | InlineBox::HardBreak => {}
                }
            }

            y += line.height;
        }
    }

    fn compute_line_offset(&self, line: &LineBox) -> Px {
        if let Some(max_width) = self.config.max_width {
            match self.config.text_align {
                TextAlign::Left => 0.px(),
                TextAlign::Center => (max_width - line.content_width) / 2.0,
                TextAlign::Right => max_width - line.content_width,
            }
        } else {
            0.px()
        }
    }
}
