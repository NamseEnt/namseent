use super::*;
use crate::*;

#[derive(Debug)]
pub struct RenderDone {
    pub(crate) tree_ctx: TreeContext,
}

pub trait Component: StaticType + Debug {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone;
    fn arc<'a>(self) -> Arc<dyn 'a + Component>
    where
        Self: Sized + 'a,
    {
        Arc::new(self)
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum StaticTypeId {
    Option(Option<Box<StaticTypeId>>),
    Single(TypeId),
    Tuple(Vec<StaticTypeId>),
}
pub trait StaticType {
    // fn static_type_id(&self) -> StaticTypeId;
    /// This would be not 'static
    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<T: Component> StaticType for &T {
    // fn static_type_id(&self) -> StaticTypeId {
    //     (*self).static_type_id()
    // }
}

impl<T: Component> Component for &T {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        (*self).render(ctx)
    }
}

impl StaticType for RenderingTree {
    // fn static_type_id(&self) -> StaticTypeId {
    //     StaticTypeId::Single(TypeId::of::<RenderingTree>())
    // }
}

impl Component for RenderingTree {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        ctx.done_with_rendering_tree(|_| self.clone())
    }
}

impl StaticType for &dyn Component {
    // fn static_type_id(&self) -> StaticTypeId {
    //     (*self).static_type_id()
    // }
}

impl Component for &dyn Component {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        (*self).render(ctx)
    }
}

impl StaticType for Arc<dyn Component> {
    // fn static_type_id(&self) -> StaticTypeId {
    //     self.as_ref().static_type_id()
    // }
}

impl Component for Arc<dyn Component> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        self.as_ref().render(ctx)
    }
}

// TODO

// impl<T: StaticType> StaticType for Option<T> {
//     fn static_type_id(&self) -> StaticTypeId {
//         StaticTypeId::Option(self.as_ref().map(|v| Box::new(v.static_type_id())))
//     }
// }
// impl<T: Component> Component for Option<T> {
//     fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
//         if let Some(v) = self {
//             v.render(ctx)
//         } else {
//             use_render_nothing()
//         }
//     }
// }

macro_rules! component_impl {
    (
        $(
            ($
                ($T:ident, $i:tt),
            *),
        )*
    ) => {
        $(
            impl<$($T: StaticType),*> StaticType for ($($T,)*) {
                fn static_type_name(&self) -> &'static str {
                    std::any::type_name::<Self>()
                }
            }
            impl<$($T: Component),*> Component for ($($T,)*) {
                fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
                    $(ctx.add(&self.$i as &dyn Component);)*
                    ctx.done()
                }
            }
        )*
    };
}

component_impl!(
    (T0, 0),
    (T0, 0, T1, 1),
    (T0, 0, T1, 1, T2, 2),
    (T0, 0, T1, 1, T2, 2, T3, 3),
    (T0, 0, T1, 1, T2, 2, T3, 3, T4, 4),
    (T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5),
    (T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6),
    (T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7),
);
