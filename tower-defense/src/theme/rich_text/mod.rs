//! Tower Defense Rich Text Implementation
//!
//! Token-based rendering system with style stack management.
//! Supports both static (const-compatible) and dynamic (runtime) tokens.
//!
//! # Architecture
//!
//! - **token**: Token types with static and dynamic variants
//! - **style**: Style system with stack-based context management
//! - **inline_box**: Text measurement and inline element types
//! - **layout**: Layout engine for text wrapping and alignment
//! - **renderer**: Core rendering logic for tokens

mod inline_box;
mod layout;
mod renderer;
mod style;
mod token;

// Re-export public API
pub use inline_box::{InlineBox, LineBox, PositionedInlineBox, ShapedText};
pub use layout::{LayoutConfig, LayoutEngine};
pub use renderer::{RenderedRichText, RichTextRenderer};
pub use style::{StyleContext, StyleDelta, StyleStack};
pub use token::Token;
