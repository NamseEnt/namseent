use crate::app::theme::THEME;
use namui::*;

#[component]
pub struct DarkFrame {
    pub wh: Wh<Px>,
}
impl Component for DarkFrame {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        ctx.add(path(
            Path::new().add_rect(Rect::zero_wh(wh)),
            Paint::new(THEME.primary.dark).set_blend_mode(BlendMode::Multiply),
        ));
    }
}

#[component]
pub struct LightFrame {
    pub wh: Wh<Px>,
}
impl Component for LightFrame {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        ctx.add(path(
            Path::new().add_rect(Rect::zero_wh(wh)),
            Paint::new(THEME.primary.main.with_alpha(25)).set_blend_mode(BlendMode::Screen),
        ));
    }
}
