use super::constants::SHOP_WH;
use super::layout::ShopLayout;
use super::open_button::ShopOpenButton;
use crate::game_state::{mutate_game_state, use_game_state};
use namui::*;

pub struct ShopModal {
    pub screen_wh: Wh<Px>,
}

impl Component for ShopModal {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;
        let game_state = use_game_state(ctx);

        let (opened, set_opened) = ctx.state(|| true);

        let toggle_open = || {
            set_opened.mutate(|opened| *opened = !*opened);
        };
        let shop_slots = &game_state.shop_slots;

        let purchase_item = |slot_index: usize| {
            mutate_game_state(move |game_state| {
                game_state.purchase_shop_item(slot_index);
            });
        };

        let offset = ((screen_wh - SHOP_WH) * 0.5).to_xy();

        ctx.compose(|ctx| {
            ctx.translate(offset).add(ShopOpenButton {
                opened: *opened,
                toggle_open: &toggle_open,
            });
        });

        ctx.compose(|ctx| {
            if !*opened {
                return;
            }
            ctx.translate(offset).add(ShopLayout {
                shop_slots,
                purchase_item: &purchase_item,
            });
        });
    }
}
