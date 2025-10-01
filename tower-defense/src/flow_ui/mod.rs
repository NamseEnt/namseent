mod placing_tower;
mod selecting_tower;
mod selecting_upgrade;

use crate::game_state::{flow::GameFlow, use_game_state};
use namui::*;
pub use selecting_tower::TowerPreviewContent;

pub struct FlowUi;

impl Component for FlowUi {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);

        match &game_state.flow {
            GameFlow::Initializing => {}
            GameFlow::Contract(..) => {}
            GameFlow::SelectingTower(selecting_tower_flow) => {
                ctx.add(selecting_tower::SelectingTowerUi {
                    selecting_tower_flow,
                });
            }
            GameFlow::PlacingTower { hand: _ } => {
                ctx.add(placing_tower::PlacingTowerUi);
            }
            GameFlow::Defense => {}
            GameFlow::SelectingUpgrade { upgrades: _ } => {}
            GameFlow::Result => {}
        };
    }
}
