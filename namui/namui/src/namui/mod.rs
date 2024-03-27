mod bounding_box;
mod common;
pub(crate) mod hooks;
pub mod math;
mod random;
mod render;
pub mod system;
pub mod utils;

pub use self::random::*;
pub use ::url::Url;
pub use anyhow::{anyhow, bail, Result};
pub use auto_ops;
pub use bounding_box::*;
#[cfg(target_family = "wasm")]
pub use clipboard::ClipboardItem as _;
pub use common::*;
pub use futures::{future::join_all, future::try_join_all, join, try_join};
pub use hooks::*;
pub use hooks_macro::*;
pub use lazy_static::lazy_static;
pub use namui_cfg::*;
pub use namui_skia::*;
pub use namui_type as types;
pub use namui_type::*;
pub use render::*;
#[cfg(target_family = "wasm")]
pub use render::{text_input, TextInput, TextInputInstance};
pub use serde;
pub use shader_macro::shader;
pub use system::media::{FullLoadOnceAudio, MediaHandle};
pub use system::*;

#[cfg(not(target_family = "wasm"))]
pub use tokio::task::spawn;
#[cfg(target_family = "wasm")]
pub use wasm_bindgen_futures::spawn_local as spawn;

#[cfg(not(target_family = "wasm"))]
pub use tokio::task::spawn_blocking;

pub fn start<C: Component>(component: impl Send + Sync + Fn() -> C + 'static) {
    namui_type::set_log(|x| log::log(x));

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                system::init_system()
                    .await
                    .expect("Failed to initialize namui system");

                crate::log!("Namui system initialized");

                tokio::task::block_in_place(|| hooks::run_loop(component));
            })
    });

    system::take_main_thread();
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
