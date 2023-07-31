pub mod animation;
mod common;
pub(crate) mod draw;
pub mod hooks;
pub mod math;
mod namui_context;
mod random;
mod render;
mod skia;
pub mod system;
pub mod utils;

pub use self::random::*;
pub use ::url::Url;
pub use hooks::*;
// pub use audio::Audio;
use anyhow::Result;
pub use auto_ops;
pub use clipboard::ClipboardItem as _;
pub use common::*;
pub use draw::{DrawCall, DrawCommand, PathDrawCommand, TextAlign, TextBaseline, TextDrawCommand};
pub use lazy_static::lazy_static;
pub use namui_cfg::*;
pub use namui_context::NamuiContext;
pub use namui_type as types;
pub use namui_type::*;
pub use render::{
    absolute, clip, draw_rendering_tree, handle_web_event, image::*, on_top, path::*, rect::*,
    rotate, scale, text::*, text_input, transform, translate, AttachEventBuilder, FileDropEvent,
    ImageSource, KeyboardEvent, Matrix3x3, MouseCursor, MouseEvent, MouseEventCallback,
    MouseEventType, RenderingData, RenderingTree, WheelEvent, WheelEventCallback,
};
pub use serde;
pub use shader_macro::shader;
pub use skia::{
    make_runtime_effect_shader, BlendMode, ClipOp, Color, FilterMode, Font, Image, MipmapMode,
    PaintBuilder, PaintStyle, PathBuilder, Shader, StrokeCap, StrokeJoin, TileMode, Typeface,
};
pub(crate) use skia::{ColorFilter, Paint, Path};
use std::sync::Arc;
pub use system::*;

#[cfg(target_family = "wasm")]
pub use wasm_bindgen_futures::spawn_local;

pub async fn init() -> NamuiContext {
    let namui_context = NamuiContext::new();

    system::init()
        .await
        .expect("Failed to initialize namui system");

    namui_context
}

pub fn start(namui_context: NamuiContext, component: &dyn Component) {
    namui_context.start(component);
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}

// /// `now()` is not ISO 8601. It's time since the program started.
pub fn now() -> Time {
    system::time::now()
}
