mod constants;
mod deck_pile_buttons;
mod paper_content;
mod placing_tower_next_fab;
mod reroll_fab;
mod selecting_tower_next_fab;
mod tower_preview;

use crate::{
    card::Card,
    game_state::tower_selection::get_highest_tower_template,
    game_state::use_game_state,
    hand::xy_with_spring,
    theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
};
use namui::*;

use constants::{
    BOTTOM_OUTSIDE_HEIGHT, CONTAINER_PADDING, PAPER_HEIGHT, PREVIEW_HEIGHT, PREVIEW_RIGHT_OVERLAP,
    PREVIEW_WIDTH, panel_width,
};
use deck_pile_buttons::DeckPileButtons;
use paper_content::PaperContent;
use placing_tower_next_fab::PlacingTowerNextFab;
use reroll_fab::HandRerollFab;
use selecting_tower_next_fab::SelectingTowerNextFab;

pub struct HandPanel;

impl Component for HandPanel {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let screen_wh = screen::size().into_type::<Px>();
        let raw_core = game_state.raw_core_state();
        let hand = game_state.state().presentation_hand_snapshot();
        let upgrade_state = game_state.state().presentation_upgrade_state_snapshot();
        let selecting_tower = matches!(raw_core.flow(), td_core::GameFlowState::SelectingTower);
        let placing_tower = matches!(raw_core.flow(), td_core::GameFlowState::PlacingTower);
        let hand_flow_active = selecting_tower || placing_tower;

        let selected_slot_ids = ctx.track_eq(&hand.selected_slot_ids());
        let selected_slot_indices = ctx.memo(|| {
            if selected_slot_ids.is_empty() {
                return Vec::new();
            }
            let active_slot_ids = hand.active_slot_ids();
            selected_slot_ids
                .iter()
                .filter_map(|slot_id| active_slot_ids.iter().position(|id| id == slot_id))
                .collect::<Vec<_>>()
        });
        let using_cards = ctx.memo(|| {
            let slot_ids = if !selected_slot_ids.is_empty() {
                selected_slot_ids.clone_inner()
            } else {
                hand.active_slot_ids()
            };

            hand.get_items(&slot_ids)
                .filter_map(|item| item.as_card().copied())
                .collect::<Vec<Card>>()
        });
        let tower_template = ctx.memo({
            let rerolled_count = raw_core.progress().rerolled_count;
            let config = crate::config::GameConfig::from_core_state(raw_core.config().clone())
                .expect("raw game config must be restorable for hand presentation");
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

        let panel_wh = Wh::new(panel_width(), PAPER_HEIGHT + BOTTOM_OUTSIDE_HEIGHT);
        let panel_x = (screen_wh.width - panel_wh.width) / 2.0;
        let open_xy = Xy::new(
            panel_x,
            screen_wh.height - PAPER_HEIGHT + BOTTOM_OUTSIDE_HEIGHT,
        );
        let closed_xy = Xy::new(panel_x, screen_wh.height + BOTTOM_OUTSIDE_HEIGHT);
        let target_xy = if hand_flow_active { open_xy } else { closed_xy };
        let animated_xy = xy_with_spring(ctx, target_xy, closed_xy);

        let reroll_health_cost = raw_core.stage_modifiers().reroll_health_cost;
        let reroll_disabled = raw_core.progress().left_dice == 0
            || crate::Health::from_raw(raw_core.hp_raw())
                .saturating_sub(crate::Health::from_usize(reroll_health_cost))
                < crate::Health::from_integer(1);

        ctx.add_with_key(
            "selecting-tower-next-fab",
            SelectingTowerNextFab {
                screen_wh,
                visible: selecting_tower,
                tower_template_available: tower_template.is_some(),
                selected_slot_indices: selected_slot_indices.clone_inner(),
            },
        );
        ctx.add_with_key(
            "placing-tower-next-fab",
            PlacingTowerNextFab {
                screen_wh,
                has_unplaced_towers: !raw_core.hand().slots.is_empty(),
                visible: placing_tower,
            },
        );
        ctx.add_with_key(
            "hand-reroll-fab",
            HandRerollFab {
                screen_wh,
                visible: selecting_tower,
                disabled: reroll_disabled,
                health_cost: reroll_health_cost,
                selected_slot_indices: selected_slot_indices.clone_inner(),
            },
        );
        ctx.add_with_key(
            "deck-pile-buttons",
            DeckPileButtons {
                screen_wh,
                visible: selecting_tower,
                draw_count: raw_core.deck().draw_pile.len(),
                discard_count: raw_core.deck().discard_pile.len(),
            },
        );

        ctx.absolute(animated_xy).compose(|ctx| {
            ctx.add(PaperContent);

            ctx.add(PaperContainerBackground {
                width: panel_wh.width,
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

            let preview_x = PREVIEW_RIGHT_OVERLAP - PREVIEW_WIDTH;
            let preview_height = (screen_wh.height - open_xy.y - CONTAINER_PADDING)
                .min(PREVIEW_HEIGHT)
                .max(0.px());
            ctx.translate((preview_x, CONTAINER_PADDING)).add(
                crate::hand_panel::tower_preview::HandTowerPreview {
                    wh: Wh::new(PREVIEW_WIDTH, PREVIEW_HEIGHT),
                    visible_wh: Wh::new(PREVIEW_WIDTH, preview_height),
                    tower_template: tower_template.clone_inner(),
                },
            );
        });
    }
}
