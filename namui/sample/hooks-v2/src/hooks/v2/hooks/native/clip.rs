use super::*;

pub fn clip<'a>(
    path_builder: namui::PathBuilder,
    clip_op: namui::ClipOp,
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
    path_builder: namui::PathBuilder,
    clip_op: namui::ClipOp,
    component: Box<dyn Component + 'a>,
}
impl StaticType for Clip<'_> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<Clip>()
    }
}

impl<'a> Component for Clip<'a> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let &Self {
            ref path_builder,
            clip_op,
            ref component,
        } = self;
        let path_builder = path_builder.clone();

        use_render_with_rendering_tree(
            move |ctx| {
                ctx.add(component.as_ref());
            },
            move |children| {
                namui::clip(
                    path_builder.clone(),
                    clip_op,
                    RenderingTree::Children(children),
                )
            },
        )
    }
}
