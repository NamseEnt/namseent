mod result;
pub(crate) mod selecting_tower;

use crate::game_state::use_game_state;
use namui::*;

pub struct FlowUi;

impl Component for FlowUi {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);

        match game_state.raw_core_state().flow() {
            td_core::GameFlowState::Initializing => {}
            td_core::GameFlowState::Shopping(_) => {}
            td_core::GameFlowState::SelectingTower => {
                ctx.add(selecting_tower::SelectingTowerUi);
            }
            td_core::GameFlowState::PlacingTower => {}
            td_core::GameFlowState::Defense(_) => {}
            td_core::GameFlowState::TreasureSelection { .. } => {}
            td_core::GameFlowState::Result { .. } => {
                ctx.add(result::ResultModal);
            }
        };
    }
}
