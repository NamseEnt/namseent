pub(crate) mod channel;
mod component;
mod instance;
mod key;
mod native;
mod render;
mod sig;
mod tree;
mod value;

pub(crate) use channel::*;
pub use component::*;
pub use hooks_macro::*;
pub(crate) use instance::*;
use key::*;
pub use native::*;
pub use render::*;
pub use render::*;
pub use sig::*;
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex, OnceLock},
};
pub(crate) use tree::*;
pub use value::*;

static TREE_CTX: OnceLock<TreeContext> = OnceLock::new();
pub(crate) async fn start<C: Component>(root_component: impl Send + Sync + 'static + Fn() -> C) {
    TREE_CTX.set(TreeContext::new(root_component)).unwrap();

    TREE_CTX.get().unwrap().start().await;
}

pub(crate) fn on_raw_event(event: RawEvent) {
    TREE_CTX.get().map(|ctx| ctx.on_raw_event(event));
}

pub(crate) fn render_and_draw() {
    TREE_CTX.get().map(|ctx| ctx.render_and_draw());
}

pub fn boxed<'a, T: 'a>(value: T) -> Box<T> {
    Box::new(value)
}

/// callback!('a, A)
/// -> &'a (dyn 'a + Fn(A))
#[macro_export]
macro_rules! callback {
    ($lifetime: lifetime, $param: ty) => {
        // &$lifetime (dyn $lifetime + Fn($param))
        Box<dyn $lifetime + FnOnce($param)>
    };
    ($lifetime: lifetime) => {
        // &$lifetime (dyn $lifetime + Fn())
        Box<dyn $lifetime + FnOnce()>
    };
}

use crate::RawEvent;
pub use callback;
