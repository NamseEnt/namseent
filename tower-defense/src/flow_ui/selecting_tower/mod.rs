mod shop_modal;
mod tower_selecting_hand;

pub use tower_selecting_hand::TowerPreviewContent;

use crate::game_state::flow::SelectingTowerFlow;
use namui::*;
use shop_modal::ShopModal;
use tower_selecting_hand::TowerSelectingHand;

pub struct SelectingTowerUi<'a> {
    pub selecting_tower_flow: &'a SelectingTowerFlow,
}

impl Component for SelectingTowerUi<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            selecting_tower_flow,
        } = self;

        ctx.add(TowerSelectingHand {
            hand: &selecting_tower_flow.hand,
        });

        // ctx.add(ShopModal {
        //     shop: &selecting_tower_flow.shop,
        // });
    }
}
