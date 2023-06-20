pub mod animation;
mod common;
pub(crate) mod draw;
pub mod event;
pub mod math;
mod namui_context;
mod random;
mod render;
mod skia;
pub mod system;
pub mod utils;

pub use self::random::*;
pub use ::url::Url;
pub use audio::Audio;
pub use auto_ops;
pub use clipboard::ClipboardItem as _;
pub use common::*;
pub use draw::{DrawCall, DrawCommand, PathDrawCommand, TextAlign, TextBaseline, TextDrawCommand};
pub use event::{Event, NamuiEvent};
pub use lazy_static::lazy_static;
pub use namui_cfg::*;
pub use namui_context::NamuiContext;
pub use namui_type as types;
pub use namui_type::*;
pub use render::{
    absolute, clip, image::*, on_top, path::*, react, rect::*, rotate, scale, text::*, text_input,
    transform, translate, AttachEventBuilder, FileDropEvent, ImageSource, KeyDownEvent,
    KeyboardEvent, Matrix3x3, MouseCursor, MouseEvent, MouseEventCallback, MouseEventType, React,
    RenderingData, RenderingTree, TextInput, WheelEvent, WheelEventCallback,
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

pub trait Entity {
    type Props;
    fn update(&mut self, event: &Event);
    fn render(&self, props: &Self::Props) -> RenderingTree;
}

pub async fn init() -> NamuiContext {
    let event_receiver = event::init();

    system::init()
        .await
        .expect("Failed to initialize namui system");

    let namui_context = NamuiContext::new(event_receiver);

    namui_context
}

pub async fn start<TProps>(
    namui_context: NamuiContext,
    state: &mut dyn Entity<Props = TProps>,
    props: &TProps,
) {
    namui_context.start(state, props).await;
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
