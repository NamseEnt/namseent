mod common;
pub mod hooks;
pub mod math;
mod random;
mod render;
pub mod system;
pub mod utils;

use std::sync::OnceLock;

pub use self::random::*;
pub use ::anyhow::{self, Result, anyhow, bail};
pub use ::url::Url;
pub use auto_ops;
pub use common::*;
pub use futures::{StreamExt, future::join_all, future::try_join_all, join, try_join};
pub use hooks::*;
pub use lazy_static::lazy_static;
pub use namui_asset_macro::register_assets;
pub use namui_cfg::*;
pub use namui_rendering_tree::*;
pub use namui_type as types;
pub use namui_type::*;
pub use rand;
pub use rayon;
pub use render::*;
pub use serde;
pub use shader_macro::shader;
use std::cell::RefCell;
pub use system::{
    audio::Audio,
    network::http::{RequestExt, ResponseExt},
    *,
};
pub use tokio;
pub use tokio::task::{spawn, spawn_local};

pub mod particle {
    pub use namui_particle::{Emitter, Particle, System};
}

static TOKIO_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

thread_local! {
    static LOOPER: RefCell<Option<Looper>> = const { RefCell::new(None) };
}

pub fn start(root_component: RootComponent) {
    LOOPER.with(|looper| looper.replace(Some(Looper::new(root_component))));

    let tokio_runtime = TOKIO_RUNTIME.get_or_init(|| tokio_runtime().unwrap());

    tokio_runtime.spawn(async move {
        system::init_system()
            .await
            .expect("Failed to initialize namui system");

        println!("Namui system initialized");

        #[cfg(target_os = "wasi")]
        {
            // crate::screen::run_event_hook_loop(component)
        }
    });

    #[cfg(target_os = "wasi")]
    {
        // skia::on_skia_drawing_thread().unwrap();
    }
    // #[cfg(not(target_os = "wasi"))]
    // {
    //     tokio_runtime.block_on(async move {
    //         screen::take_main_thread(component);
    //     });
    // }
}

fn tokio_runtime() -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(2 * 1024 * 1024)
        .max_blocking_threads(32)
        .build()
        .map_err(|e| anyhow!("Failed to create tokio runtime: {:?}", e))
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $println!::log(format!($($arg)*));
    }}
}
