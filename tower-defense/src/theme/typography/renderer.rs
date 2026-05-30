use super::inline_box::{InlineBox, LineBox, ShapedText};
use super::layout::{LayoutConfig, LayoutEngine};
use super::style::StyleStack;
use super::token::Token;
use crate::icon::{Icon, IconSize};
use namui::*;

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

    /// Render tokens into a RenderingTree
    pub fn render(&self, tokens: &[Token]) -> RenderingTree {
        let mut style_stack = StyleStack::new(self.default_style.clone());
        let boxes = self.tokens_to_boxes(tokens, &mut style_stack);

        let layout_engine = LayoutEngine::new(self.layout_config);
        let lines = layout_engine.layout(boxes);

        self.build_rendering_tree(&lines)
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
                        let mut fill_style = shaped.style.clone();
                        let line_height_percent = fill_style.line_height_percent;
                        let border = fill_style.border.take();

                        items.push(namui::text(TextParam {
                            text: shaped.text.clone(),
                            x,
                            y: box_y,
                            align: TextAlign::Left,
                            baseline: positioned.text_baseline,
                            font: shaped.font.clone(),
                            style: fill_style,
                            max_width: None,
                        }));

                        if let Some(border) = border {
                            let stroke_paint = Paint::new(border.color)
                                .set_style(PaintStyle::Stroke)
                                .set_stroke_width(border.width)
                                .set_stroke_position(StrokePosition::Outside)
                                .set_stroke_join(StrokeJoin::Round)
                                .set_anti_alias(true);

                            items.push(RenderingTree::Node(DrawCommand::Text {
                                command: arena_alloc(TextDrawCommand {
                                    text: shaped.text.clone(),
                                    font: shaped.font.clone(),
                                    x,
                                    y: box_y,
                                    paint: stroke_paint,
                                    align: TextAlign::Left,
                                    baseline: positioned.text_baseline,
                                    max_width: None,
                                    line_height_percent,
                                    underline: None,
                                }),
                            }));
                        }
                    }
                    InlineBox::Atomic { content, .. } => {
                        items.push(translate(x, box_y, *content));
                    }
                    InlineBox::SoftBreak | InlineBox::HardBreak | InlineBox::Space { .. } => {}
                }
            }

            y += line.height;
        }

        RenderingTree::Children(arena_alloc_slice(items))
    }
}
