use super::*;
use crate::*;

#[derive(Debug)]
pub struct RenderDone {
    pub(crate) rendering_tree: RenderingTree,
}

pub trait Component: StaticType + Debug {
    fn render(self, ctx: &RenderCtx) -> RenderDone;
    fn attach_event<'a>(self, on_event: impl 'a + FnOnce(Event)) -> AttachEvent<'a, Self>
    where
        Self: 'a + Sized,
    {
        native::attach_event(self, on_event)
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self>
    where
        Self: Sized,
    {
        Err(self)
    }
    #[cfg(target_family = "wasm")]
    fn with_mouse_cursor<'a>(self, cursor: MouseCursor) -> WithMouseCursor<Self>
    where
        Self: 'a + Sized,
    {
        native::with_mouse_cursor(self, cursor)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StaticTypeId {
    Option(Option<Box<StaticTypeId>>),
    Single(TypeId),
    Tuple(Vec<StaticTypeId>),
}
pub trait StaticType {
    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl StaticType for RenderingTree {}

impl Component for RenderingTree {
    fn render(self, _ctx: &RenderCtx) -> RenderDone {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(self)
    }
}

impl<T: StaticType> StaticType for Option<T> {}

impl<T: Component> Component for Option<T> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        ctx.compose(|ctx| {
            if let Some(v) = self {
                ctx.add(v);
            }
        })
        .done()
    }
}

impl StaticType for DrawCommand {}
impl Component for DrawCommand {
    fn render(self, _ctx: &RenderCtx) -> RenderDone {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(RenderingTree::Node(self))
    }
}

impl StaticType for PathDrawCommand {}
impl Component for PathDrawCommand {
    fn render(self, _ctx: &RenderCtx) -> RenderDone {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(RenderingTree::Node(DrawCommand::Path {
            command: self.into(),
        }))
    }
}

impl StaticType for ImageDrawCommand {}
impl Component for ImageDrawCommand {
    fn render(self, _ctx: &RenderCtx) -> RenderDone {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(RenderingTree::Node(DrawCommand::Image {
            command: self.into(),
        }))
    }
}

impl StaticType for TextDrawCommand {}
impl Component for TextDrawCommand {
    fn render(self, _ctx: &RenderCtx) -> RenderDone {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(RenderingTree::Node(DrawCommand::Text {
            command: self.into(),
        }))
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
                fn render(self, ctx: &RenderCtx) -> RenderDone {
                    $(ctx.component(self.$i);)*
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
