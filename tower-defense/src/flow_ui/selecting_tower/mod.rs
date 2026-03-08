mod shop_modal;
pub(crate) mod tower_selecting_hand;

pub use tower_selecting_hand::TowerPreviewContent;

use crate::game_state::flow::SelectingTowerFlow;
use namui::*;
use shop_modal::ShopModal;

pub struct SelectingTowerUi<'a> {
    pub selecting_tower_flow: &'a SelectingTowerFlow,
}

impl Component for SelectingTowerUi<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            selecting_tower_flow,
        } = self;

        ctx.add(ShopModal {
            shop: &selecting_tower_flow.shop,
        });
    }
}
