pub mod hooks;
mod random;
mod render;
pub mod system;
pub mod utils;

mod ffi;

pub use self::random::*;
pub use anyhow::{Result, anyhow};
pub use auto_ops;
pub use futures::{StreamExt, future::join_all, future::try_join_all, join, try_join};
pub use hooks::*;
pub use namui_asset_macro::register_assets;
pub use namui_cfg::*;
pub use namui_rendering_tree::*;
pub use namui_type as types;
pub use namui_type::*;
pub use rand;
pub use render::*;
pub use shader_macro::shader;
use std::{
    cell::RefCell,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};
pub use system::*;
pub use tokio;
pub use tokio::task::{spawn, spawn_local};
pub use tracing;
pub use tracing::{Level, debug, error, info, instrument, span, trace, warn};

pub mod particle {
    pub use namui_particle::{Emitter, Particle, ParticleSprites, RenderEmitter};
}
thread_local! {
    static TOKIO_RUNTIME: RefCell<Option<tokio::runtime::Runtime>> =
        RefCell::new(Some(build_tokio_runtime()));
    static LOOPER: RefCell<Option<Looper>> = const { RefCell::new(None) };
    static RESPONSE_BUFFER: RefCell<Vec<u8>> = Default::default();
}

fn build_tokio_runtime() -> tokio::runtime::Runtime {
    #[cfg(target_arch = "wasm32")]
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build();
    #[cfg(not(target_arch = "wasm32"))]
    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(2 * 1024 * 1024)
        .max_blocking_threads(32)
        .build();
    result
        .map_err(|e| anyhow!("Failed to create tokio runtime: {:?}", e))
        .unwrap()
}

pub fn start(root_component: RootComponent) {
    system::log::init_log_plugin_with_default();
    LOOPER.set(Some(Looper::new(root_component)));
}

pub fn start_with_log_config(root_component: RootComponent, log_config: system::log::LogConfig) {
    system::log::init_log_plugin(log_config);
    LOOPER.set(Some(Looper::new(root_component)));
}

/// Write response data in `[len: u32 LE][data...]` format into RESPONSE_BUFFER.
/// Returns pointer to the buffer. Valid until the next call to write_response/write_empty_response.
pub(crate) fn write_response(data: &[u8]) -> *const u8 {
    RESPONSE_BUFFER.with_borrow_mut(|buf| {
        buf.clear();
        buf.reserve(4 + data.len());
        buf.extend_from_slice(&(data.len() as u32).to_le_bytes());
        buf.extend_from_slice(data);
        buf.as_ptr()
    })
}

/// Write empty response `[len: u32 = 0]`, signals "redraw with previous data".
pub(crate) fn write_empty_response() -> *const u8 {
    write_response(&[])
}

/// Returns null for no change, or pointer to `[len: u32 LE][data...]`.
/// - null: no change
/// - len == 0: mouse position changed, redraw with previous rendering tree
/// - len > 0: new rendering tree data
fn on_event(event: RawEvent) -> *const u8 {
    thread_local! {
        static PREV_SENT_TREE_BYTES: RefCell<Vec<u8>> = Default::default();
    }
    static MOUSE_POSITION_ON_LAST_REDRAW: AtomicU32 = AtomicU32::new(0);

    let is_screen_redraw = matches!(event, RawEvent::ScreenRedraw);

    let mut result: *const u8 = std::ptr::null();

    TOKIO_RUNTIME.with(|tokio_runtime| {
        let Some(ref runtime) = *tokio_runtime.borrow() else {
            return; // Already shut down
        };
        runtime.block_on(async {
            LOOPER.with_borrow_mut(|looper| {
                let rendering_tree = looper.as_mut().unwrap().tick(event);

                system::audio::flush_audio();

                if !is_screen_redraw {
                    return;
                }

                let bytes =
                    bincode::encode_to_vec(rendering_tree, bincode::config::standard()).unwrap();
                let tree_changed = PREV_SENT_TREE_BYTES.with_borrow(|prev| prev != &bytes);
                let mouse_position_changed = MOUSE_POSITION_ON_LAST_REDRAW.load(Ordering::Relaxed)
                    != system::mouse::mouse_position_u32();

                if tree_changed {
                    result = write_response(&bytes);
                    PREV_SENT_TREE_BYTES.replace(bytes);
                } else if mouse_position_changed {
                    result = write_empty_response();
                }

                MOUSE_POSITION_ON_LAST_REDRAW
                    .store(system::mouse::mouse_position_u32(), Ordering::Relaxed);
            });

            for _ in 0..16 {
                tokio::task::yield_now().await;
            }
        });
    });

    result
}

pub fn render(rendering_trees: impl IntoIterator<Item = RenderingTree>) -> RenderingTree {
    let mut iter = rendering_trees.into_iter();
    let first = 'outer: {
        for x in iter.by_ref() {
            if x != RenderingTree::Empty {
                break 'outer x;
            }
        }
        return RenderingTree::Empty;
    };
    let second = 'outer: {
        for x in iter.by_ref() {
            if x != RenderingTree::Empty {
                break 'outer x;
            }
        }
        return first;
    };

    let mut children = vec![first, second];
    children.extend(iter.filter(|x| *x != RenderingTree::Empty));
    RenderingTree::Children(arena_alloc_slice(children))
}

pub fn try_render(func: impl FnOnce() -> Option<RenderingTree>) -> RenderingTree {
    func().unwrap_or(RenderingTree::Empty)
}
