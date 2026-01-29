use super::renderer::{RenderedRichText, RichTextRenderer};
use super::style::{StyleContext, StyleDelta};
use super::token::Token;
use super::{DEFAULT_TEXT_STYLE, FontSize, palette};
use crate::icon::{IconAttribute, IconAttributePosition, IconKind};
use namui::*;

/// Typography variant type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypographyVariant {
    Headline,
    Paragraph,
}

/// Positioned rich text output
pub struct PositionedRichText {
    pub rich_text: RenderedRichText,
    pub offset: Xy<Px>,
}

impl PositionedRichText {
    pub fn new(rich_text: RenderedRichText, offset: Xy<Px>) -> Self {
        Self { rich_text, offset }
    }
}

impl Component for PositionedRichText {
    fn render(self, ctx: &RenderCtx) {
        ctx.translate(self.offset).add(self.rich_text);
    }
}

/// Fluent typography builder
pub struct TypographyBuilder<'a> {
    variant: TypographyVariant,
    tokens: Vec<Token<'a>>,
    layout_config: super::layout::LayoutConfig,
}

impl<'a> TypographyBuilder<'a> {
    /// Start building headline typography
    pub fn headline() -> Self {
        Self {
            variant: TypographyVariant::Headline,
            tokens: Vec::new(),
            layout_config: super::layout::LayoutConfig {
                max_width: None,
                text_align: TextAlign::Left,
                line_height_percent: 1.3,
            },
        }
    }

    /// Start building paragraph typography
    pub fn paragraph() -> Self {
        Self {
            variant: TypographyVariant::Paragraph,
            tokens: Vec::new(),
            layout_config: super::layout::LayoutConfig {
                max_width: None,
                text_align: TextAlign::Left,
                line_height_percent: 1.3,
            },
        }
    }

    // Note: new() method removed because it requires 'static lifetime for text
    // Use headline().text("...") or paragraph().text("...") instead

    /// Set font size using predefined FontSize enum
    /// - FontSize::Large, Medium, Small: Uses predefined sizes based on variant
    /// - FontSize::Custom { size }: Uses custom size
    pub fn size(mut self, size: FontSize) -> Self {
        let size_px = match size {
            FontSize::Large => match self.variant {
                TypographyVariant::Headline => super::HEADLINE_FONT_SIZE_LARGE,
                TypographyVariant::Paragraph => super::PARAGRAPH_FONT_SIZE_LARGE,
            },
            FontSize::Medium => match self.variant {
                TypographyVariant::Headline => super::HEADLINE_FONT_SIZE_MEDIUM,
                TypographyVariant::Paragraph => super::PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            FontSize::Small => match self.variant {
                TypographyVariant::Headline => super::HEADLINE_FONT_SIZE_SMALL,
                TypographyVariant::Paragraph => super::PARAGRAPH_FONT_SIZE_SMALL,
            },
            FontSize::Custom { size } => IntPx::from(size),
        };
        self.tokens
            .push(Token::ApplyStyle(StyleDelta::font_size(size_px)));
        self
    }

    /// Set text color
    pub fn color(mut self, color: Color) -> Self {
        self.tokens
            .push(Token::ApplyStyle(StyleDelta::color(color)));
        self
    }

    /// Set bold style
    pub fn bold(mut self) -> Self {
        self.tokens.push(Token::ApplyStyle(StyleDelta::bold()));
        self
    }

    /// Set underline style
    pub fn underline(mut self) -> Self {
        self.tokens.push(Token::ApplyStyle(StyleDelta {
            font_size: None,
            color: None,
            bold: None,
            underline: Some(true),
            border: None,
            vertical_align: None,
        }));
        self
    }

    /// Add stroke (text border)
    pub fn stroke(mut self, width: Px, color: Color) -> Self {
        self.tokens
            .push(Token::ApplyStyle(StyleDelta::stroke(width, color)));
        self
    }

    /// Add static text (borrowed reference - lifetime must match builder's 'a)
    pub fn static_text(mut self, text: &'a str) -> Self {
        self.tokens.push(Token::StaticText(text));
        self
    }

    /// Add dynamic text (owned String - no lifetime constraints)
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.tokens.push(Token::Text(text.into()));
        self
    }

    /// Add localized rich text (supports l10n types with builder integration)
    pub fn l10n<L>(self, localized: L, locale: &crate::l10n::Locale) -> Self
    where
        L: crate::l10n::LocalizedText,
    {
        localized.apply_to_builder(self, locale)
    }

    /// Add static icon (TODO: implement icon rendering)
    pub fn icon<F>(mut self, icon_kind: IconKind) -> Self {
        self.tokens
            .push(Token::Icon(TypographyIcon::new(icon_kind)));
        self
    }

    pub fn icon_with_attribute<F>(mut self, icon_kind: IconKind, build: F) -> Self
    where
        F: FnOnce(&mut TypographyIcon),
    {
        let mut icon = TypographyIcon::new(icon_kind);
        build(&mut icon);
        self.tokens.push(Token::Icon(icon));
        self
    }

    /// Add space
    pub fn space(mut self) -> Self {
        self.tokens.push(Token::Space);
        self
    }

    /// Add line break
    pub fn line_break(mut self) -> Self {
        self.tokens.push(Token::LineBreak);
        self
    }

    /// Apply temporary style scope using save/restore pattern
    /// The closure receives a builder with the style saved,
    /// and the style is automatically restored after the closure returns
    pub fn with_style<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut TypographyBuilder<'a>) -> TypographyBuilder<'a>,
    {
        self.tokens.push(Token::Save);
        f(&mut self);
        self.tokens.push(Token::Restore);
        self
    }

    /// Set text alignment for layout (Left, Center, Right)
    pub fn text_align(mut self, align: TextAlign) -> Self {
        self.layout_config.text_align = align;
        self
    }

    /// Set vertical alignment (Top, Middle, Bottom)
    pub fn vertical_align(mut self, align: super::style::VerticalAlign) -> Self {
        self.tokens
            .push(Token::ApplyStyle(StyleDelta::vertical_align(align)));
        self
    }

    /// Set max width for text wrapping
    pub fn max_width(mut self, max_width: Px) -> Self {
        self.layout_config.max_width = Some(max_width);
        self
    }

    /// Set line height as a percentage multiplier (e.g., 1.3 for 130% line height)
    pub fn line_height(mut self, percent: f32) -> Self {
        self.layout_config.line_height_percent = percent;
        self
    }

    /// Build the typography
    pub fn render(self) -> RenderedRichText {
        let default_style = match self.variant {
            TypographyVariant::Headline => StyleContext::new(
                super::HEADLINE_FONT_NAME.to_string(),
                super::HEADLINE_FONT_SIZE_MEDIUM,
                palette::ON_SURFACE,
                DEFAULT_TEXT_STYLE,
            ),
            TypographyVariant::Paragraph => StyleContext::new(
                super::PARAGRAPH_FONT_NAME.to_string(),
                super::PARAGRAPH_FONT_SIZE_MEDIUM,
                palette::ON_SURFACE,
                DEFAULT_TEXT_STYLE,
            ),
        };

        let renderer = RichTextRenderer::new(default_style, self.layout_config);
        renderer.render(&self.tokens)
    }

    /// Render and position at top-left
    pub fn render_left_top(self) -> PositionedRichText {
        PositionedRichText::new(self.render(), Xy::zero())
    }

    /// Render and position at left with vertical centering
    pub fn render_left_center(self, height: Px) -> PositionedRichText {
        let rendered = self.render();
        let offset_y = (height - rendered.height) / 2.0;
        PositionedRichText::new(rendered, Xy::new(Px::zero(), offset_y))
    }

    /// Render and center in the given size
    pub fn render_center(self, wh: Wh<Px>) -> PositionedRichText {
        let rendered = self.render();
        let offset_x = (wh.width - rendered.width) / 2.0;
        let offset_y = (wh.height - rendered.height) / 2.0;
        PositionedRichText::new(rendered, Xy::new(offset_x, offset_y))
    }

    /// Render and position at right-top
    pub fn render_right_top(self, width: Px) -> PositionedRichText {
        let rendered = self.render();
        let offset_x = width - rendered.width;
        PositionedRichText::new(rendered, Xy::new(offset_x, Px::zero()))
    }
}

/// Convenience function for headline
pub fn headline() -> TypographyBuilder<'static> {
    TypographyBuilder::headline()
}

/// Convenience function for paragraph  
pub fn paragraph() -> TypographyBuilder<'static> {
    TypographyBuilder::paragraph()
}

#[derive(Debug, Clone)]
pub struct TypographyIcon {
    pub icon_kind: IconKind,
    pub attributes: Vec<IconAttribute>,
}
impl TypographyIcon {
    fn new(icon_kind: IconKind) -> Self {
        Self {
            icon_kind,
            attributes: Vec::new(),
        }
    }

    /// Add an attribute to the icon
    pub fn attribute(mut self, icon_kind: IconKind, position: IconAttributePosition) -> Self {
        self.attributes.push(IconAttribute {
            icon_kind,
            position,
        });
        self
    }
}
