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
pub use futures::{future::join_all, future::try_join_all, join, try_join, StreamExt};
pub use hooks::*;
pub use lazy_static::lazy_static;
pub use namui_cfg::*;
pub use namui_skia::*;
pub use namui_type as types;
pub use namui_type::*;
pub use render::*;
pub use serde;
pub use shader_macro::shader;
#[cfg(target_os = "windows")]
pub use system::media::*;
pub use system::network::http::{RequestExt, ResponseExt};
pub use system::*;
pub use tokio;
pub use tokio::task::{spawn, spawn_local};

pub fn start(component: impl 'static + Fn(&RenderCtx) + Send) {
    namui_type::set_log(|x| log::log(x));

    let tokio_runtime: tokio::runtime::Runtime =
        tokio_runtime().expect("Failed to create tokio runtime");
    tokio_runtime.spawn(async move {
        system::init_system()
            .await
            .expect("Failed to initialize namui system");

        crate::log!("Namui system initialized");

        #[cfg(target_os = "wasi")]
        {
            crate::screen::run_event_hook_loop(component)
        }
    });

    #[cfg(target_os = "wasi")]
    {
        skia::on_skia_drawing_thread().unwrap();
    }
    #[cfg(not(target_os = "wasi"))]
    {
        tokio_runtime.block_on(async move {
            screen::take_main_thread(component);
        });
    }
}

fn tokio_runtime() -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(2 * 1024 * 1024)
        .build()
        .map_err(|e| anyhow!("Failed to create tokio runtime: {:?}", e))
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
