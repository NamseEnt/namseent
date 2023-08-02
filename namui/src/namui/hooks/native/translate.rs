use crate::*;

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
impl StaticType for Translate<'_> {}

impl Component for Translate<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let x = self.x;
        let y = self.y;
        ctx.use_children_with_rendering_tree(
            |ctx| {
                ctx.add(self.component.as_ref());
                ctx.done()
            },
            |children| crate::translate(x, y, RenderingTree::Children(children)),
        )
    }
}
