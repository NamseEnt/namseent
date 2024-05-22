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
// #[cfg(target_family = "wasm")]
// pub use clipboard::ClipboardItem as _;
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
// #[cfg(target_family = "wasm")]
// pub use render::{text_input, TextInput, TextInputInstance};
pub use serde;
pub use shader_macro::shader;
#[cfg(not(target_family = "wasm"))]
pub use system::media::*;
pub use system::*;

#[cfg(not(target_family = "wasm"))]
pub use tokio::task::spawn;
#[cfg(target_family = "wasm")]
pub fn spawn<F>(future: F) -> FakeJoinHandle
where
    F: std::future::Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
    FakeJoinHandle
}

#[cfg(target_family = "wasm")]
mod join_handle {
    pub struct FakeJoinHandle;

    impl FakeJoinHandle {
        /// NOTE: This method does nothing in wasm
        pub fn abort(self) {}
    }
}

#[cfg(target_family = "wasm")]
pub use join_handle::*;

#[cfg(not(target_family = "wasm"))]
pub use tokio::task::spawn_blocking;
#[cfg(target_family = "wasm")]
/// WARNING: spawn_blocking in wasm will block the main thread
pub async fn spawn_blocking<F, R>(f: F) -> Result<R>
where
    F: FnOnce() -> R,
{
    Ok(f())
}

pub fn start(component: impl 'static + Fn(&RenderCtx)) {
    namui_type::set_log(|x| log::log(x));

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
    wasm_bindgen_futures::spawn_local(fut)
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
