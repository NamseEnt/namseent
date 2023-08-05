use super::*;
use crate::*;

#[derive(Debug)]
pub struct RenderDone {
    pub(crate) rendering_tree: RenderingTree,
}

pub trait Component: StaticType + Debug {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone;
    fn arc<'a>(self) -> Arc<dyn 'a + Component>
    where
        Self: Sized + 'a,
    {
        Arc::new(self)
    }
    fn on_event<'a>(self, on_event: impl 'a + FnOnce(Event)) -> OnEvent<'a>
    where
        Self: 'a + Sized,
    {
        native::on_event(self, on_event)
    }
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
    fn render<'a>(&'a self, _ctx: &'a RenderCtx) -> RenderDone {
        RenderDone {
            rendering_tree: self.clone(),
        }
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

impl<'a> StaticType for Arc<dyn 'a + Component> {
    // fn static_type_id(&self) -> StaticTypeId {
    //     self.as_ref().static_type_id()
    // }
}

impl<'b> Component for Arc<dyn 'b + Component> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        self.as_ref().render(ctx)
    }
}

impl StaticType for Box<dyn Component> {}

impl Component for Box<dyn Component> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        self.as_ref().render(ctx)
    }
}

impl<T: StaticType> StaticType for Option<T> {
    // fn static_type_id(&self) -> StaticTypeId {
    //     self.as_ref().static_type_id()
    // }
}

impl<T: Component> Component for Option<T> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        todo!()
        // if let Some(v) = self {
        //     v.render(ctx)
        // }
    }
}

// pub struct RenderBox<'a> {
//     render: Mutex<Option<Box<dyn 'a + FnOnce(&RenderCtx)>>>,
// }
// impl Debug for RenderBox<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("RenderBox").finish()
//     }
// }
// impl<'a> RenderBox<'a> {
//     pub(crate) fn new(render: impl 'a + FnOnce(&RenderCtx)) -> Self {
//         Self {
//             render: Mutex::new(Some(Box::new(render))),
//         }
//     }
// }

// impl StaticType for RenderBox<'_> {
//     // fn static_type_id(&self) -> StaticTypeId {
//     //     self.as_ref().static_type_id()
//     // }
// }

// impl Component for RenderBox<'_> {
//     fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
//         let mut render = self.render.lock().unwrap();
//         let render = render.take().unwrap();
//         render(ctx)
//     }
// }

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

impl<T: StaticType> StaticType for Vec<(String, T)> {}
impl<T: Component> Component for Vec<(String, T)> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        for (k, v) in self {
            ctx.add(k.to_string(), v);
        }
        ctx.return_internal()
    }
}

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
                    $(ctx.add($i.to_string(), &self.$i);)*
                    ctx.return_internal()
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
