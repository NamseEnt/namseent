use crate::game_state::use_game_state;
use crate::shop_panel::constants::*;
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use namui::*;
use namui_prebuilt::table;

pub(super) struct ShopActionArea {
    pub wh: Wh<Px>,
}

impl Component for ShopActionArea {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let _game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::padding_no_clip(
                INNER_PADDING + ACTION_MARGIN_Y * 0.5,
                table::horizontal([
                    table::ratio_no_clip(1, |wh, ctx| {
                        ctx.add(super::refresh_button::RefreshButton::new(wh));
                    }),
                ]),
            )(wh, ctx);
        });

        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER_LOW,
            shadow: true,
            arrow: None,
        });
    }
}
