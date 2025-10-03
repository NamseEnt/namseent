mod bounding_box;
mod common;
pub mod hooks;
pub mod math;
mod random;
mod render;
pub mod system;
pub mod utils;

pub use self::random::*;
pub use ::anyhow::{self, Result, anyhow, bail};
pub use auto_ops;
pub use bounding_box::*;
pub use common::*;
pub use futures::{StreamExt, future::join_all, future::try_join_all, join, try_join};
pub use hooks::*;
pub use lazy_static::lazy_static;
pub use namui_cfg::*;
pub use namui_skia::*;
pub use namui_type as types;
pub use namui_type::*;
pub use rand;
pub use rayon;
pub use render::*;
pub use serde;
use std::{
    cell::RefCell,
    ptr::null_mut,
    sync::{Once, OnceLock, atomic::AtomicPtr},
};
pub use system::{
    network::http::{RequestExt, ResponseExt},
    *,
};
pub use tokio;
pub use tokio::task::spawn_local;

pub mod particle {
    pub use namui_particle::{Emitter, Particle, System, fire_and_forget};
}

static COMPONENT: OnceLock<Box<dyn Fn(&RenderCtx) + Send + Sync + 'static>> = OnceLock::new();
static TOKIO_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::init(crate::Instant::now));
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    TOKIO_RUNTIME.get().unwrap().spawn(future)
}

pub fn start<Root: Fn(&RenderCtx) + Send + Sync + 'static>(component: Root) -> Result<()> {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        setup_rayon_concurrency().unwrap();
    });

    let runtime = TOKIO_RUNTIME.get_or_init(|| tokio_runtime().unwrap());
    let _guard = runtime.enter();

    COMPONENT
        .set(Box::new(component))
        .map_err(|_| anyhow!("Component already initialized"))?;

    let component = COMPONENT.get().unwrap();
    let internal_root = InternalRoot::new(component);
    WORLD.with(|world_cell| {
        let mut world = world_cell.borrow_mut();
        let rendering_tree = world.run(&internal_root);
        println!("rendering_tree: {:?}", rendering_tree);
        let bytes = bincode::encode_to_vec(rendering_tree, bincode::config::standard()).unwrap();
    });
    Ok(())
}
unsafe extern "C" {
    fn _hardware_concurrency() -> u32;
}

fn setup_rayon_concurrency() -> Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(unsafe { _hardware_concurrency() } as usize)
        .build_global()?;
    Ok(())
}

fn tokio_runtime() -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(2 * 1024 * 1024)
        .worker_threads(unsafe { _hardware_concurrency() } as usize)
        .max_blocking_threads(unsafe { _hardware_concurrency() } as usize)
        .build()
        .map_err(|e| anyhow!("Failed to create tokio runtime: {:?}", e))
}

thread_local! {
    static LAST_RENDERING_TREE_BYTES: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}
#[unsafe(no_mangle)]
pub extern "C" fn _get_last_rendering_tree_bytes_ptr() -> *const u8 {
    LAST_RENDERING_TREE_BYTES.with(|cell| cell.borrow().as_ptr())
}
#[unsafe(no_mangle)]
pub extern "C" fn _get_last_rendering_tree_bytes_len() -> usize {
    LAST_RENDERING_TREE_BYTES.with(|cell| cell.borrow().len())
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_event() {
    let runtime = TOKIO_RUNTIME.get().unwrap();
    let _guard = runtime.enter();

    WORLD.with(|world_cell| {
        let mut world = world_cell.borrow_mut();
        let component = COMPONENT
            .get()
            .expect("Component not initialized. Call start() first.");
        let internal_root = InternalRoot::new(component);
        let rendering_tree = world.run(&internal_root);

        LAST_RENDERING_TREE_BYTES.with(|cell| {
            let mut bytes = cell.borrow_mut();
            *bytes = bincode::encode_to_vec(rendering_tree, bincode::config::standard()).unwrap();
        });
    })
}
