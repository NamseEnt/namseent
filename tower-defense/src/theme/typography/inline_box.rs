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
    pub vertical_align: super::style::VerticalAlign,
}

impl ShapedText {
    /// Shape text and measure its dimensions
    pub fn shape(
        text: String,
        font: Font,
        style: TextStyle,
        vertical_align: super::style::VerticalAlign,
    ) -> Self {
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
            vertical_align,
        }
    }

    /// Split text to fit within max_width
    /// Returns the first part that fits and the remaining part
    /// Returns (fits, remaining) where fits is text that fits in max_width
    /// and remaining is the rest
    pub fn split_to_fit(&self, max_width: Px) -> Option<(Self, Self)> {
        if self.width <= max_width {
            return None; // Fits already
        }

        // Binary search for the longest text that fits
        let text_len = self.text.chars().count();
        let mut left = 0;
        let mut right = text_len;

        while left < right {
            let mid = (left + right).div_ceil(2);
            let substring: String = self.text.chars().take(mid).collect();
            let shaped = Self::shape(
                substring,
                self.font.clone(),
                self.style.clone(),
                self.vertical_align,
            );

            if shaped.width <= max_width {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        if left == 0 {
            return None; // Can't fit a single character
        }

        let fits_str: String = self.text.chars().take(left).collect();
        let remaining_str: String = self.text.chars().skip(left).collect();

        let fits = Self::shape(
            fits_str,
            self.font.clone(),
            self.style.clone(),
            self.vertical_align,
        );
        let remaining = Self::shape(
            remaining_str,
            self.font.clone(),
            self.style.clone(),
            self.vertical_align,
        );

        Some((fits, remaining))
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
        vertical_align: super::style::VerticalAlign,
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

    pub fn vertical_align(&self) -> super::style::VerticalAlign {
        match self {
            InlineBox::Text(shaped) => shaped.vertical_align,
            InlineBox::Atomic { vertical_align, .. } => *vertical_align,
            InlineBox::SoftBreak | InlineBox::HardBreak | InlineBox::Space { .. } => {
                super::style::VerticalAlign::Top
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

    /// Try to split the inline box to fit within max_width
    /// Returns (fits, remaining) if split is possible
    pub fn split_to_fit(&self, max_width: Px) -> Option<(Self, Self)> {
        match self {
            InlineBox::Text(shaped) => {
                shaped
                    .split_to_fit(max_width)
                    .map(|(fits_shaped, remaining_shaped)| {
                        (
                            InlineBox::Text(fits_shaped),
                            InlineBox::Text(remaining_shaped),
                        )
                    })
            }
            _ => None, // Only text boxes can be split
        }
    }
}

/// Positioned inline box within a line
#[derive(Debug, Clone)]
pub struct PositionedInlineBox {
    pub inline_box: InlineBox,
    pub x: Px,
    pub y: Px,
    pub text_baseline: TextBaseline,
}

/// Line box containing positioned inline boxes
#[derive(Debug, Clone)]
pub struct LineBox {
    pub boxes: Vec<PositionedInlineBox>,
    pub content_width: Px,
    pub height: Px,
    pub baseline: Px,
}
