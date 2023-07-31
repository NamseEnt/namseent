mod channel;
// mod draw;
mod event;
mod hooks;
mod instance;
// mod native;
mod sig;
mod start;
mod tree;
mod value;

pub(crate) use channel::*;
// pub use draw::*;
pub use event::*;
pub use hooks::*;
pub(crate) use instance::*;
use namui::RenderingTree;
// pub use native::*;
pub use sig::*;
pub use start::*;
// pub use state::*;
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};
pub use tree::*;
pub use value::*;

#[derive(Debug)]
pub struct RenderDone {
    // component_tree: ComponentTree,
    tree_ctx: TreeContext,
}

impl StaticType for RenderingTree {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<RenderingTree>()
    }
}

impl Component for RenderingTree {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let rendering_tree = self.clone();

        ctx.use_children_with_rendering_tree(|ctx| ctx.done(), |_| rendering_tree)
    }
}

pub trait Component: StaticType + Debug {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone;
    // fn attach_event<'a>(
    //     self,
    //     attach_event: impl FnOnce(&mut native::AttachEventBuilder),
    // ) -> AttachEvent<'a>
    // where
    //     Self: 'a + Sized,
    // {
    //     native::attach_event(self, attach_event)
    // }
}

impl StaticType for &dyn Component {
    fn static_type_id(&self) -> TypeId {
        StaticType::static_type_id(&**self)
    }
}
impl Component for &dyn Component {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        (**self).render(ctx)
    }
}

impl<T0: StaticType> StaticType for (T0,) {
    fn static_type_id(&self) -> TypeId {
        StaticType::static_type_id(&self.0)
    }
}
impl<T0: Component> Component for (T0,) {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        self.0.render(ctx)
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
