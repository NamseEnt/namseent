mod common;
pub mod hooks;
pub mod math;
mod random;
mod render;
pub mod system;
pub mod utils;

pub use self::random::*;
pub use anyhow::{Result, anyhow};
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
use std::{
    cell::RefCell,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};
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

/// Return values
///
/// - 0: No rendering tree changed
/// - u64::MAX: Redraw with previous rendering tree
/// - Other: Rendering tree changed. ptr 32bit, len 32bit
fn on_event(event: RawEvent) -> u64 {
    thread_local! {
        static RENDERING_TREE: RefCell<RenderingTree> = Default::default();
        static RENDERING_TREE_BYTES: RefCell<Box<[u8]>> = Default::default();
    }
    static RENDERING_TREE_CHANGED: AtomicBool = AtomicBool::new(false);
    static MOUSE_POSITION_ON_LAST_REDRAW: AtomicU32 = AtomicU32::new(0);

    let is_screen_redraw = matches!(event, RawEvent::ScreenRedraw);

    let mut out_ptr = 0;
    let mut out_len_ptr = 0;

    TOKIO_RUNTIME.with(|tokio_runtime| {
        let _guard = tokio_runtime.enter();

        LOOPER.with_borrow_mut(|looper| {
            let rendering_tree = looper.as_mut().unwrap().tick(event);

            if !RENDERING_TREE_CHANGED.load(Ordering::Relaxed) {
                RENDERING_TREE_CHANGED.store(
                    RENDERING_TREE
                        .with_borrow(|prev_rendering_tree| prev_rendering_tree != &rendering_tree),
                    Ordering::Relaxed,
                );
            }

            RENDERING_TREE.replace(rendering_tree);

            if !is_screen_redraw {
                return;
            }

            let rendering_tree_changed = RENDERING_TREE_CHANGED.load(Ordering::Relaxed);
            let mouse_position_changed = MOUSE_POSITION_ON_LAST_REDRAW.load(Ordering::Relaxed)
                != system::mouse::mouse_position_u32();

            if rendering_tree_changed {
                let bytes = RENDERING_TREE.with_borrow(|rendering_tree| {
                    bincode::encode_to_vec(rendering_tree, bincode::config::standard())
                        .unwrap()
                        .into_boxed_slice()
                });

                let len = bytes.len();

                RENDERING_TREE_BYTES.replace(bytes);

                out_ptr = RENDERING_TREE_BYTES.with_borrow(|bytes| bytes.as_ptr() as usize);
                out_len_ptr = len;
            } else if mouse_position_changed {
                out_ptr = usize::MAX;
                out_len_ptr = usize::MAX;
            }

            MOUSE_POSITION_ON_LAST_REDRAW
                .store(system::mouse::mouse_position_u32(), Ordering::Relaxed);
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
