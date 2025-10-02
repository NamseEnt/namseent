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
use std::sync::OnceLock;
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

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    TOKIO_RUNTIME.get().unwrap().spawn(future)
}

pub fn start<Root: Fn(&RenderCtx) + Send + Sync + 'static>(component: Root) -> Result<()> {
    COMPONENT.set(Box::new(component));
    TOKIO_RUNTIME.set(tokio_runtime()?);
    let runtime = TOKIO_RUNTIME.get().unwrap();
    let _guard = runtime.enter();
    Looper::new(COMPONENT.get().unwrap());
    println!("looper new done");

    setup_rayon_concurrency()?;
    println!("after setup_rayon_concurrency");
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
