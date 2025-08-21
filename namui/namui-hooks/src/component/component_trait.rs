use crate::*;

pub trait Component {
    fn render(self, ctx: &RenderCtx);
    fn direct_rendering_tree(self) -> Result<RenderingTree, Self>
    where
        Self: Sized,
    {
        Err(self)
    }
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
        if let Some(v) = self {
            ctx.add(v);
        }
    }
}

impl<'a, T> Component for &'a Option<T>
where
    &'a T: Component,
{
    fn render(self, ctx: &RenderCtx) {
        if let Some(v) = self {
            ctx.add(v);
        }
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

impl<T: FnOnce(&RenderCtx, S), S> Component for (T, S) {
    fn render(self, ctx: &RenderCtx) {
        self.0(ctx, self.1)
    }
}
