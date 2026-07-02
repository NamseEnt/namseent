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

use crate::game_state::use_game_state;
use crate::hand::xy_with_spring;
use crate::mutate_game_state;
use crate::shop_panel::action_area::ShopActionArea;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};

use constants::{
    ACTION_MARGIN_Y, BG_HEIGHT, PANEL_PADDING, SHOP_PANEL_HEIGHT, STICKY_VISIBLE_HEIGHT,
    STICKY_WIDTH, TOGGLE_HEIGHT, TOP_BAR_HEIGHT, action_area_height, action_area_width,
    shop_panel_wh,
};
use namui::*;

use paper_content::ShopPaperContent;
use sticky_bar::StickyBar;
// use voyager::Voyager;

pub struct ShopPanel;

struct ShopPanelLayout {
    pub panel_wh: Wh<Px>,
    pub bg_y: Px,
    pub action_wh: Wh<Px>,
    pub action_xy: Xy<Px>,
    pub sticky_xy: Xy<Px>,
    pub closed_xy: Xy<Px>,
    pub target_xy: Xy<Px>,
}

impl ShopPanelLayout {
    #[inline]
    fn compute(can_open: bool, panel_open: bool, screen_wh: Wh<Px>) -> Self {
        let panel_wh = shop_panel_wh();
        let bg_y = SHOP_PANEL_HEIGHT - BG_HEIGHT;
        let action_wh = Wh::new(action_area_width(), action_area_height());
        let action_xy = Xy::new(panel_wh.width - action_wh.width, bg_y + action_wh.height);
        let sticky_x = (panel_wh.width - STICKY_WIDTH) / 2.0;
        let sticky_y = SHOP_PANEL_HEIGHT;
        let sticky_xy = Xy::new(sticky_x, sticky_y);

        let center_x = (screen_wh.width - panel_wh.width) / 2.0;
        let closed_xy = if can_open {
            Xy::new(
                center_x,
                TOP_BAR_HEIGHT - TOGGLE_HEIGHT + STICKY_VISIBLE_HEIGHT - sticky_y + ACTION_MARGIN_Y,
            )
        } else {
            // Disabled state should be fully hidden above the top bar.
            // We move the panel higher so the sticky toggle is not visible under top bar.
            Xy::new(center_x, -panel_wh.height - TOGGLE_HEIGHT)
        };
        let open_xy = Xy::new(
            center_x,
            TOP_BAR_HEIGHT + (screen_wh.height - BG_HEIGHT) * 0.5 - panel_wh.height + BG_HEIGHT,
        );
        let target_xy = if panel_open { open_xy } else { closed_xy };

        ShopPanelLayout {
            panel_wh,
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
        let can_open_shop = game_state.can_open_shop_panel();

        // use shared flag instead of local state
        let forced_open = game_state.shop_panel_forced_open;
        let (last_can_open, set_last_can_open) = ctx.state(|| can_open_shop);

        if can_open_shop && !forced_open && !*last_can_open {
            mutate_game_state(|gs| {
                gs.shop_panel_forced_open = true;
            });
        }
        if !can_open_shop && forced_open {
            mutate_game_state(|gs| {
                gs.shop_panel_forced_open = false;
            });
        }
        if can_open_shop != *last_can_open {
            set_last_can_open.set(can_open_shop);
        }

        let panel_open = can_open_shop && forced_open;
        let layout = ShopPanelLayout::compute(can_open_shop, panel_open, screen_wh);
        let animated_xy = xy_with_spring(ctx, layout.target_xy, layout.closed_xy);
        let sticky_wh = Wh::new(STICKY_WIDTH, TOGGLE_HEIGHT);

        ctx.absolute(animated_xy).compose(|ctx| {
            ctx.translate((0.px(), layout.bg_y)).add(ShopPaperContent {
                wh: Wh::new(layout.action_xy.x - PANEL_PADDING, SHOP_PANEL_HEIGHT),
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
                    outline_color: None,
                    shadow: true,
                    arrow: None,
                });

            let sticky_center = sticky_wh.to_xy() * 0.5;
            ctx.translate(layout.sticky_xy)
                .translate(sticky_center)
                .rotate((-5.0).deg())
                .translate(-sticky_center)
                .add(StickyBar {
                    wh: sticky_wh,
                    panel_open,
                    disabled: !can_open_shop,
                    on_toggle: &|| {
                        if !can_open_shop {
                            return;
                        }
                        mutate_game_state(|gs| {
                            gs.shop_panel_forced_open = !gs.shop_panel_forced_open;
                        });
                    },
                });

            // ctx.add(Voyager);
        });
    }
}
