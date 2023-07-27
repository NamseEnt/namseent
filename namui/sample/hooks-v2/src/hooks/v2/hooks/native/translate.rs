use super::*;

pub(crate) fn translate<'a>(
    x: namui::Px,
    y: namui::Px,
    component: impl Component + 'a,
) -> impl Component + 'a {
    Translate {
        x,
        y,
        component: Box::new(component),
    }
}

#[derive(Debug)]
struct Translate<'a> {
    x: namui::Px,
    y: namui::Px,
    component: Box<dyn Component + 'a>,
}
impl StaticType for Translate<'_> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<Translate>()
    }
}

impl<'a> Component for Translate<'a> {
    fn render(&self) -> RenderDone {
        let x = self.x;
        let y = self.y;
        use_render_with_rendering_tree(
            move |ctx| {
                ctx.add(self.component.as_ref());
            },
            move |children| namui::translate(x, y, RenderingTree::Children(children)),
        )
    }
}
