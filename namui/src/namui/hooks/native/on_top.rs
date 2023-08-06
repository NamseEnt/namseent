use crate::*;

pub fn on_top<'a>(component: impl Component + 'a) -> Box<dyn Component + 'a> {
    boxed(OnTop {
        component: Box::new(component),
    })
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
        *ctx.matrix.lock().unwrap() = Matrix3x3::identity();

        ctx.return_(self.component.as_ref())
    }
}
