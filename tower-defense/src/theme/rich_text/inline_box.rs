use namui::*;

/// Measured text with typographic metrics
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
    /// Shape text and measure its dimensions
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

/// Inline box element (can be laid out)
#[derive(Debug, Clone)]
pub enum InlineBox {
    /// Text element with measured metrics
    Text(ShapedText),
    /// Embedded atomic content (icon, image)
    Atomic {
        content: RenderingTree,
        width: Px,
        height: Px,
        baseline: Px,
    },
    /// Soft break (wrap opportunity)
    SoftBreak,
    /// Hard break (explicit newline)
    HardBreak,
    /// Space element
    Space { width: Px },
}

impl InlineBox {
    pub fn width(&self) -> Px {
        match self {
            InlineBox::Text(shaped) => shaped.width,
            InlineBox::Atomic { width, .. } => *width,
            InlineBox::SoftBreak | InlineBox::HardBreak => 0.px(),
            InlineBox::Space { width } => *width,
        }
    }

    pub fn height(&self) -> Px {
        match self {
            InlineBox::Text(shaped) => shaped.height,
            InlineBox::Atomic { height, .. } => *height,
            InlineBox::SoftBreak | InlineBox::HardBreak => 0.px(),
            InlineBox::Space { .. } => 0.px(),
        }
    }

    pub fn baseline(&self) -> Px {
        match self {
            InlineBox::Text(shaped) => shaped.ascent,
            InlineBox::Atomic { baseline, .. } => *baseline,
            InlineBox::SoftBreak | InlineBox::HardBreak => 0.px(),
            InlineBox::Space { .. } => 0.px(),
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

/// Positioned inline box within a line
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
}
