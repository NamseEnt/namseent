mod bounding_box;
mod common;
pub mod hooks;
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
pub use common::*;
pub use futures::{future::join_all, future::try_join_all, join, try_join};
pub use hooks::*;
pub use hooks_macro::*;
pub use lazy_static::lazy_static;
pub use namui_cfg::*;
pub use namui_skia::*;
pub use namui_tokio as tokio;
pub use namui_type as types;
pub use namui_type::*;
pub use render::*;
pub use serde;
pub use shader_macro::shader;
#[cfg(not(target_family = "wasm"))]
pub use system::media::*;
pub use system::*;
#[cfg(not(target_family = "wasm"))]
pub use tokio::task::spawn;
// pub use tokio::task::spawn_blocking;
#[cfg(target_family = "wasm")]
pub use tokio::task::spawn_local as spawn;
// #[cfg(target_family = "wasm")]
// pub use clipboard::ClipboardItem as _;
// #[cfg(target_family = "wasm")]
// pub use render::{text_input, TextInput, TextInputInstance};

pub fn start(component: impl 'static + Fn(&RenderCtx)) {
    namui_type::set_log(|x| log::log(x));

    std::thread::spawn(|| {
        println!("hi")
    });

    spawn_runtime(async move {
        system::init_system()
            .await
            .expect("Failed to initialize namui system");

        crate::log!("Namui system initialized");

        hooks::run_loop(component);
    });

    system::take_main_thread();
}

#[cfg(not(target_family = "wasm"))]
fn spawn_runtime(fut: impl std::future::Future<Output = ()> + 'static) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(fut)
}

#[cfg(target_family = "wasm")]
fn spawn_runtime(fut: impl std::future::Future<Output = ()> + 'static) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(fut)
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
