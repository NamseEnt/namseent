use namui::*;

/// Vertical alignment for inline boxes
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum VerticalAlign {
    #[default]
    Top,
    Middle,
    Bottom,
}

/// Style changes to apply (partial updates) - const-compatible
#[derive(Debug, Clone, Copy)]
pub struct StyleDelta {
    pub font_size: Option<IntPx>,
    pub color: Option<Color>,
    pub bold: Option<bool>,
    pub underline: Option<bool>,
    pub border: Option<TextStyleBorder>,
    pub vertical_align: Option<VerticalAlign>,
}

impl StyleDelta {
    /// Create empty style delta
    pub const fn empty() -> Self {
        Self {
            font_size: None,
            color: None,
            bold: None,
            underline: None,
            border: None,
            vertical_align: None,
        }
    }

    /// Create style delta with color
    pub const fn color(color: Color) -> Self {
        Self {
            font_size: None,
            color: Some(color),
            bold: None,
            underline: None,
            border: None,
            vertical_align: None,
        }
    }

    /// Create style delta with bold
    pub const fn bold() -> Self {
        Self {
            font_size: None,
            color: None,
            bold: Some(true),
            underline: None,
            border: None,
            vertical_align: None,
        }
    }

    /// Create style delta with font size
    pub const fn font_size(size: IntPx) -> Self {
        Self {
            font_size: Some(size),
            color: None,
            bold: None,
            underline: None,
            border: None,
            vertical_align: None,
        }
    }

    /// Create style delta with stroke (border)
    pub const fn stroke(width: Px, color: Color) -> Self {
        Self {
            font_size: None,
            color: None,
            bold: None,
            underline: None,
            border: Some(TextStyleBorder { color, width }),
            vertical_align: None,
        }
    }

    /// Create style delta with vertical alignment
    pub const fn vertical_align(align: VerticalAlign) -> Self {
        Self {
            font_size: None,
            color: None,
            bold: None,
            underline: None,
            border: None,
            vertical_align: Some(align),
        }
    }
}

/// Complete style context (full state)
#[derive(Debug, Clone)]
pub struct StyleContext {
    pub font_name: String,
    pub font_size: IntPx,
    pub color: Color,
    pub bold: bool,
    pub underline: bool,
    pub text_style: TextStyle,
    pub vertical_align: VerticalAlign,
}

impl StyleContext {
    /// Create new style context with defaults
    pub fn new(font_name: String, font_size: IntPx, color: Color, text_style: TextStyle) -> Self {
        Self {
            font_name,
            font_size,
            color,
            bold: false,
            underline: false,
            text_style,
            vertical_align: VerticalAlign::Middle,
        }
    }

    /// Apply a style delta to create new context
    pub fn apply_delta(&self, delta: StyleDelta) -> Self {
        let mut ctx = self.clone();
        if let Some(size) = delta.font_size {
            ctx.font_size = size;
        }
        if let Some(color) = delta.color {
            ctx.color = color;
        }
        if let Some(bold) = delta.bold {
            ctx.bold = bold;
        }
        if let Some(underline) = delta.underline {
            ctx.underline = underline;
        }
        if let Some(border) = delta.border {
            ctx.text_style.border = Some(border);
        }
        if let Some(vertical_align) = delta.vertical_align {
            ctx.vertical_align = vertical_align;
        }
        ctx
    }

    /// Convert to Font
    pub fn to_font(&self) -> Font {
        Font {
            name: self.font_name.clone(),
            size: self.font_size,
        }
    }

    /// Convert to TextStyle
    pub fn to_text_style(&self) -> TextStyle {
        let mut style = self.text_style.clone();
        style.color = self.color;
        style
    }
}

/// Style stack manages nested style contexts (Canvas-like save/restore pattern)
pub struct StyleStack {
    stack: Vec<StyleContext>,
}

impl StyleStack {
    /// Create new style stack with default context
    pub fn new(default_style: StyleContext) -> Self {
        Self {
            stack: vec![default_style],
        }
    }

    /// Push a new style context with applied delta (Deprecated: use save/apply_delta/restore pattern)
    #[deprecated(note = "Use save/apply_delta/restore pattern instead")]
    pub fn push(&mut self, delta: StyleDelta) {
        let current = self.current().clone();
        let new_style = current.apply_delta(delta);
        self.stack.push(new_style);
    }

    /// Pop the current style context (Deprecated: use save/restore pattern)
    #[deprecated(note = "Use save/restore pattern instead")]
    pub fn pop(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    /// Apply style delta to current style context (accumulates changes)
    pub fn apply_delta(&mut self, delta: StyleDelta) {
        if let Some(last) = self.stack.last_mut() {
            *last = last.apply_delta(delta);
        }
    }

    /// Save current style state (like canvas.save())
    pub fn save(&mut self) {
        if let Some(current) = self.stack.last() {
            self.stack.push(current.clone());
        }
    }

    /// Restore previously saved style state (like canvas.restore())
    pub fn restore(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    /// Get current style context
    pub fn current(&self) -> &StyleContext {
        self.stack.last().unwrap()
    }

    /// Get mutable reference to current style context
    pub fn current_mut(&mut self) -> &mut StyleContext {
        self.stack.last_mut().unwrap()
    }

    /// Get stack depth for debugging
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}
