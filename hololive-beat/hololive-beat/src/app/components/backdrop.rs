use crate::app::theme::THEME;
use namui::prelude::*;

#[component]
pub struct Backdrop {
    pub wh: Wh<Px>,
}
impl Component for Backdrop {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh } = self;

        ctx.component(path(
            Path::new().add_rect(Rect::zero_wh(wh)),
            Paint::new(THEME.primary.darker.with_alpha(242)),
        ));

        
    }
}
