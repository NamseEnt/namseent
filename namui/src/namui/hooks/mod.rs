pub(crate) mod channel;
mod component;
mod event;
pub mod hooks;
mod instance;
mod native;
mod sig;
mod tree;
mod value;

pub(crate) use channel::*;
pub use component::*;
pub use event::*;
pub use hooks::*;
pub use hooks_macro::*;
pub(crate) use instance::*;
pub use native::*;
pub use sig::*;
pub use state::*;
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};
pub use tree::*;
pub use value::*;

fn update_or_push<T>(vector: &mut Vec<T>, index: usize, value: T) {
    if let Some(prev) = vector.get_mut(index) {
        *prev = value;
    } else {
        assert_eq!(vector.len(), index);
        vector.insert(index, value);
    }
}
