mod constants;
mod deck_pile_buttons;
mod paper_content;
mod placing_tower_next_fab;
mod reroll_fab;
mod selecting_tower_next_fab;
mod tower_preview;

use crate::{
    card::Card,
    flow_ui::selecting_tower::tower_selecting_hand::get_highest_tower::get_highest_tower_template,
    game_state::{flow::GameFlow, use_game_state},
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
        let selecting_tower = matches!(game_state.flow, GameFlow::SelectingTower(_));
        let placing_tower = matches!(game_state.flow, GameFlow::PlacingTower);
        let hand_flow_active = selecting_tower || placing_tower;

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

        let panel_wh = Wh::new(panel_width(), PAPER_HEIGHT + BOTTOM_OUTSIDE_HEIGHT);
        let panel_x = (screen_wh.width - panel_wh.width) / 2.0;
        let open_xy = Xy::new(
            panel_x,
            screen_wh.height - PAPER_HEIGHT + BOTTOM_OUTSIDE_HEIGHT,
        );
        let closed_xy = Xy::new(panel_x, screen_wh.height + BOTTOM_OUTSIDE_HEIGHT);
        let target_xy = if hand_flow_active { open_xy } else { closed_xy };
        let animated_xy = xy_with_spring(ctx, target_xy, closed_xy);

        let reroll_health_cost = game_state.stage_modifiers.get_reroll_health_cost();
        let reroll_disabled =
            game_state.left_dice == 0 || (game_state.hp - reroll_health_cost as f32) < 1.0;

        ctx.add_with_key(
            "selecting-tower-next-fab",
            SelectingTowerNextFab {
                screen_wh,
                visible: selecting_tower,
                tower_template: tower_template.clone_inner(),
            },
        );
        ctx.add_with_key(
            "placing-tower-next-fab",
            PlacingTowerNextFab {
                screen_wh,
                has_unplaced_towers: !game_state.hand.is_empty(),
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
            },
        );
        ctx.add_with_key(
            "deck-pile-buttons",
            DeckPileButtons {
                screen_wh,
                visible: selecting_tower,
                draw_count: game_state.deck.draw_pile().len(),
                discard_count: game_state.deck.discard_pile().len(),
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
