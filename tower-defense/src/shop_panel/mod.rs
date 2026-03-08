mod action_area;
mod constants;
mod items;
mod paper_content;
mod refresh_button;
mod slot_layout_calculator;
mod slot_renderer;
mod slot_rendering_data;
mod sticky_bar;

use crate::game_state::{flow::GameFlow, use_game_state};
use crate::hand::xy_with_spring;
use crate::shop_panel::action_area::ShopActionArea;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use constants::{
    ACTION_HEIGHT, ACTION_MARGIN_Y, BG_HEIGHT, PAPER_HEIGHT, STICKY_HEIGHT, STICKY_VISIBLE_HEIGHT,
    STICKY_WIDTH, TOP_BAR_HEIGHT, action_area_width, shop_panel_wh,
};
use namui::*;

use paper_content::ShopPaperContent;
use sticky_bar::StickyBar;

pub struct ShopPanel;

impl Component for ShopPanel {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let screen_wh = screen::size().into_type::<Px>();
        let in_shop_flow = matches!(game_state.flow, GameFlow::SelectingTower(_));
        let can_open_shop = in_shop_flow;

        let (forced_open, set_forced_open) = ctx.state(|| true);
        let (last_can_open, set_last_can_open) = ctx.state(|| can_open_shop);

        if can_open_shop && !*forced_open && !*last_can_open {
            set_forced_open.set(true);
        }
        if !can_open_shop && *forced_open {
            set_forced_open.set(false);
        }
        if can_open_shop != *last_can_open {
            set_last_can_open.set(can_open_shop);
        }

        let panel_open = can_open_shop && *forced_open;
        let panel_wh = shop_panel_wh();

        // unified internal layout coordinates for panel children
        let paper_y = px(0.0);
        let bg_y = paper_y + (PAPER_HEIGHT - BG_HEIGHT);
        let action_wh = Wh::new(action_area_width(), ACTION_HEIGHT + ACTION_MARGIN_Y);
        let action_xy = Xy::new(
            (panel_wh.width - action_wh.width) / 2.0,
            bg_y + BG_HEIGHT - (action_wh.height * 0.5) + ACTION_MARGIN_Y,
        );
        let sticky_x = (panel_wh.width - STICKY_WIDTH) / 2.0;
        let sticky_y = action_xy.y + action_wh.height - STICKY_VISIBLE_HEIGHT + ACTION_MARGIN_Y;

        let open_xy = Xy::new(
            (screen_wh.width - panel_wh.width) / 2.0,
            (screen_wh.height - panel_wh.height) / 2.0,
        );
        let closed_xy = Xy::new(
            (screen_wh.width - panel_wh.width) / 2.0,
            TOP_BAR_HEIGHT - STICKY_HEIGHT + STICKY_VISIBLE_HEIGHT - sticky_y,
        );
        let target_xy = if panel_open { open_xy } else { closed_xy };
        let animated_xy = xy_with_spring(ctx, target_xy, closed_xy);
        let panel_xy = animated_xy;

        ctx.absolute(panel_xy).compose(|ctx| {
            ctx.translate((0.px(), paper_y)).add(ShopPaperContent {
                wh: Wh::new(panel_wh.width, PAPER_HEIGHT),
            });

            let sticky_center = Wh::new(STICKY_WIDTH, STICKY_HEIGHT).to_xy() * 0.5;
            ctx.translate((sticky_x, sticky_y))
                .translate(sticky_center)
                .rotate((-5.0).deg())
                .translate(-sticky_center)
                .add(StickyBar {
                    wh: Wh::new(STICKY_WIDTH, STICKY_HEIGHT),
                    panel_open,
                    disabled: !can_open_shop,
                    on_toggle: &|| {
                        if !can_open_shop {
                            return;
                        }
                        set_forced_open.set(!*forced_open);
                    },
                });

            ctx.translate(action_xy)
                .add(ShopActionArea { wh: action_wh });

            ctx.translate((0.px(), bg_y)).add(PaperContainerBackground {
                width: panel_wh.width,
                height: BG_HEIGHT,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Paper,
                color: crate::theme::palette::SURFACE_CONTAINER_LOWEST,
                shadow: true,
            });
        });
    }
}
