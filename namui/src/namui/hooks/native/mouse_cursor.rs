use super::*;
use crate::*;

pub(crate) fn with_mouse_cursor<C: Component>(
    component: C,
    cursor: MouseCursor,
) -> WithMouseCursor<C> {
    WithMouseCursor { component, cursor }
}

#[derive(Debug)]
pub struct WithMouseCursor<C: Component> {
    component: C,
    cursor: MouseCursor,
}
impl<C: Component> StaticType for WithMouseCursor<C> {}
impl<C: Component> Component for WithMouseCursor<C> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self { component, cursor } = self;
        ctx.component(component);
        let done = ctx.done();

        if ctx
            .tree_ctx
            .is_cursor_determined
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            return done;
        }

        let xy = system::mouse::position();
        let mouse_is_in = ctx.clip_in(xy)
            && done
                .rendering_tree
                .xy_in(ctx.inverse_matrix().transform_xy(xy));

        if mouse_is_in {
            ctx.tree_ctx
                .is_cursor_determined
                .store(true, std::sync::atomic::Ordering::Relaxed);
            system::mouse::set_mouse_cursor(&cursor);
        }

        return done;
    }
}
