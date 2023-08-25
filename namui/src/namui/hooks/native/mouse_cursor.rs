use super::*;
use crate::*;

pub(crate) fn with_mouse_cursor<C: Component>(
    component: C,
    mouse_cursor: MouseCursor,
) -> WithMouseCursor<C> {
    WithMouseCursor {
        component,
        mouse_cursor,
    }
}

pub struct WithMouseCursor<C: Component> {
    component: C,
    mouse_cursor: MouseCursor,
}
impl<C: Component> StaticType for WithMouseCursor<C> {}
impl<C: Component> Debug for WithMouseCursor<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithMouseCursor")
            .field("component", &self.component)
            .finish()
    }
}
impl<C: Component> Component for WithMouseCursor<C> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            component,
            mouse_cursor,
        } = self;
        ctx.component(component);
        let done = ctx.done();
        let rendering_tree =
            RenderingTree::Special(SpecialRenderingNode::MouseCursor(MouseCursorNode {
                rendering_tree: Box::new(done.rendering_tree),
                cursor: Box::new(mouse_cursor),
            }));
        RenderDone { rendering_tree }
    }
}
