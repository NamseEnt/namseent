mod constants;
mod items;
mod layout;
mod open_button;

use crate::game_state::{mutate_game_state, use_game_state};
use crate::shop::Shop;
use constants::SHOP_WH;
use layout::ShopLayout;
use namui::*;
use open_button::ShopOpenButton;

pub struct ShopModal<'a> {
    pub shop: &'a Shop,
}

impl Component for ShopModal<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { shop } = self;
        let screen_wh = screen::size().into_type::<Px>();

        let (opened, set_opened) = ctx.state(|| true);

        let toggle_open = || {
            set_opened.mutate(|opened| *opened = !*opened);
        };

        let purchase_item = |slot_index: usize| {
            mutate_game_state(move |game_state| {
                game_state.purchase_shop_item(slot_index);
            });
        };

        let game_state = use_game_state(ctx);
        let can_purchase_items: Vec<bool> = shop
            .slots
            .iter()
            .enumerate()
            .map(|(index, _)| game_state.can_purchase_shop_item(index))
            .collect();

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
                shop,
                purchase_item: &purchase_item,
                can_purchase_items: &can_purchase_items,
            });
        });
    }
}
