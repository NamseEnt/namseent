mod component_trait;
mod ctx;
mod instance;

pub use component_trait::*;
pub use ctx::*;
pub(crate) use instance::*;

pub enum ControlledMemo<T> {
    Changed(T),
    Unchanged(T),
}
