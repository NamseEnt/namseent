use crate::{
    game_state::{mutate_game_state, use_game_state},
    hand::{HandComponent, HandSlotId},
    sound,
};
use namui::*;

use super::constants::{CONTAINER_PADDING, PANEL_PADDING};

pub(super) struct PaperContent;

impl Component for PaperContent {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let hand = game_state.state().presentation_hand_snapshot();

        let on_card_click = |id: HandSlotId| {
            mutate_game_state(move |game_state| {
                if let Some(selected) = game_state.toggle_selecting_tower_card(id) {
                    if selected {
                        sound::play_card_selected_sound();
                    } else {
                        sound::play_card_deselected_sound();
                    }
                } else if game_state.select_placing_tower_slot(id) {
                    sound::play_card_selected_sound();
                }
            });
        };

        ctx.translate((PANEL_PADDING + CONTAINER_PADDING, CONTAINER_PADDING))
            .add(HandComponent {
                hand: &hand,
                on_click: &on_card_click,
            });
    }
}
