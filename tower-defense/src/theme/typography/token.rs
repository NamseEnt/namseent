use crate::theme::typography::TypographyIcon;

/// Token system for rich text rendering
/// Supports both static (const-compatible) and dynamic (runtime-allocated) tokens
/// Rich text token - supports both static and dynamic content
#[derive(Debug, Clone)]
pub enum Token<'a> {
    /// Static text content (const-compatible)
    Text(&'a str),
    /// Apply style delta to current style (accumulates changes)
    ApplyStyle(super::style::StyleDelta),
    /// Save current style state (like canvas.save())
    Save,
    /// Restore previously saved style state (like canvas.restore())
    Restore,
    /// Static icon by name (scales with current font size)
    Icon(TypographyIcon),
    /// Space character
    Space,
    /// Hard line break
    LineBreak,
}
