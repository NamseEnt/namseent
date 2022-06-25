mod common;
pub(crate) mod draw;
mod font;
mod manager;
use std::{any::Any, sync::Arc, time::Duration};
mod namui_state;
mod skia;
pub use common::*;
pub use draw::{DrawCall, DrawCommand, PathDrawCommand, TextAlign, TextBaseline, TextDrawCommand};
use futures::future::join;
pub use render::{
    absolute, clip, image::*, path::*, rect::*, rotate, scale, text::*, text_input, translate,
    types::*, ImageSource, MouseCursor, MouseEvent, MouseEventCallback, MouseEventType,
    RenderingData, RenderingTree, TextInput, WheelEventCallback,
};
pub use skia::{
    types::{ClipOp, Color, PaintStyle, StrokeJoin},
    BlendMode, Font, Image, LtrbRect, PaintBuilder, PathBuilder, StrokeCap, Typeface,
};
pub(crate) use skia::{ColorFilter, Paint, Path};
pub mod event;
pub use event::NamuiEvent;
mod render;
pub use self::manager::{managers, Code};
use self::{
    font::*,
    namui_state::{get_namui_state, NamuiState},
};
mod random;
pub use self::random::*;
pub mod screen;
pub use namui_cfg::*;
pub mod fs;
pub mod math;
pub use url::Url;
mod namui_context;
pub use namui_context::NamuiContext;

#[cfg(not(test))]
#[cfg(target_family = "wasm")]
mod namui_web;
#[cfg(not(test))]
#[cfg(target_family = "wasm")]
pub use self::namui_web::*;

#[cfg(test)]
mod namui_mock;
#[cfg(test)]
pub use self::namui_mock::*;

pub trait Entity {
    type Props;
    fn update(&mut self, event: &dyn Any);
    fn render(&self, props: &Self::Props) -> RenderingTree;
}

pub async fn init() -> NamuiContext {
    let mut namui_context = Namui::init();

    join(init_font(&mut namui_context), init_filesystem()).await;

    namui_context
}

pub async fn start<TProps>(
    mut namui_context: NamuiContext,
    state: &mut dyn Entity<Props = TProps>,
    props: &TProps,
) {
    namui_context.start(state, props).await;
}

async fn init_font(namui_context: &mut NamuiContext) {
    match load_all_fonts(namui_context, &managers().font_manager.typeface_manager).await {
        Ok(()) => {
            log("Font loaded".to_string());
        }
        Err(e) => {
            log(format!("Font loading failed: {}", e));
        }
    };
}

async fn init_filesystem() {
    match fs::init().await {
        Ok(()) => {
            log("Filesystem initialized".to_string());
        }
        Err(e) => {
            log(format!("Filesystem initialize failed: {:?}", e));
        }
    };
}

fn on_frame() {
    event::send(NamuiEvent::AnimationFrame);

    Namui::request_animation_frame(Box::new(move || {
        on_frame();
    }));
}

pub fn state() -> Arc<NamuiState> {
    get_namui_state()
}

pub fn log(format: String) {
    Namui::log(format);
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*));
    }}
}

/// `now()` is not ISO 8601. It's time since the program started.
pub fn now() -> Duration {
    Namui::now()
}
