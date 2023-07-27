mod channel;
mod draw;
mod event;
mod hooks;
mod instance;
mod sig;
mod start;
mod tree;
mod value;

pub(crate) use channel::*;
pub use draw::*;
pub use event::*;
pub use hooks::*;
pub(crate) use instance::*;
use namui::RenderingTree;
pub use sig::*;
pub use start::*;
pub use state::*;
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};
use tree::*;
pub use value::*;

#[derive(Debug)]
pub struct RenderDone {
    component_tree: ComponentTree,
}

impl StaticType for RenderingTree {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<RenderingTree>()
    }
}

impl Component for RenderingTree {
    fn render(&self) -> RenderDone {
        use_render_with_rendering_tree(self.clone())
    }
}

pub trait Component: StaticType + Debug {
    fn render(&self) -> RenderDone;
}

impl<T0: StaticType> StaticType for (T0,) {
    fn static_type_id(&self) -> TypeId {
        StaticType::static_type_id(&self.0)
    }
}
impl<T0: Component> Component for (T0,) {
    fn render(&self) -> RenderDone {
        self.0.render()
    }
}

pub trait StaticType {
    fn static_type_id(&self) -> TypeId;
    /// This would be not 'static
    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

fn update_or_push<T>(vector: &mut Vec<T>, index: usize, value: T) {
    if let Some(prev) = vector.get_mut(index) {
        *prev = value;
    } else {
        assert_eq!(vector.len(), index);
        vector.insert(index, value);
    }
}
