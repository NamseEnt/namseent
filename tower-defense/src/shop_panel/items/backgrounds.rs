use namui::*;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};

pub struct ShopItemInfoBackground {
    pub wh: Wh<Px>,
    pub color: Color,
}

impl Component for ShopItemInfoBackground {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, color } = self;
        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::PaperSingleLayer,
            color,
            shadow: false,
        });
    }
}
