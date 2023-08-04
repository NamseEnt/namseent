use crate::*;

pub fn on_top<'a>(component: impl Component + 'a) -> OnTop<'a> {
    OnTop {
        component: Box::new(component),
    }
}

#[derive(Debug)]
pub struct OnTop<'a> {
    component: Box<dyn Component + 'a>,
}
impl StaticType for OnTop<'_> {
    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl Component for OnTop<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        ctx.add(self.component.as_ref());
        ctx.done_with_rendering_tree(|children| crate::on_top(RenderingTree::Children(children)))
    }
}
