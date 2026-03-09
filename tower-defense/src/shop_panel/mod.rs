mod action_area;
mod constants;
mod items;
mod paper_content;
mod refresh_button;
mod slot_layout_calculator;
mod slot_renderer;
mod slot_rendering_data;
mod sticky_bar;
mod voyager;

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
use voyager::Voyager;

/// Top‑level component representing the in‑game shop panel.  Handles all
/// layout and sub‑components (paper, sticky bar, action area).
pub struct ShopPanel;

struct ShopPanelLayout {
    pub panel_wh: Wh<Px>,
    pub paper_y: Px,
    pub bg_y: Px,
    pub action_wh: Wh<Px>,
    pub action_xy: Xy<Px>,
    pub sticky_xy: Xy<Px>,
    pub closed_xy: Xy<Px>,
    pub target_xy: Xy<Px>,
}

impl ShopPanelLayout {
    /// Compute all layout coordinates based on whether the panel may be open and
    /// the current screen size.  This function is `#[inline]` since it runs each
    /// frame during animation.
    #[inline]
    fn compute(can_open: bool, screen_wh: Wh<Px>) -> Self {
        let panel_wh = shop_panel_wh();
        let paper_y = px(0.0);
        let bg_y = paper_y + (PAPER_HEIGHT - BG_HEIGHT);
        let action_wh = Wh::new(action_area_width(), ACTION_HEIGHT + ACTION_MARGIN_Y);
        let action_xy = Xy::new(
            (panel_wh.width - action_wh.width) / 2.0,
            bg_y + BG_HEIGHT - (action_wh.height * 0.5) + ACTION_MARGIN_Y,
        );
        let sticky_x = (panel_wh.width - STICKY_WIDTH) / 2.0;
        let sticky_y = action_xy.y + action_wh.height - STICKY_VISIBLE_HEIGHT + ACTION_MARGIN_Y;
        let sticky_xy = Xy::new(sticky_x, sticky_y);

        let closed_xy = Xy::new(
            (screen_wh.width - panel_wh.width) / 2.0,
            TOP_BAR_HEIGHT - STICKY_HEIGHT + STICKY_VISIBLE_HEIGHT - sticky_y,
        );
        let open_xy = Xy::new(
            (screen_wh.width - panel_wh.width) / 2.0,
            (screen_wh.height - panel_wh.height) / 2.0,
        );
        let target_xy = if can_open { open_xy } else { closed_xy };

        ShopPanelLayout {
            panel_wh,
            paper_y,
            bg_y,
            action_wh,
            action_xy,
            sticky_xy,
            closed_xy,
            target_xy,
        }
    }
}

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
        let layout = ShopPanelLayout::compute(panel_open, screen_wh);
        let animated_xy = xy_with_spring(ctx, layout.target_xy, layout.closed_xy);
        let panel_xy = animated_xy;

        ctx.absolute(panel_xy).compose(|ctx| {
            ctx.translate((0.px(), layout.paper_y))
                .add(ShopPaperContent {
                    wh: Wh::new(layout.panel_wh.width, PAPER_HEIGHT),
                });

            let sticky_center = Wh::new(STICKY_WIDTH, STICKY_HEIGHT).to_xy() * 0.5;
            ctx.translate(layout.sticky_xy)
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

            ctx.translate(layout.action_xy).add(ShopActionArea {
                wh: layout.action_wh,
            });

            ctx.translate((0.px(), layout.bg_y))
                .add(PaperContainerBackground {
                    width: layout.panel_wh.width,
                    height: BG_HEIGHT,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::Paper,
                    color: crate::theme::palette::SURFACE_CONTAINER_LOWEST,
                    shadow: true,
                });

            ctx.add(Voyager);
        });
    }
}
