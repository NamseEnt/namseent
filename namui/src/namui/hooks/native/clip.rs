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
    fn render<'a>(&'a self, ctx: &'a RenderCtx) {
        let &Self {
            ref path_builder,
            clip_op,
            ref component,
        } = self;
        let path_builder = path_builder.clone();
        ctx.add(component.as_ref());
        ctx.done_with_rendering_tree(move |children| {
            crate::clip(
                path_builder.clone(),
                clip_op,
                RenderingTree::Children(children),
            )
        })
    }
}
