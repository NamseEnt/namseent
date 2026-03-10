mod action_area;
mod constants;
mod paper_content;
mod sticky_bar;

use crate::{
    game_state::{flow::GameFlow, use_game_state},
    hand::xy_with_spring,
    mutate_game_state,
    theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
};
use namui::*;

use action_area::HandActionArea;
use constants::{
    BOTTOM_OUTSIDE_HEIGHT, PANEL_PADDING, PAPER_HEIGHT, STICKY_HEIGHT, STICKY_SHIFT,
    STICKY_VISIBLE_HEIGHT, STICKY_WIDTH, interaction_width, panel_width,
};
use paper_content::PaperContent;
use sticky_bar::StickyBar;

pub struct HandPanel;

impl Component for HandPanel {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let screen_wh = screen::size().into_type::<Px>();
        let in_hand_flow = matches!(
            game_state.flow,
            GameFlow::SelectingTower(_) | GameFlow::PlacingTower
        );
        let can_open_hand = in_hand_flow;

        // sync forced-open flag stored in game state
        let forced_open = game_state.hand_panel_forced_open;

        let (last_can_open, set_last_can_open) = ctx.state(|| can_open_hand);

        // when availability changes, automatically open/close as before
        if can_open_hand && !forced_open && !*last_can_open {
            mutate_game_state(|gs| {
                gs.hand_panel_forced_open = true;
            });
        }

        if !can_open_hand && forced_open {
            mutate_game_state(|gs| {
                gs.hand_panel_forced_open = false;
            });
        }

        if can_open_hand != *last_can_open {
            set_last_can_open.set(can_open_hand);
        }

        let panel_open = can_open_hand && forced_open;
        let target_offset = if panel_open {
            Xy::zero()
        } else {
            Xy::new(px(0.0), PAPER_HEIGHT - STICKY_VISIBLE_HEIGHT)
        };
        let animated_offset = xy_with_spring(
            ctx,
            target_offset,
            Xy::new(px(0.0), PAPER_HEIGHT - STICKY_VISIBLE_HEIGHT),
        );

        let panel_wh = Wh::new(
            panel_width(),
            STICKY_HEIGHT + PAPER_HEIGHT + BOTTOM_OUTSIDE_HEIGHT,
        );
        let panel_xy = Xy::new(
            (screen_wh.width - panel_wh.width) / 2.0,
            screen_wh.height - (STICKY_HEIGHT + PAPER_HEIGHT)
                + BOTTOM_OUTSIDE_HEIGHT
                + animated_offset.y,
        );

        ctx.absolute(panel_xy).compose(|ctx| {
            let content_x = px(0.0);
            let content_y = px(0.0);
            let content_width = panel_wh.width;
            let paper_y = content_y + STICKY_HEIGHT - STICKY_VISIBLE_HEIGHT;
            let action_x = content_x + content_width - PANEL_PADDING - interaction_width();
            let sticky_x = action_x + interaction_width() - STICKY_WIDTH + STICKY_SHIFT;
            let sticky_center = Wh::new(STICKY_WIDTH, STICKY_HEIGHT).to_xy() * 0.5;

            ctx.translate((sticky_x, 0.px()))
                .translate(sticky_center)
                .rotate(12.5.deg())
                .translate(-sticky_center)
                .add(StickyBar {
                    wh: Wh::new(STICKY_WIDTH, STICKY_HEIGHT),
                    panel_open,
                    disabled: !can_open_hand,
                    on_toggle: &|| {
                        if !can_open_hand {
                            return;
                        }
                        mutate_game_state(|gs| {
                            gs.hand_panel_forced_open = !gs.hand_panel_forced_open;
                        });
                    },
                });

            ctx.translate((0.px(), paper_y)).add(PaperContent {
                wh: Wh::new(content_width, PAPER_HEIGHT),
            });

            ctx.translate((action_x, paper_y)).add(HandActionArea {
                wh: Wh::new(interaction_width(), PAPER_HEIGHT),
            });

            ctx.translate((content_x, paper_y))
                .add(PaperContainerBackground {
                    width: content_width,
                    height: PAPER_HEIGHT,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::Paper,
                    color: crate::theme::palette::SURFACE_CONTAINER_LOWEST,
                    shadow: true,
                    arrow: None,
                });
        });
    }
}
