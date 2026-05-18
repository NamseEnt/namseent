pub(crate) mod tower_selecting_hand;

use namui::*;

pub struct SelectingTowerUi;

impl Component for SelectingTowerUi {
    fn render(self, _ctx: &RenderCtx) {
        // shop UI is handled by ShopPanel, nothing rendered here
    }
}
