use crate::*;

pub fn translate<'a>(
    x: crate::Px,
    y: crate::Px,
    component: impl Component + 'a,
) -> Box<dyn Component + 'a> {
    boxed(Translate {
        x,
        y,
        component: Box::new(component),
    })
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
        ctx.matrix
            .lock()
            .unwrap()
            .translate(self.x.as_f32(), self.y.as_f32());

        ctx.return_(self.component.as_ref())
    }
}
