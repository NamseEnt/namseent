pub(crate) mod channel;
mod clipping;
mod component;
mod ctx;
mod event;
mod instance;
mod key;
mod macros;
mod native;
mod sig;
mod value;

use crate::RawEvent;
pub(crate) use channel::*;
pub(crate) use clipping::*;
pub use component::*;
pub use ctx::*;
pub(crate) use event::*;
pub use hooks_macro::*;
pub(crate) use instance::*;
use key::*;
pub use macros::*;
pub use native::*;
pub use sig::*;
use std::{
    any::TypeId,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex, OnceLock},
};
pub use value::*;

static TREE_CTX: OnceLock<TreeContext> = OnceLock::new();
pub(crate) async fn start<C: Component>(root_component: impl Send + Sync + 'static + Fn() -> C) {
    TREE_CTX.set(TreeContext::new(root_component)).unwrap();

    TREE_CTX.get().unwrap().start().await;

    #[cfg(not(target_family = "wasm"))]
    crate::system::screen::await_event_loop_join().await;
}

pub(crate) fn on_raw_event(event: RawEvent) {
    if let Some(ctx) = TREE_CTX.get() {
        ctx.on_raw_event(event)
    }
}

pub(crate) fn render_and_draw() {
    if let Some(ctx) = TREE_CTX.get() {
        ctx.render_and_draw()
    }
}

pub(crate) fn stop_event_propagation() {
    if let Some(ctx) = TREE_CTX.get() {
        ctx.stop_event_propagation()
    }
}
