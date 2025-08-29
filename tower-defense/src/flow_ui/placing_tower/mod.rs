mod tower_placing_hand;

use namui::*;
use tower_placing_hand::TowerPlacingHand;

pub struct PlacingTowerUi;

impl Component for PlacingTowerUi {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(TowerPlacingHand);
    }
}
