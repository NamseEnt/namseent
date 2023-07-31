use super::*;

pub fn translate<'a>(x: crate::Px, y: crate::Px, component: impl Component + 'a) -> Translate<'a> {
    Translate {
        x,
        y,
        component: Box::new(component),
    }
}

#[derive(Debug)]
pub struct Translate<'a> {
    x: crate::Px,
    y: crate::Px,
    component: Box<dyn Component + 'a>,
}
impl StaticType for Translate<'_> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<Translate>()
    }
}

impl<'a> Component for Translate<'a> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let x = self.x;
        let y = self.y;
        use_render_with_rendering_tree(
            move |ctx| {
                ctx.add(self.component.as_ref());
            },
            move |children| crate::translate(x, y, RenderingTree::Children(children)),
        )
    }
}
