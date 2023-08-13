mod bounding_box;
mod common;
pub mod hooks;
pub mod math;
mod namui_context;
mod random;
mod render;
pub mod system;
pub mod utils;

// pub use audio::Audio;
pub use self::random::*;
pub use ::url::Url;
pub use anyhow::{anyhow, bail, Result};
pub use auto_ops;
pub use bounding_box::*;
pub use clipboard::ClipboardItem as _;
pub use common::*;
pub use hooks::*;
pub use lazy_static::lazy_static;
pub use namui_cfg::*;
pub use namui_context::NamuiContext;
pub use namui_type as types;
pub use namui_type::*;
pub use render::{image::*, path::*, rect::*, text::*, text_input, TextInput, TextInputInstance};
pub use serde;
pub use shader_macro::shader;
pub use system::*;
pub use wasm_bindgen_futures::spawn_local;

pub async fn init() -> NamuiContext {
    let namui_context = NamuiContext::new();

    system::init()
        .await
        .expect("Failed to initialize namui system");

    crate::log!("Namui system initialized");

    namui_context
}

pub async fn start<C: Component>(
    namui_context: NamuiContext,
    component: impl Send + Sync + Fn() -> C + 'static,
) {
    namui_context.start(component).await;
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
