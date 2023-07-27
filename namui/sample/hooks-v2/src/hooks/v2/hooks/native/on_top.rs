use super::*;

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
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<OnTop>()
    }
}

impl<'a> Component for OnTop<'a> {
    fn render(&self) -> RenderDone {
        use_render_with_rendering_tree(
            move |ctx| {
                ctx.add(self.component.as_ref());
            },
            move |children| namui::on_top(RenderingTree::Children(children)),
        )
    }
}
