mod result;
pub(crate) mod selecting_tower;

use crate::game_state::{flow::GameFlow, use_game_state};
use namui::*;
pub use selecting_tower::TowerPreviewContent;

pub struct FlowUi;

impl Component for FlowUi {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);

        match &game_state.flow {
            GameFlow::Initializing => {}
            GameFlow::SelectingTower(_) | GameFlow::SelectingTreasure(_) => {
                ctx.add(selecting_tower::SelectingTowerUi);
            }
            GameFlow::PlacingTower => {}
            GameFlow::Defense(..) => {}
            GameFlow::Result { .. } => {
                ctx.add(result::ResultModal);
            }
        };
    }
}
