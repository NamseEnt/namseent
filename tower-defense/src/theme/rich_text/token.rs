/// Token system for rich text rendering
/// Supports both static (const-compatible) and dynamic (runtime-allocated) tokens
/// Rich text token - supports both static and dynamic content
#[derive(Debug, Clone)]
pub enum Token<'a> {
    /// Static text content (const-compatible)
    Text(&'a str),
    /// Dynamic text content (runtime-allocated)
    DynamicText(String),
    /// Push style changes (enters a new scope)
    PushStyle(super::style::StyleDelta),
    /// Pop style changes (exits current scope)
    PopStyle,
    /// Static icon by name (scales with current font size)
    Icon(&'a str),
    /// Dynamic icon by name (runtime-allocated)
    DynamicIcon(String),
    /// Space character
    Space,
    /// Hard line break
    LineBreak,
}
