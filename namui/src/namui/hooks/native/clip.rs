use crate::*;

pub fn clip<'a>(
    path: crate::Path,
    clip_op: crate::ClipOp,
    component: impl Component + 'a,
) -> Clip<'a> {
    Clip {
        path,
        clip_op,
        component: Box::new(component),
    }
}

#[derive(Debug)]
pub struct Clip<'a> {
    path: crate::Path,
    clip_op: crate::ClipOp,
    component: Box<dyn Component + 'a>,
}
impl StaticType for Clip<'_> {}

impl Component for Clip<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        todo!()
        // let rendering_tree = ctx.ghost_render(self.component);
        // ctx.return_(crate::clip(
        //     self.path.clone(),
        //     self.clip_op,
        //     rendering_tree,
        // ))
    }
}
