mod common;
pub mod hooks;
pub mod math;
mod random;
mod render;
pub mod system;
pub mod utils;

pub use self::random::*;
pub use ::anyhow::{self, Result, anyhow, bail};
pub use ::url::Url;
pub use auto_ops;
use bytes::BufMut;
pub use common::*;
pub use futures::{StreamExt, future::join_all, future::try_join_all, join, try_join};
pub use hooks::*;
pub use namui_asset_macro::register_assets;
pub use namui_cfg::*;
pub use namui_rendering_tree::*;
pub use namui_type as types;
pub use namui_type::*;
pub use orx_parallel::*;
pub use rand;
pub use render::*;
pub use serde;
pub use shader_macro::shader;
use std::cell::RefCell;
pub use system::{
    // audio::Audio,
    network::http::{RequestExt, ResponseExt},
    *,
};
pub use tokio;
pub use tokio::task::{spawn, spawn_local};

pub mod particle {
    pub use namui_particle::{Emitter, Particle, System};
}
thread_local! {
    static TOKIO_RUNTIME: tokio::runtime::Runtime = tokio_runtime().unwrap();
    static LOOPER: RefCell<Option<Looper>> = const { RefCell::new(None) };
    static RENDERING_TREE_BYTES: RefCell<Box<[u8]>> = Default::default();
}

#[unsafe(no_mangle)]
extern "C" fn _init_system() {
    system::init_system().unwrap();
}

pub fn start(root_component: RootComponent) {
    LOOPER.set(Some(Looper::new(root_component)));
}

fn on_event(event: RawEvent) -> u64 {
    let mut out_ptr = 0;
    let mut out_len_ptr = 0;
    TOKIO_RUNTIME.with(|tokio_runtime| {
        let _guard = tokio_runtime.enter();

        LOOPER.with_borrow_mut(|looper_cell| unsafe {
            let Some(rendering_tree) = looper_cell.as_mut().unwrap().tick(event) else {
                return;
            };

            RENDERING_TREE_BYTES.with_borrow_mut(|bytes| {
                *bytes = bincode::encode_to_vec(rendering_tree, bincode::config::standard())
                    .unwrap()
                    .into_boxed_slice();

                let len = bytes.len();
                out_ptr = bytes.as_ptr() as usize;
                out_len_ptr = len;
            })
        })
    });

    (out_ptr as u64) << 32 | out_len_ptr as u64
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
