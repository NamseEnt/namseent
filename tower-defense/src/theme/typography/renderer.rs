use super::inline_box::{InlineBox, LineBox, ShapedText};
use super::layout::{LayoutConfig, LayoutEngine};
use super::style::StyleStack;
use super::token::Token;
use crate::icon::{Icon, IconSize};
use namui::*;

/// Rendered rich text result
/// Contains the rendering tree and dimensions for immediate rendering
#[derive(State, Clone)]
pub struct RenderedRichText {
    rendering_tree: RenderingTree,
    pub width: Px,
    pub height: Px,
}

impl RenderedRichText {
    /// Convert into the underlying rendering tree
    pub fn into_rendering_tree(self) -> RenderingTree {
        self.rendering_tree
    }
}

impl Component for RenderedRichText {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(self.rendering_tree);
    }
}

/// Rich text renderer
pub struct RichTextRenderer {
    default_style: super::style::StyleContext,
    layout_config: LayoutConfig,
}

impl RichTextRenderer {
    pub fn new(default_style: super::style::StyleContext, layout_config: LayoutConfig) -> Self {
        Self {
            default_style,
            layout_config,
        }
    }

    /// Render tokens into RenderedRichText
    pub fn render(&self, tokens: &[Token]) -> RenderedRichText {
        let mut style_stack = StyleStack::new(self.default_style.clone());
        let boxes = self.tokens_to_boxes(tokens, &mut style_stack);

        let layout_engine = LayoutEngine::new(self.layout_config);
        let lines = layout_engine.layout(boxes);

        let (width, height) = self.calculate_dimensions(&lines);
        let rendering_tree = self.build_rendering_tree(&lines);

        RenderedRichText {
            rendering_tree,
            width,
            height,
        }
    }

    fn tokens_to_boxes(&self, tokens: &[Token], style_stack: &mut StyleStack) -> Vec<InlineBox> {
        let mut boxes = Vec::new();

        for token in tokens {
            match token {
                Token::StaticText(text) => {
                    let style = style_stack.current();
                    let shaped = ShapedText::shape(
                        text.to_string(),
                        style.to_font(),
                        style.to_text_style(),
                        style.vertical_align,
                    );
                    boxes.push(InlineBox::Text(shaped));
                }
                Token::Text(text) => {
                    let style = style_stack.current();
                    let shaped = ShapedText::shape(
                        text.clone(),
                        style.to_font(),
                        style.to_text_style(),
                        style.vertical_align,
                    );
                    boxes.push(InlineBox::Text(shaped));
                }
                Token::ApplyStyle(delta) => {
                    style_stack.apply_delta(*delta);
                }
                Token::Save => {
                    style_stack.save();
                }
                Token::Restore => {
                    style_stack.restore();
                }
                Token::Icon(icon) => {
                    let style = style_stack.current();
                    let font_size: Px = style.font_size.into_px();
                    let icon_wh = Wh::single(font_size);

                    let icon_component = Icon::new(icon.icon_kind)
                        .size(IconSize::Custom { size: font_size })
                        .wh(icon_wh)
                        .attributes(icon.attributes.clone())
                        .to_rendering_tree();

                    boxes.push(InlineBox::Atomic {
                        content: icon_component,
                        width: font_size,
                        height: font_size,
                        baseline: font_size * 0.8,
                        vertical_align: style.vertical_align,
                    });
                }
                Token::Space => {
                    let style = style_stack.current();
                    let space_shaped = ShapedText::shape(
                        " ".to_string(),
                        style.to_font(),
                        style.to_text_style(),
                        style.vertical_align,
                    );
                    boxes.push(InlineBox::Space {
                        width: space_shaped.width,
                    });
                    boxes.push(InlineBox::SoftBreak);
                }
                Token::LineBreak => {
                    boxes.push(InlineBox::HardBreak);
                }
            }
        }

        boxes
    }

    fn build_rendering_tree(&self, lines: &[LineBox]) -> RenderingTree {
        let mut items = Vec::new();
        let mut y = 0.px();

        for line in lines {
            let x_offset = match self.layout_config.text_align {
                TextAlign::Left => 0.px(),
                TextAlign::Center => {
                    if let Some(max_width) = self.layout_config.max_width {
                        (max_width - line.content_width) / 2.0
                    } else {
                        0.px()
                    }
                }
                TextAlign::Right => {
                    if let Some(max_width) = self.layout_config.max_width {
                        max_width - line.content_width
                    } else {
                        0.px()
                    }
                }
            };

            for positioned in &line.boxes {
                let x = x_offset + positioned.x;
                let box_y = y + positioned.y;

                match &positioned.inline_box {
                    InlineBox::Text(shaped) => {
                        items.push(namui::text(TextParam {
                            text: shaped.text.clone(),
                            x,
                            y: box_y,
                            align: TextAlign::Left,
                            baseline: positioned.text_baseline,
                            font: shaped.font.clone(),
                            style: shaped.style.clone(),
                            max_width: None,
                        }));
                    }
                    InlineBox::Atomic { content, .. } => {
                        items.push(translate(x, box_y, content.clone()));
                    }
                    InlineBox::SoftBreak | InlineBox::HardBreak | InlineBox::Space { .. } => {}
                }
            }

            y += line.height;
        }

        RenderingTree::Children(items)
    }

    fn calculate_dimensions(&self, lines: &[LineBox]) -> (Px, Px) {
        let total_width = lines
            .iter()
            .map(|l| l.content_width)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.px());
        let total_height = lines.iter().map(|l| l.height).sum();

        (total_width, total_height)
    }
}
