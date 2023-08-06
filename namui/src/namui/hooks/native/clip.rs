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
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        todo!()
        // let rendering_tree = ctx.ghost_render(self.component);
        // ctx.return_(crate::clip(
        //     self.path_builder.clone(),
        //     self.clip_op,
        //     rendering_tree,
        // ))
    }
}
