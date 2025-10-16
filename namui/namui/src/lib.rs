mod common;
pub mod hooks;
pub mod math;
mod random;
mod render;
pub mod system;
pub mod utils;

pub use self::random::*;
pub use anyhow::anyhow;
pub use auto_ops;
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
    network::http::{RequestExt, ResponseExt},
    *,
};
pub use tokio;
pub use tokio::task::{spawn, spawn_local};
pub use url::Url;

pub mod particle {
    pub use namui_particle::{Emitter, Particle, System};
}
thread_local! {
    static TOKIO_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(2 * 1024 * 1024)
        .max_blocking_threads(32)
        .build()
        .map_err(|e| anyhow!("Failed to create tokio runtime: {:?}", e)).unwrap();
    static LOOPER: RefCell<Option<Looper>> = const { RefCell::new(None) };
    static RENDERING_TREE_BYTES: RefCell<Box<[u8]>> = Default::default();
    static FROZEN_STATES: RefCell<Box<[u8]>> = Default::default();
}

#[unsafe(no_mangle)]
extern "C" fn _init_system() {
    system::init_system().unwrap();
}

#[unsafe(no_mangle)]
extern "C" fn _freeze_world() -> u64 {
    let looper = LOOPER.with_borrow_mut(|looper| looper.take().unwrap());
    let frozen_states = looper.world.freeze_states();
    FROZEN_STATES.with_borrow_mut(|bytes| {
        *bytes = frozen_states.into_boxed_slice();
        (bytes.as_ptr() as u64) << 32 | bytes.len() as u64
    })
}

#[unsafe(no_mangle)]
extern "C" fn _set_freeze_states(ptr: *const u8, len: usize) {
    LOOPER.with_borrow_mut(|looper| {
        looper
            .as_mut()
            .unwrap()
            .world
            .set_frozen_states(unsafe { std::slice::from_raw_parts(ptr, len) });
    });
}

pub fn start(root_component: RootComponent) {
    LOOPER.set(Some(Looper::new(root_component)));
}

fn on_event(event: RawEvent) -> u64 {
    let mut out_ptr = 0;
    let mut out_len_ptr = 0;
    TOKIO_RUNTIME.with(|tokio_runtime| {
        let _guard = tokio_runtime.enter();

        LOOPER.with_borrow_mut(|looper| {
            let Some(rendering_tree) = looper.as_mut().unwrap().tick(event) else {
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

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $println!::log(format!($($arg)*));
    }}
}
