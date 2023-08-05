use crate::*;

pub fn clip<'a>(
    path_builder: crate::PathBuilder,
    clip_op: crate::ClipOp,
    component: impl Component + 'a,
) -> Clip<'a> {
    Clip {
        path_builder,
        clip_op,
        component: Box::new(component),
    }
}

#[derive(Debug)]
pub struct Clip<'a> {
    path_builder: crate::PathBuilder,
    clip_op: crate::ClipOp,
    component: Box<dyn Component + 'a>,
}
impl StaticType for Clip<'_> {}

impl Component for Clip<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            ref path_builder,
            clip_op,
            ref component,
        } = self;
        let rendering_tree = ctx.ghost_render(component.as_ref());
        ctx.return_(crate::clip(path_builder.clone(), clip_op, rendering_tree))
    }
}
