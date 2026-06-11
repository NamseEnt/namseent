mod action_area;
mod constants;
mod paper_content;
mod sticky_bar;
mod tower_preview;

use crate::{
    card::Card,
    flow_ui::selecting_tower::tower_selecting_hand::get_highest_tower::get_highest_tower_template,
    game_state::{flow::GameFlow, use_game_state},
    hand::xy_with_spring,
    mutate_game_state,
    theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
};
use namui::*;

use action_area::{HandActionArea, HandActionFlow};
use constants::{
    BOTTOM_OUTSIDE_HEIGHT, PANEL_PADDING, PAPER_HEIGHT, PREVIEW_HEIGHT, PREVIEW_RIGHT_OVERLAP,
    PREVIEW_WIDTH, STICKY_HEIGHT, STICKY_HIDDEN_BY_PAPER_HEIGHT, STICKY_SHIFT,
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

        let selected_slot_ids = ctx.track_eq(&game_state.hand.selected_slot_ids());
        let using_cards = ctx.memo(|| {
            let slot_ids = if !selected_slot_ids.is_empty() {
                selected_slot_ids.clone_inner()
            } else {
                game_state.hand.active_slot_ids()
            };

            game_state
                .hand
                .get_items(&slot_ids)
                .filter_map(|item| item.as_card().copied())
                .collect::<Vec<Card>>()
        });
        let tower_template = ctx.memo({
            let upgrade_state = game_state.upgrade_state.clone();
            let rerolled_count = game_state.rerolled_count;
            let config = game_state.config.clone();
            move || {
                if using_cards.is_empty() {
                    None
                } else {
                    Some(get_highest_tower_template(
                        &using_cards,
                        &upgrade_state,
                        rerolled_count,
                        &config,
                    ))
                }
            }
        });

        let panel_open = can_open_hand && forced_open;
        let active_action_flow = HandActionFlow::from_game_flow(&game_state.flow);
        let target_offset = if panel_open {
            Xy::zero()
        } else if can_open_hand {
            Xy::new(px(0.0), PAPER_HEIGHT - STICKY_VISIBLE_HEIGHT)
        } else {
            Xy::new(px(0.0), PAPER_HEIGHT + STICKY_HEIGHT)
        };
        let animated_offset = xy_with_spring(
            ctx,
            target_offset,
            Xy::new(px(0.0), PAPER_HEIGHT + STICKY_HEIGHT),
        );

        let panel_wh = Wh::new(
            panel_width(),
            STICKY_HEIGHT + PAPER_HEIGHT + BOTTOM_OUTSIDE_HEIGHT,
        );
        let content_x = px(0.0);
        let content_y = px(0.0);
        let content_width = panel_wh.width;
        let paper_y = content_y + STICKY_HEIGHT - STICKY_HIDDEN_BY_PAPER_HEIGHT;
        let action_x = content_x + content_width - PANEL_PADDING - interaction_width();
        let action_offscreen_y = paper_y + PAPER_HEIGHT + 8.px();
        let sticky_x = action_x + interaction_width() - STICKY_WIDTH + STICKY_SHIFT;
        let sticky_center = Wh::new(STICKY_WIDTH, STICKY_HEIGHT).to_xy() * 0.5;
        let selecting_target_y = if active_action_flow == Some(HandActionFlow::SelectingTower) {
            paper_y
        } else {
            action_offscreen_y
        };
        let placing_target_y = if active_action_flow == Some(HandActionFlow::PlacingTower) {
            paper_y
        } else {
            action_offscreen_y
        };

        let selecting_action_xy = xy_with_spring(
            ctx,
            Xy::new(action_x, selecting_target_y),
            Xy::new(action_x, action_offscreen_y),
        );
        let placing_action_xy = xy_with_spring(
            ctx,
            Xy::new(action_x, placing_target_y),
            Xy::new(action_x, action_offscreen_y),
        );

        let panel_xy = Xy::new(
            (screen_wh.width - panel_wh.width) / 2.0,
            screen_wh.height - (STICKY_HEIGHT + PAPER_HEIGHT)
                + BOTTOM_OUTSIDE_HEIGHT
                + animated_offset.y,
        );

        ctx.absolute(panel_xy).compose(|ctx| {
            ctx.translate((0.px(), paper_y)).add(PaperContent {
                wh: Wh::new(content_width, PAPER_HEIGHT),
            });

            ctx.translate(selecting_action_xy).add(HandActionArea {
                wh: Wh::new(interaction_width(), PAPER_HEIGHT),
                flow: HandActionFlow::SelectingTower,
                active_flow: active_action_flow,
                tower_template: tower_template.clone_inner(),
            });

            ctx.translate(placing_action_xy).add(HandActionArea {
                wh: Wh::new(interaction_width(), PAPER_HEIGHT),
                flow: HandActionFlow::PlacingTower,
                active_flow: active_action_flow,
                tower_template: tower_template.clone_inner(),
            });

            ctx.translate((content_x, paper_y))
                .add(PaperContainerBackground {
                    width: content_width,
                    height: PAPER_HEIGHT,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::Paper,
                    color: crate::theme::palette::SURFACE_CONTAINER_LOWEST,
                    outline_color: None,
                    shadow: true,
                    arrow: None,
                })
                .attach_event(|event| {
                    let Event::MouseDown { event } = event else {
                        return;
                    };
                    if event.is_local_xy_in() {
                        event.stop_propagation();
                    }
                });

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

            let preview_x = PREVIEW_RIGHT_OVERLAP - PREVIEW_WIDTH;
            let preview_y = paper_y;
            ctx.translate((preview_x, preview_y)).add(
                crate::hand_panel::tower_preview::HandTowerPreview {
                    wh: Wh::new(PREVIEW_WIDTH, PREVIEW_HEIGHT),
                    tower_template: tower_template.clone_inner(),
                    panel_open,
                },
            );
        });
    }
}
