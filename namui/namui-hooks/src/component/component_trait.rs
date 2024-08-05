use crate::*;

pub trait Component {
    fn render(self, ctx: &RenderCtx);
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self>
    where
        Self: Sized,
    {
        Err(self)
    }
    // TODO
    // #[cfg(target_family = "wasm")]
    // fn with_mouse_cursor<'a>(self, cursor: MouseCursor) -> WithMouseCursor<Self>
    // where
    //     Self: 'a + Sized,
    // {
    //     native::with_mouse_cursor(self, cursor)
    // }
    fn attach_event<'a>(self, on_event: impl 'a + FnOnce(Event)) -> AttachEvent<'a, Self>
    where
        Self: 'a + Sized,
    {
        attach_event(self, on_event)
    }
}

impl Component for RenderingTree {
    fn render(self, _ctx: &RenderCtx) {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(self)
    }
}

impl<T: Component> Component for Option<T> {
    fn render(self, ctx: &RenderCtx) {
        ctx.compose(|ctx| {
            if let Some(v) = self {
                ctx.add(v);
            }
        });
    }
}

impl Component for DrawCommand {
    fn render(self, _ctx: &RenderCtx) {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(RenderingTree::Node(self))
    }
}

impl Component for PathDrawCommand {
    fn render(self, _ctx: &RenderCtx) {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(RenderingTree::Node(DrawCommand::Path {
            command: self.into(),
        }))
    }
}

impl Component for ImageDrawCommand {
    fn render(self, _ctx: &RenderCtx) {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(RenderingTree::Node(DrawCommand::Image {
            command: self.into(),
        }))
    }
}

impl Component for TextDrawCommand {
    fn render(self, _ctx: &RenderCtx) {
        unreachable!()
    }
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self> {
        Ok(RenderingTree::Node(DrawCommand::Text {
            command: self.into(),
        }))
    }
}

impl<T: FnOnce(&RenderCtx)> Component for T {
    fn render(self, ctx: &RenderCtx) {
        self(ctx)
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
            impl<$($T: Component),*> Component for ($($T,)*) {
                fn render(self, ctx: &RenderCtx) {
                    $(ctx.add(self.$i);)*
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
