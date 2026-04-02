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
    ACTION_HEIGHT, ACTION_MARGIN_Y, BG_HEIGHT, PAPER_HEIGHT, STICKY_HEIGHT, STICKY_VISIBLE_HEIGHT,
    STICKY_WIDTH, TOP_BAR_HEIGHT, action_area_width, shop_panel_wh,
};
use namui::*;

use paper_content::ShopPaperContent;
use sticky_bar::StickyBar;
use voyager::Voyager;

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
    pub show_action_area: bool,
}

impl ShopPanelLayout {
    #[inline]
    fn compute(can_open: bool, panel_open: bool, screen_wh: Wh<Px>) -> Self {
        let panel_wh = shop_panel_wh();
        let paper_y = px(0.0);
        let bg_y = paper_y + (PAPER_HEIGHT - BG_HEIGHT);
        let action_wh = Wh::new(action_area_width(), ACTION_HEIGHT + ACTION_MARGIN_Y);
        let action_xy = Xy::new(
            (panel_wh.width - action_wh.width) / 2.0,
            bg_y + BG_HEIGHT - (action_wh.height * 0.5) + ACTION_MARGIN_Y,
        );
        let show_action_area = true;
        let sticky_x = (panel_wh.width - STICKY_WIDTH) / 2.0;
        let sticky_y = if show_action_area {
            action_xy.y + action_wh.height - STICKY_VISIBLE_HEIGHT + ACTION_MARGIN_Y
        } else {
            PAPER_HEIGHT - STICKY_VISIBLE_HEIGHT
        };
        let sticky_xy = Xy::new(sticky_x, sticky_y);

        let center_x = (screen_wh.width - panel_wh.width) / 2.0;
        let closed_xy = if can_open {
            Xy::new(
                center_x,
                TOP_BAR_HEIGHT - STICKY_HEIGHT + STICKY_VISIBLE_HEIGHT - sticky_y + ACTION_MARGIN_Y,
            )
        } else {
            // Disabled state should be fully hidden above the top bar.
            // We move the panel higher so the sticky toggle is not visible under top bar.
            Xy::new(center_x, -panel_wh.height - STICKY_HEIGHT)
        };
        let open_xy = Xy::new(center_x, (screen_wh.height - panel_wh.height) / 2.0);
        let target_xy = if panel_open { open_xy } else { closed_xy };

        ShopPanelLayout {
            panel_wh,
            paper_y,
            bg_y,
            action_wh,
            action_xy,
            sticky_xy,
            closed_xy,
            target_xy,
            show_action_area,
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

        ctx.absolute(animated_xy).compose(|ctx| {
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
                        mutate_game_state(|gs| {
                            gs.shop_panel_forced_open = !gs.shop_panel_forced_open;
                        });
                    },
                });

            if layout.show_action_area {
                ctx.translate(layout.action_xy).add(ShopActionArea {
                    wh: layout.action_wh,
                });
            }

            ctx.translate((0.px(), layout.bg_y))
                .add(PaperContainerBackground {
                    width: layout.panel_wh.width,
                    height: BG_HEIGHT,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::Paper,
                    color: crate::theme::palette::SURFACE_CONTAINER_LOWEST,
                    shadow: true,
                    arrow: None,
                });

            ctx.add(Voyager);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shop_panel_disabled_closed_is_completely_hidden_above_top_bar() {
        let screen_wh = Wh::new(px(1920.0), px(1080.0));
        let layout = ShopPanelLayout::compute(false, false, screen_wh);

        // panel top is above screen, sticky bar is also fully above top-bar region
        assert!(layout.closed_xy.y < -layout.panel_wh.height + TOP_BAR_HEIGHT);
    }

    #[test]
    fn shop_panel_enabled_closed_shows_sticky_under_top_bar() {
        let screen_wh = Wh::new(px(1920.0), px(1080.0));
        let layout = ShopPanelLayout::compute(true, false, screen_wh);

        assert!(layout.closed_xy.y > -layout.panel_wh.height);
    }

    #[test]
    fn shop_panel_enabled_open_should_center_panel() {
        let screen_wh = Wh::new(px(1920.0), px(1080.0));
        let layout = ShopPanelLayout::compute(true, true, screen_wh);

        let panel_center_y = layout.target_xy.y + (layout.panel_wh.height / 2.0);
        let screen_center_y = screen_wh.height / 2.0;
        assert!((panel_center_y - screen_center_y).abs() < px(0.1));
    }
}
