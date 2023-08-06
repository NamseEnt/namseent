use crate::*;

pub fn translate<'a, C: Component + 'a>(x: crate::Px, y: crate::Px, component: C) -> Translate<C> {
    Translate { x, y, component }
}

#[derive(Debug)]
pub struct Translate<C: Component> {
    x: crate::Px,
    y: crate::Px,
    component: C,
}
impl<C: Component> StaticType for Translate<C> {}

impl<C: Component> Component for Translate<C> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        ctx.matrix
            .lock()
            .unwrap()
            .translate(self.x.as_f32(), self.y.as_f32());

        ctx.component(self.component);
        ctx.done()
    }
}
