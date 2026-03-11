use crate::{
    game_state::{flow::GameFlow, mutate_game_state, use_game_state},
    hand::{HandComponent, HandSlotId},
    sound,
};
use namui::*;
use namui_prebuilt::table;

use super::constants::{PANEL_PADDING, interaction_width};

pub(super) struct PaperContent {
    pub wh: Wh<Px>,
}

impl Component for PaperContent {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);

        let on_card_click = |id: HandSlotId| {
            mutate_game_state(move |game_state| match &mut game_state.flow {
                GameFlow::SelectingTower(_) => {
                    let Some(item) = game_state.hand.get_item(id) else {
                        return;
                    };
                    if item.as_card().is_none() {
                        return;
                    }
                    if game_state.hand.selected_slot_ids().contains(&id) {
                        game_state.hand.deselect_slot(id);
                        sound::play_card_deselected_sound();
                    } else {
                        game_state.hand.select_slot(id);
                        sound::play_card_selected_sound();
                    }
                }
                GameFlow::PlacingTower => {
                    let Some(item) = game_state.hand.get_item(id) else {
                        return;
                    };
                    if item.as_tower().is_none() {
                        return;
                    }
                    if !game_state.hand.selected_slot_ids().is_empty() {
                        return;
                    }
                    game_state.hand.select_slot(id);
                    sound::play_card_selected_sound();
                }
                _ => {}
            });
        };

        ctx.compose(|ctx| {
            table::padding_no_clip(
                PANEL_PADDING,
                table::horizontal([
                    table::fixed_no_clip(
                        wh.width - interaction_width() - PANEL_PADDING,
                        |_, ctx| {
                            ctx.add(HandComponent {
                                hand: &game_state.hand,
                                on_click: &on_card_click,
                            });
                        },
                    ),
                    table::fixed(interaction_width(), |_, _| {}),
                ]),
            )(wh, ctx);
        });
    }
}
